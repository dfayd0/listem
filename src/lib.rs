pub mod csrf;
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

#[derive(Clone)]
pub struct AppState
{
    app_name: String,
    db_pool:  Pool<ConnectionManager<SqliteConnection>>,
}

impl AppState
{
    pub fn new(app_name: impl Into<String>, db_pool: Pool<ConnectionManager<SqliteConnection>>) -> Self
    {
        Self {
            app_name: app_name.into(),
            db_pool,
        }
    }

    pub fn app_name(&self) -> &str
    {
        &self.app_name
    }
}

pub fn build_router(state: AppState) -> Router
{
    Router::new()
        .route("/", get(routes::index))
        .route("/home", get(routes::home))
        .route("/todolist", get(routes::todolist))
        .route("/add_todo", post(routes::add_todo))
        .route("/about", get(routes::about))
        .route("/delete_todo/{id}", delete(routes::delete_todo))
        .route("/toggle_todo/{id}", put(routes::toggle_todo))
        .route("/edit_form/{id}", get(routes::edit_todo_form))
        .route("/edit/{id}", post(routes::edit_todo))
        .fallback(routes::not_found)
        .nest_service("/static", ServeDir::new("static"))
        .layer(axum::middleware::from_fn(csrf::csrf_protect))
        .with_state(state)
}
