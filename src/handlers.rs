// TODO: implement logic for models here
use tracing::info;
use crate::models::{
    Model1, Model2
};
use crate::errors::CustomError;
use actix_web::web;

pub async fn health_check() -> &'static str {
    "Server is running!"
}

pub async fn handler_1(model1: web::Json<Model1>) -> Result<Model1, CustomError> {
    info!("Handler-1 is called for: {field1}", field1 = model1.field1.as_str());
    Ok(Model1 {
        field1: model1.field1.to_string(),
        field2: model1.field2.to_string(),
        field3: model1.field3.to_string(),
    })
    // Ok(("Handler-1 returns successfully").to_string())
}

pub async fn handler_2(model2: web::Json<Model2>) -> Result<(), CustomError> {
    info!("Handler-2 is called for {field1}", field1 = model2.field1.as_str());
    Err(CustomError::Error1)
}
