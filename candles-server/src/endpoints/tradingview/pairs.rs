use crate::Context;
use actix_web::{get, web, Responder};

#[get("/pairs")]
pub async fn get_pairs(context: web::Data<Context>) -> impl Responder {
    web::Json(context.markets.clone())
}
