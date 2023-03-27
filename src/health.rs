use actix_web::{get, web, HttpResponse, Responder};

#[get("/liveness")]
pub async fn get_liveness() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/readiness")]
pub async fn get_readiness() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/version")]
pub async fn get_version() -> impl Responder {
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_liveness)
        .service(get_readiness)
        .service(get_version);
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::StatusCode,
        test,
        web::{self, Bytes},
        App,
    };

    #[actix_web::test]
    async fn test_get_liveness() {
        let app =
            test::init_service(App::new().service(web::scope("/health").configure(config))).await;
        let req = test::TestRequest::get()
            .uri("/health/liveness")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_readiness() {
        let app =
            test::init_service(App::new().service(web::scope("/health").configure(config))).await;
        let req = test::TestRequest::get()
            .uri("/health/readiness")
            .to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_version() {
        let app =
            test::init_service(App::new().service(web::scope("/health").configure(config))).await;
        let req = test::TestRequest::get().uri("/health/version").to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
        let res_body: Bytes = test::read_body(res).await;

        assert_eq!(res_body, env!("CARGO_PKG_VERSION"));
    }
}
