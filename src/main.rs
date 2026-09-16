use listem::{
    build_router,
    db,
    AppState,
};
use tracing::info;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

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
    let state = AppState::new("Listem", db::create_db_pool());

    info!("Initializing server...");
    let app = build_router(state);

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
