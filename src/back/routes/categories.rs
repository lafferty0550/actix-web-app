use actix_web::{web, HttpResponse};

async fn get_categories(

) -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_categories);
}