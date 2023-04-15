use axum::http::header::AUTHORIZATION;
use axum::http::HeaderName;
use axum::routing::{get, post};
use axum::Router;
use delivery_backend::error::AppError;
use delivery_backend::routers;
use delivery_backend::state::{setup_app_state, AppState};
use std::env;
use std::net::{IpAddr, SocketAddr};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer};
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use tower_http::trace::TraceLayer;

/// Run the app
///
/// Accepts a `Router` which is converted into a service
/// which is going to become the one router to rule them all.
#[tracing::instrument(skip(app))]
async fn run_app(app: Router) -> anyhow::Result<()> {
    let host = match env::var("AXUM_HOST") {
        Ok(v) => v.parse::<IpAddr>()?,
        Err(_) => {
            tracing::info!("AXUM_HOST not specified, using default");
            "0.0.0.0".parse::<IpAddr>().unwrap()
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
    axum::Server::bind(&SocketAddr::new(host, port)).serve(app.into_make_service()).await?;

    Ok(())
}

/// App setup
///
/// Initializes the root router, middlewares and the app state
#[tracing::instrument]
async fn setup_app() -> Result<Router, AppError> {
    let x_request_id_header = HeaderName::from_static("x-request-id");

    let middleware_stack = ServiceBuilder::new()
        .layer(SetSensitiveRequestHeadersLayer::new(std::iter::once(AUTHORIZATION)))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(SetRequestIdLayer::new(x_request_id_header.clone(), MakeRequestUuid))
        .layer(PropagateHeaderLayer::new(x_request_id_header));

    let app_state = setup_app_state().await?;
    tracing::info!("Application setup ok");

    let router = Router::<AppState>::new()
        .route("/search", post(routers::customer_search))
        .route("/history", get(routers::customer_history))
        .nest("/customer", routers::customer_router())
        .nest("/auth", routers::auth_router())
        .route_layer(middleware_stack)
        .with_state(app_state);

    Ok(router)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = setup_app().await?;

    run_app(app).await?;

    Ok(())
}
