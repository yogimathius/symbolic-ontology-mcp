use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use std::time::Duration;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::{self, TraceLayer};
use tracing::{Level, info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::Config;

/// Initialize the tracing subscriber based on the provided configuration
pub fn init_tracing(config: &Config) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.get_tracing_filter().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Creates a trace layer for HTTP request and response logging.
///
/// This function returns a TraceLayer configured with custom behavior for logging
/// HTTP requests and responses, with specific handling for different status codes.
///
/// # Returns
///
/// A configured `TraceLayer` middleware that can be added to an Axum router.
///
/// # Example
///
/// ```
/// use axum::Router;
/// use crate::logging::trace_layer;
///
/// let app = Router::new()
///     .route("/", get(|| async { "Hello, World!" }))
///     .layer(trace_layer());
/// ```
pub fn trace_layer() -> TraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    trace::DefaultMakeSpan,
    impl Fn(&Request<Body>, &tracing::Span) + Copy,
    impl Fn(&axum::http::Response<Body>, Duration, &tracing::Span) + Copy,
> {
    tower_http::trace::TraceLayer::new_for_http()
        .make_span_with(tower_http::trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(|request: &Request<_>, _span: &tracing::Span| {
            info!(
                "Request: {} {} {:?}",
                request.method(),
                request.uri(),
                request.version()
            );
        })
        .on_response(
            |response: &axum::http::Response<Body>, latency: Duration, _span: &tracing::Span| {
                let status = response.status();
                let latency = format!("{:.3} ms", latency.as_secs_f64() * 1000.0);

                if status.is_success() {
                    info!(
                        "Response: {} in {} - {}",
                        status.as_u16(),
                        latency,
                        status.canonical_reason().unwrap_or("Unknown")
                    );
                } else if status.is_server_error() {
                    warn!(
                        "Server error: {} in {} - {}",
                        status.as_u16(),
                        latency,
                        status.canonical_reason().unwrap_or("Unknown")
                    );
                } else {
                    info!(
                        "Non-success response: {} in {} - {}",
                        status.as_u16(),
                        latency,
                        status.canonical_reason().unwrap_or("Unknown")
                    );
                }
            },
        )
}
