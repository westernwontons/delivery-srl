use axum::routing::{get, post};
use axum::Router;
use delivery_backend::error::AppError;
use delivery_backend::routes;
use delivery_backend::state::{setup_app_state, AppState};
use std::env;
use std::net::{IpAddr, SocketAddr};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

/// Run the app
///
/// Accepts a `Router` which is converted into a service
/// which is going to become the one router to rule them all.
#[tracing::instrument(skip(app))]
async fn run_app(app: Router) -> anyhow::Result<()> {
    let host = match env::var("AXUM_HOST") {
        Ok(v) => v,
        Err(_) => {
            tracing::info!("AXUM_HOST not specified, using default");
            "0.0.0.0".into()
        }
    };

    let port = match env::var("AXUM_PORT") {
        Ok(v) => v.parse::<u16>()?,
        Err(_) => {
            tracing::info!("AXUM_PORT not specified, using default");
            3000
        }
    };

    tracing::info!("Binding app to 0.0.0.0:3000");
    axum::Server::bind(&SocketAddr::new(host.parse::<IpAddr>()?, port))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// App setup
///
/// Initializes the root router, middlewares and the app state
#[tracing::instrument]
async fn setup_app() -> Result<Router, AppError> {
    let middleware_stack =
        ServiceBuilder::new().layer(TraceLayer::new_for_http());
    let app_state = setup_app_state().await?;
    tracing::info!("Application setup ok");

    Ok(Router::<AppState>::new()
        .route("/search", post(routes::customer_search))
        .route("/history", get(routes::customer_history))
        .nest("/customer", routes::customer_router())
        .route_layer(middleware_stack)
        .with_state(app_state))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = setup_app().await?;

    run_app(app).await?;

    Ok(())
}
