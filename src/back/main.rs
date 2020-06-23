use actix_web::{web, App, HttpServer};

mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/categories").configure(routes::categories::init))
    })
        .bind("127.0.0.1:3000")?
        .run()
        .await
}