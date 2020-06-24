use mongodb::{
    Collection,
    Cursor,
    bson,
    bson::{oid::ObjectId, doc},
    results::InsertOneResult,
    error::{
        Result as MongoResult,
        Error as MongoError,
        ErrorKind,
    },
};
use std::sync::Arc;
use futures::StreamExt;

use crate::models::{
    CategoryModel,
    InsertableCategoryModel,
};

pub struct CategoriesService {
    col: Arc<Collection>,
}

impl CategoriesService {
    pub fn new(col: Arc<Collection>) -> Self {
        Self {
            col,
        }
    }

    pub async fn find_all(&self) -> Result<Vec<CategoryModel>, MongoError> {
        let cur: Cursor = match self.col.find(None, None).await {
            Ok(cur) => cur,
            Err(e) => {
                eprintln!("Failed to find categories: {:?}", e);
                return Err(e);
            }
        };
        let result: Vec<MongoResult<bson::Document>> = cur.collect().await;

        Ok(result.into_iter().map(|res| {
            let doc: bson::Document = res.unwrap();
            // TODO: converting whole document to struct
            CategoryModel {
                _id: doc.get("_id").unwrap().as_object_id().unwrap().clone(),
                name: doc.get_str("name").unwrap().to_string(),
                description: doc.get_str("description").unwrap().to_string(),
            }
        }).collect())
    }

    pub async fn insert(
        &self,
        category: InsertableCategoryModel,
    ) -> Result<String, MongoError> {
        let bson: bson::Bson = match bson::to_bson(&category) {
            Ok(bson) => bson,
            Err(e) => {
                eprintln!("Failed to convert to bson: {:?}", e);
                return Err(MongoError {
                    kind: Arc::new(ErrorKind::BsonEncode(e)),
                });
            }
        };
        let doc: bson::Document = bson.as_document().unwrap().clone();
        let result: InsertOneResult = match self.col.insert_one(doc, None).await {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Failed to insert document: {:?}", e);
                return Err(e);
            }
        };
        Ok(result.inserted_id.as_object_id().unwrap().to_string())
    }

    pub async fn insert_many(
        &self,
        categories: Vec<InsertableCategoryModel>,
    ) -> Result<usize, MongoError> {
        let categories: Vec<bson::Document> = categories.into_iter()
            .map(|a| -> bson::Document {
                let bson = bson::to_bson(&a).unwrap();
                bson.as_document().unwrap().clone()
            }).collect();
        match self.col.insert_many(categories.into_iter(), None).await {
            Ok(res) => Ok(res.inserted_ids.len()),
            Err(e) => {
                eprintln!("Failed to insert document: {:?}", e);
                return Err(e);
            }
        }
    }

    pub async fn delete_one(&self, oid: ObjectId) -> Result<(), MongoError> {
        match self.col.delete_one(doc! {"_id": oid}, None).await {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to delete document: {:?}", e);
                return Err(e);
            }
        }
    }

    pub async fn delete_all(&self) -> Result<i64, MongoError> {
        match self.col.delete_many(doc! {}, None).await {
            Ok(result) => Ok(result.deleted_count),
            Err(e) => {
                eprintln!("Failed to delete document: {:?}", e);
                return Err(e);
            }
        }
    }
}