use crate::utils::time::current_time;
use actix_web::{get, HttpResponse, Responder};

#[get("/time")]
pub async fn get_server_time() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", current_time()))
}
