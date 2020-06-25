use actix_web::{
    web,
    middleware,
    App,
    HttpServer,
};
use actix_cors::Cors;
use mongodb::{
    Client,
    Database,
    Collection,
    error::Result as MongoResult,
};
use serde::Serialize;
use std::sync::Arc;

mod routes;
mod services;
mod models;

use services::Services;

#[derive(Serialize)]
pub struct Res<'a, T> {
    success: bool,
    data: Option<T>,
    msg: Option<&'a str>,
}

impl<'a, T> Res<'a, T> {
    pub fn build(data: Option<T>, msg: Option<&'a str>) -> Self {
        Self {
            success: true,
            data,
            msg,
        }
    }
}

#[derive(Serialize)]
pub struct ErrRes<'a> {
    success: bool,
    msg: Option<&'a str>,
}

impl<'a> ErrRes<'a> {
    pub fn build(msg: Option<&'a str>) -> Self {
        Self {
            success: false,
            msg,
        }
    }
}

pub struct AppState {
    services: Arc<Services>,
}

impl AppState {
    pub fn new(services: Arc<Services>) -> Self {
        Self {
            services,
        }
    }
}

async fn init_services() -> MongoResult<Arc<Services>> {
    let client: Client = Client::with_uri_str("mongodb://localhost:27017").await?;
    println!("Connected to DB successfuly!");
    let db: Database = client.database("fullstack-demo-shop");
    let categories: Arc<Collection> = Arc::new(db.collection("categories"));
    let images: Arc<Collection> = Arc::new(db.collection("images"));

    Ok(Arc::new(Services::new(categories, images)))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let services_ptr = match init_services().await {
        Ok(services) => services,
        Err(e) => panic!("Failed connection to Mongo: {:?}", e),
    };

    HttpServer::new(move || {
        App::new()
            .data(AppState::new(services_ptr.clone()))
            .wrap(Cors::new().supports_credentials().finish())
            .wrap(middleware::NormalizePath)
            .service(web::scope("/categories").configure(routes::categories::init))
    })
        .bind("127.0.0.1:3000")?
        .run()
        .await
}