use actix_web::{
    get,
    post,
    delete,
    web,
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

use crate::{
    AppState,
    Res,
    ErrRes,
    models::InsertableCategoryModel,
};

#[get("/")]
async fn get_categories(
    state: web::Data<AppState>,
) -> HttpResponse {
    match state.services.categories.find_all().await {
        Ok(categories) => HttpResponse::Ok()
            .json(Res::build(Some(categories), None)),
        Err(e) => {
            eprintln!("Error getting categories: {:?}", e);
            HttpResponse::InternalServerError()
                .json(ErrRes::build(Some("Failed to get categories")))
        }
    }
}

#[post("/")]
async fn add_category(
    state: web::Data<AppState>,
    body: web::Json<InsertableCategoryModel>,
) -> HttpResponse {
    match state.services.categories.insert(body.into_inner()).await {
        Ok(id) => HttpResponse::Ok()
            .json(Res::build(Some(id), None)),
        Err(e) => {
            eprintln!("Error inserting categories: {:?}", e);
            HttpResponse::InternalServerError()
                .json(ErrRes::build(Some("Failed to insert category")))
        }
    }
}

#[post("/many/")]
async fn add_categories(
    state: web::Data<AppState>,
    body: web::Json<Vec<InsertableCategoryModel>>,
) -> HttpResponse {
    match state.services.categories.insert_many(body.into_inner()).await {
        Ok(usize) => HttpResponse::Ok()
            .json(Res::build(Some(usize), None)),
        Err(e) => {
            eprintln!("Error inserting categories: {:?}", e);
            HttpResponse::InternalServerError()
                .json(ErrRes::build(Some("Failed to insert categories")))
        }
    }
}


#[delete("/")]
async fn delete_categories(
    state: web::Data<AppState>
) -> HttpResponse {
    match state.services.categories.delete_all().await {
        Ok(count) => HttpResponse::Ok()
            .json(Res::build(Some(count), None)),
        Err(e) => {
            eprintln!("Error deleting category: {:?}", e);
            HttpResponse::InternalServerError()
                .json(ErrRes::build(Some("Failed to delete categories")))
        }
    }
}


#[delete("/{id}/")]
async fn delete_category(
    state: web::Data<AppState>,
    info: web::Path<String>,
) -> HttpResponse {
    let oid = match ObjectId::with_string(info.into_inner().as_str()) {
        Ok(oid) => oid,
        Err(e) => {
            eprintln!("Error generating OID: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ErrRes::build(Some("Invalid categoryID")));
        }
    };
    match state.services.categories.delete_one(oid).await {
        Ok(()) => HttpResponse::Ok()
            .json(Res::<()>::build(None, None)),
        Err(e) => {
            eprintln!("Error deleting category: {:?}", e);
            HttpResponse::InternalServerError()
                .json(ErrRes::build(Some("Failed to delete category")))
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_categories);
    cfg.service(add_category);
    cfg.service(add_categories);
    cfg.service(delete_category);
    cfg.service(delete_categories);
}