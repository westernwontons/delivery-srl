use axum::{
    routing::{get, post},
    Router
};
use delivery_backend::routes;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

/// Run the app
///
/// Accepts a `Router` which is converted into a service
/// which is going to become the one router to rule them all.
async fn run_app(app: Router) -> anyhow::Result<()> {
    axum::Server::bind(&"0.0.0.0:3000".parse::<SocketAddr>().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn setup_app() -> Router {
    let middleware_stack =
        ServiceBuilder::new().layer(TraceLayer::new_for_http());

    Router::<()>::new()
        .route("/search", post(routes::search))
        .route("/history", get(routes::history))
        .nest("/client", routes::client_router())
        .route_layer(middleware_stack)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = setup_app();

    run_app(app).await?;

    Ok(())
}
