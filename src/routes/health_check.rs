use actix_web::{web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

async fn get() -> impl Responder {
    return HttpResponse::Ok();
}

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health-check").route(web::get().to(get)));
}
