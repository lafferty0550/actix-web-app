use actix_web::{
    get,
    post,
    delete,
    patch,
    web,
    HttpResponse,
};
use actix_multipart::Multipart;
use mongodb::bson::oid::ObjectId;
use futures::TryStreamExt;
use async_std::prelude::*;

use crate::{
    AppState,
    Res,
    ErrRes,
    models::{
        InsertableCategoryModel,
        EditableCategoryModel,
    },
};
use std::path::Path;
use std::ffi::OsStr;

#[get("")]
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

#[get("/{id}")]
async fn get_category(
    (state, info):
    (web::Data<AppState>, web::Path<String>)
) -> HttpResponse {
    let oid = match ObjectId::with_string(info.into_inner().as_str()) {
        Ok(oid) => oid,
        Err(e) => {
            eprintln!("Error generating OID: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ErrRes::build(Some("Invalid categoryID")));
        }
    };
    match state.services.categories.find_one(oid).await {
        Ok(category) => HttpResponse::Ok()
            .json(Res::build(Some(category), None)),
        Err(e) => {
            eprintln!("Error finding category: {:?}", e);
            HttpResponse::InternalServerError()
                .json(ErrRes::build(Some("Failed to find category")))
        }
    }
}

#[post("/")]
async fn add_category(
    (state, body):
    (web::Data<AppState>, web::Json<InsertableCategoryModel>)
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
    (state, body):
    (web::Data<AppState>, web::Json<Vec<InsertableCategoryModel>>)
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
    (state, info):
    (web::Data<AppState>, web::Path<String>)
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

#[patch("/{id}/")]
async fn update_category(
    (state, body, info):
    (web::Data<AppState>, web::Json<EditableCategoryModel>, web::Path<String>)
) -> HttpResponse {
    let oid = match ObjectId::with_string(info.into_inner().as_str()) {
        Ok(oid) => oid,
        Err(e) => {
            eprintln!("Error generating OID: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ErrRes::build(Some("Invalid categoryID")));
        }
    };
    match state.services.categories.update_one(oid, body.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(Res::<()>::build(None, None)),
        Err(e) => {
            eprintln!("Error updating category! {:?}", e);
            HttpResponse::Ok().json(ErrRes::build(Some("Failed to update category")))
        }
    }
}

#[post("/upload/")]
async fn upload_img(
    (state, mut payload):
    (web::Data<AppState>, Multipart)
) -> HttpResponse {
    let mut id: String = String::default();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();

        id = match state.services.images.generate().await {
            Ok(oid) => oid.to_hex(),
            Err(e) => {
                eprintln!("Failed to create image: {:?}", e);
                return HttpResponse::InternalServerError()
                    .json(ErrRes::build(Some("Failed to create image")));
            }
        };
        let ext = Path::new(content_type.get_filename().unwrap())
            .extension().and_then(OsStr::to_str).unwrap();
        let filepath = format!("./images/{}.{}", id, ext);

        let mut f = match async_std::fs::File::create(filepath).await {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to create image: {:?}", e);
                return HttpResponse::InternalServerError()
                    .json(ErrRes::build(Some("Failed to create image")));
            }
        };

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            match f.write_all(&data).await {
                Ok(()) => (),
                Err(e) => {
                    eprintln!("Failed to create image: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ErrRes::build(Some("Failed to create image")));
                }
            };
        }
    }
    HttpResponse::Ok().json(Res::build(Some(id), None))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_categories);
    cfg.service(get_category);
    cfg.service(add_category);
    cfg.service(add_categories);
    cfg.service(delete_category);
    cfg.service(delete_categories);
    cfg.service(update_category);
    cfg.service(upload_img);
}