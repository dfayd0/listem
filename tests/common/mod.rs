use axum::{
    body::Body,
    http::{
        header::SET_COOKIE,
        Request,
        StatusCode,
    },
};
use diesel_migrations::MigrationHarness;
use diesel::{
    connection::SimpleConnection,
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};
use diesel_migrations::{
    embed_migrations,
    EmbeddedMigrations,
};
use listem::{
    build_router,
    db,
    AppState,
};
use tempfile::TempDir;
use tower::ServiceExt;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct TestApp
{
    pub router: axum::Router,
    pub pool:   Pool<ConnectionManager<SqliteConnection>>,
    pub _dir:       TempDir,
}

impl TestApp
{
    /// Fresh app backed by its own temporary sqlite database.
    pub fn new() -> Self
    {
        let dir = tempfile::tempdir().expect("create tempdir");
        let db_path = dir.path().join("listem_test.db");
        let database_url = db_path.to_string_lossy().to_string();
        eprintln!("test db url: {database_url}");

        let mut conn = SqliteConnection::establish(&database_url)
            .expect("connect to test db");
        // sqlite quirks the app relies on
        conn.batch_execute(
            "PRAGMA foreign_keys = ON; PRAGMA busy_timeout = 5000;",
        )
        .expect("set pragmas");
        conn.run_pending_migrations(MIGRATIONS)
            .expect("run migrations");
        drop(conn);

        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("create pool");

        let state = AppState::new("Listem", pool.clone());

        Self {
            router: build_router(state),
            pool,
            _dir: dir,
        }
    }

    pub async fn request(&mut self, req: Request<Body>) -> (StatusCode, Vec<u8>)
    {
        let res = self.router.clone().oneshot(req).await.expect("oneshot");
        let status = res.status();
        let body = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .expect("read body")
            .to_vec();

        (status, body)
    }

    pub fn get(&self, uri: &str) -> Request<Body>
    {
        Request::builder().uri(uri).body(Body::empty()).unwrap()
    }

    /// GET request capturing any csrf cookie issued in the response.
    pub async fn get_with_csrf(&mut self, uri: &str) -> (StatusCode, Vec<u8>, String)
    {
        let res = self.router.clone().oneshot(self.get(uri)).await.expect("oneshot");
        let status = res.status();
        let token = extract_csrf_cookie(res.headers().get_all(SET_COOKIE).iter())
            .expect("csrf cookie should be set on first GET");
        let body = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .expect("read body")
            .to_vec();

        (status, body, token)
    }

    /// Unsafe method request with valid csrf cookie and header.
    pub fn unsafe_request(&self, method: axum::http::Method, uri: &str, token: &str, body: Body) -> Request<Body>
    {
        Request::builder()
            .method(method)
            .uri(uri)
            .header(axum::http::header::COOKIE, format!("{}={token}", listem::csrf::CSRF_COOKIE))
            .header(listem::csrf::CSRF_HEADER, token)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(body)
            .unwrap()
    }
}

pub fn extract_csrf_cookie<'a>(headers: impl Iterator<Item = &'a axum::http::HeaderValue>) -> Option<String>
{
    headers.filter_map(|value| {
        let value = value.to_str().ok()?;
        let (name, token) = value.split(';').next()?.split_once('=')?;

        if name.trim() == listem::csrf::CSRF_COOKIE
        {
            Some(token.trim().to_owned())
        }
        else
        {
            None
        }
    })
    .next()
}

/// Form encode a NewTodo/TodoForm subset.
pub fn form(fields: &[(&str, &str)]) -> Body
{
    let encoded = fields
        .iter()
        .map(|(k, v)| format!("{k}={}", urlencoded(v)))
        .collect::<Vec<_>>()
        .join("&");

    Body::from(encoded)
}

fn urlencoded(v: &str) -> String
{
    v.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

/// Insert a todo directly through the db layer.
pub fn seed_todo(pool: &Pool<ConnectionManager<SqliteConnection>>, title: &str) -> listem::models::Todo
{
    let mut conn = pool.get().expect("pool connection");

    match db::create_todo(
        &mut conn,
        listem::models::NewTodo {
            title:       title.to_owned(),
            description: String::new(),
            importance:  "low".to_owned(),
        },
    ) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("seed failed: {e:?}");
            panic!("seed todo: {e}")
        }
    }
}
