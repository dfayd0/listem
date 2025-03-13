pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

use axum::{
    routing::{
        delete,
        get,
        post,
        put,
    },
    Router,
};
use diesel::{
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[derive(Clone)]
pub struct AppState
{
    app_name: String,
    db_pool:  Pool<ConnectionManager<SqliteConnection>>,
}

#[tokio::main]
async fn main()
{
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "listem=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // the state is done at init, and wil not be duplicated by different
    // connections to the server
    let state = AppState {
        app_name: "Listem".to_owned(),
        db_pool:  db::create_db_pool(),
    };

    info!("Initializing server...");
    let app = Router::new()
        .route("/", get(routes::index))
        .route("/home", get(routes::home))
        .route("/todolist", get(routes::todolist))
        .route("/add_todo", post(routes::add_todo))
        .route("/about", get(routes::about))
        .route("/delete_todo/{id}", delete(routes::delete_todo))
        .route("/toggle_todo/{id}", put(routes::toggle_todo))
        .with_state(state)
        .nest_service("/static", ServeDir::new("static"));

    let addr = "0.0.0.0";
    let port = 4444;

    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind(format!("{addr}:{port}").as_str())
            .await
            .expect("Failed to bind to port");

    info!("Server listening on http://{addr}:{port}");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
