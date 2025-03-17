use crate::handlers;
use actix_web::{web, Responder};
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
async fn route1(_model1: web::Json<Model1>) -> impl Responder {
    handlers::handler_1(_model1).await.unwrap()
}

async fn health_check_handler() -> impl Responder {
    handlers::health_check().await
}

async fn route2(_model2: web::Json<Model2>) -> impl Responder {
    match handlers::handler_2(_model2).await {
        Ok(_) => "Handler-2 returns successfully",
        Err(_) => "Handler-2 returns with error"
    }
}