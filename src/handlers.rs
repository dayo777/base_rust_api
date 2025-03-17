// TODO: implement logic for models here

use crate::models::{
    Model1, Model2
};
use crate::errors::CustomError;
use actix_web::web;

pub async fn health_check() -> &'static str {
    "Server is running!"
}

pub async fn handler_1(_model1: web::Json<Model1>) -> Result<String, CustomError> {
    Ok(("Handler-1 returns successfully").to_string())
}

pub async fn handler_2(_model2: web::Json<Model2>) -> Result<(), CustomError> {
    Err(CustomError::Error1)
}
