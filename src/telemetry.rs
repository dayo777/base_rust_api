// // src/telemetry.rs
// use opentelemetry::sdk::{trace, Resource};
// use opentelemetry::KeyValue;
// use opentelemetry_otlp::WithExportConfig;
// use std::io;
// use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
// use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
// // use tracing_subscriber::fmt::Layer;
// use tracing::subscriber::set_global_default;
// use opentelemetry_sdk;
//
// /// initialize OpenTelemetry with JSON formatting for logs, metrics & traces
// pub fn init_telemetry(service_name: &str, otlp_endpoint: Option<&str>) -> io::Result<()> {
//     let app_name = service_name.to_string();
//
//     // define common resource attributes
//     let resource = Resource::new(vec![
//         KeyValue::new("service.name", app_name.clone()),
//         KeyValue::new("service.version", env!("CARGO_PKG_VERSION").to_string()),
//         KeyValue::new("deployment.environment", std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string())),
//     ]);
//
//     // configure OpenTelemetry trace exporter
//     let tracer = if let Some(endpoint) = otlp_endpoint {
//         // OTLP exporter for sending to collector
//         let otlp_exporter = opentelemetry_otlp::new_exporter()
//             .tonic()
//             .with_endpoint(endpoint);
//
//         opentelemetry_otlp::new_pipeline()
//             .tracing()
//             .with_exporter(otlp_exporter)
//             .with_trace_config(trace::config().with_resource(resource.clone()))
//             .install_simple()
//             .expect("Failed to install OpenTelemetry tracer")
//     } else {
//         // Use stdout exporter for local development
//         opentelemetry_sdk::exporter::trace::stdout::new_pipeline()
//             .with_trace_config(trace::config().with_resource(resource.clone()))
//             .install_simple()
//             .expect("Failed to install OpenTelemetry stdout tracer")
//     };
//
//     let opentelemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
//
//     // Create a JSON formatter layer for logs
//     let formatting_layer = BunyanFormattingLayer::new(app_name.clone(), std::io::stdout);
//
//     // configure env filter from RUST_LOG environment variable
//     let env_filter = EnvFilter::try_from_default_env()
//         .unwrap_or_else(|_| EnvFilter::new("info"));
//
//     // combine all layers
//     let subscriber = Registry::default()
//         .with(env_filter)
//         .with(JsonStorageLayer)
//         .with(formatting_layer)
//         .with(opentelemetry_layer);
//
//     // set the subscriber as global default
//     set_global_default(subscriber)
//         .expect("Failed to set global subscriber");
//
//     // bridge log events to tracing
//     tracing_log::LogTracer::init()
//         .expect("Failed to initialize log tracer");
//
//     Ok(())
// }
//
// /// shutdown telemetry providers
// pub fn shutdown_telemetry() {
//     opentelemetry::global::shutdown_tracer_provider();
// }

use opentelemetry::{KeyValue, runtime};
use opentelemetry::sdk::{Resource, trace};
use opentelemetry::trace::TraceError;
use opentelemetry_otlp::WithExportConfig;

pub fn init_trace() -> Result<trace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "basic-rust-api",
            )])),
        )
        .install_batch(runtime::Tokio)
}