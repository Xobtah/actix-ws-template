use crate::api_error::ApiError;
use actix_web::get;

pub fn config_app(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
async fn index() -> Result<actix_web::HttpResponse, ApiError> {
    log::debug!("Hitting index");
    Ok(actix_web::HttpResponse::Ok().body("Hello world !"))
}
