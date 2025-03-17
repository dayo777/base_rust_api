mod models;
mod handlers;
mod utils;
mod errors;
mod db_connection;
mod routes;


use std::io;
use actix_web::{web, App, HttpServer};
use crate::routes::all_routes;

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/v1").configure(all_routes)
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
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