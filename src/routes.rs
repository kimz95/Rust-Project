use actix_web::web;
use crate::health::*;

pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/api").route("/healthchecker",web::get().to(health::health_check_handler))
    );
}