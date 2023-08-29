use crate::api_error::ApiError;

pub fn config_app(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(actix_web::web::resource("/").route(actix_web::web::get().to(index)))
        .service(actix_web::web::resource("/hello/{name}").route(actix_web::web::get().to(hello)));
}

async fn index() -> Result<actix_web::HttpResponse, ApiError> {
    log::debug!("Hitting `index`");
    Ok(actix_web::HttpResponse::Ok().body("Hello world !"))
}

async fn hello(name: actix_web::web::Path<String>) -> Result<actix_web::HttpResponse, ApiError> {
    log::debug!("Hitting `hello`");
    Ok(actix_web::HttpResponse::Ok().body(format!("Hello {} !", name)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header::ContentType, test, web, App};

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        let body = test::read_body(res).await;
        assert_eq!(body, "Hello world !");
    }

    #[actix_web::test]
    async fn test_hello_get() {
        let app = test::init_service(App::new().route("/hello/{name}", web::get().to(hello))).await;
        let req = test::TestRequest::get().uri("/hello/Michel").to_request();
        let res = test::call_service(&app, req).await;
        assert!(res.status().is_success());
        let body = test::read_body(res).await;
        assert_eq!(body, "Hello Michel !");
    }
}
