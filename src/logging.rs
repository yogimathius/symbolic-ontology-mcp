use axum::body::Body;
use axum::http::Request;
use std::fmt::Arguments;
use std::time::Duration;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::{self, TraceLayer};
use tracing::{Level, info, warn};

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
/// use axum::routing::get;
/// use symbol_ontology_mcp::logging::trace_layer;
///
/// let app = Router::<()>::new()
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

pub fn setup_logging() -> Result<(), fern::InitError> {
    let format_timestamp = |sys_time: std::time::SystemTime| -> String {
        let duration = sys_time
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();

        let secs = duration.as_secs();
        let millis = duration.subsec_millis();

        let hours = (secs / 3600) % 24;
        let minutes = (secs / 60) % 60;
        let seconds = secs % 60;

        format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
    };

    let formatter = move |out: fern::FormatCallback, message: &Arguments, record: &log::Record| {
        let msg = format!("{}", message);
        let formatted_msg = if msg.contains("{") && msg.contains("}") && msg.contains(":") {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&msg) {
                if let Ok(pretty) = serde_json::to_string_pretty(&json) {
                    pretty
                } else {
                    msg
                }
            } else {
                msg
            }
        } else {
            msg
        };

        out.finish(format_args!(
            "{} {:5} {}: {}",
            format_timestamp(std::time::SystemTime::now()),
            record.level(),
            record
                .target()
                .split("::")
                .last()
                .unwrap_or(record.target()),
            formatted_msg
        ))
    };

    let base_config = fern::Dispatch::new()
        .format(formatter)
        .level(log::LevelFilter::Info)
        .level_for("rmcp::transport::sse_server", log::LevelFilter::Info)
        .level_for("rmcp::service", log::LevelFilter::Info)
        .level_for("sqlx", log::LevelFilter::Warn);

    let stderr_config = fern::Dispatch::new().chain(std::io::stderr());

    let stdout_config = fern::Dispatch::new().chain(std::io::stdout());

    base_config
        .chain(stderr_config)
        .chain(stdout_config)
        .apply()?;

    Ok(())
}
