// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use tower_http::trace::TraceLayer;
use tracing::{Level, Span};
use tracing_subscriber::{
    filter::EnvFilter, fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

pub fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("ontology_api_server=info,rmcp=info,tower_http=debug,axum::rejection=trace")
    });

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer())
        .init();

    Ok(())
}

pub fn trace_layer() -> TraceLayer<tower_http::classify::ServerErrorsFailureClass, fn(&Span)> {
    tower_http::trace::TraceLayer::new_for_http().make_span_with(|request: &hyper::Request<_>| {
        let path = request.uri().path();
        tracing::span!(
            Level::INFO,
            "request",
            method = %request.method(),
            path,
            version = ?request.version()
        )
    })
}
