mod routes;

use axum::{
    response::Redirect,
    routing::get,
    Router,
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
    // db_pool:
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

    let state = AppState {
        app_name: "My App".to_string(),
    };

    info!("Initializing server...");
    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/home") }))
        .route("/home", get(routes::home))
        .with_state(state)
        .nest_service("/static", ServeDir::new("static"));

    let addr = "0.0.0.0";
    let port = 4444;

    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind(format!("{addr}:{port}").as_str())
            .await
            .unwrap();

    info!("Server listening on http://{addr}:{port}");
    axum::serve(listener, app).await.unwrap();
}
