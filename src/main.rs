mod routes;

use axum::response::Redirect;
use axum::routing::{get, post};

use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main()
{
    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/home") }))
        .route("/home", get(routes::home))
        .nest_service("/static", ServeDir::new("static"));

    let addr = "0.0.0.0";
    let port = 4444;
    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind(format!("{addr}:{port}").as_str())
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}
