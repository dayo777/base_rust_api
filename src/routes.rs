use crate::handlers;
use actix_web::{web, Responder, HttpResponse};
use tracing::{info, error, instrument};
use crate::models::{
    Model1, Model2
};


// add all routes to this function
pub fn all_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
    cfg.route("/route1", web::post().to(route1));
    cfg.route("/route2", web::post().to(route2));
}


// all other route implementation goes below

#[instrument(skip(_model1))]
async fn route1(_model1: web::Json<Model1>) -> impl Responder {
    info!("Processing route 1");
    match handlers::handler_1(_model1).await {
        Ok(result) => {
            info!("Handler-1 returns successfully");
            HttpResponse::Ok().json(result)
        },
        Err(e) => {
            error!("Handler-1 returns with error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn health_check_handler() -> impl Responder {
    handlers::health_check().await
}

#[instrument(skip(_model2))]
async fn route2(_model2: web::Json<Model2>) -> impl Responder {
    info!("Processing route 2");

    match handlers::handler_2(_model2).await {
        Ok(_) => {
            info!("Handler-2 returns successfully");
            HttpResponse::Ok().json("Handler-2 return successfully")
        },
        Err(_) => {
            error!("Handler-2 returns with error");
            HttpResponse::InternalServerError().finish()
        }
    }
}