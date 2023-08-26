use actix_web::{get, HttpResponse, Responder, web};

use super::AppState;
use super::log_request;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(status);
}

#[get("/status")]
async fn status(data: web::Data<AppState<'_>>) -> impl Responder {
    log_request("GET: /status", &data.connections);

    HttpResponse::Ok().body("I am up")
}
