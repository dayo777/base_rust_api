mod models;
mod handlers;
mod utils;
mod errors;
mod db_connection;
mod routes;
mod auth;
mod telemetry;

// import crates here
use std::io;
use actix_web::{web, App, HttpServer};
use tracing::info;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::global;
use tracing_subscriber::layer::SubscriberExt;
use crate::routes::all_routes;
use crate::telemetry::init_trace;

#[tokio::main]
#[tracing::instrument]
async fn main() -> io::Result<()> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = init_trace().unwrap();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let s = HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/v1").configure(all_routes)
            )
        })
        .bind("127.0.0.1:8080")?;

    info!("Server started at 127.0.0.1:8080");
    let s = s.run().await;
    s
}

/*
    start the server on:  localhost:8080/api/v1/health
    Route-1 has to be called with the full JSON fields:
        {
            "field1": "dd1",
            "field2": "dd2",
            "field3": "dd3"
        }
 */