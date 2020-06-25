use mongodb::{
    Collection,
    Cursor,
    bson,
    bson::{
        Bson as BsonTypes,
        oid::ObjectId,
        doc,
        from_bson,
        to_bson,
    },
    results::InsertOneResult,
    error::{
        Result as MongoResult,
        Error as MongoError,
    },
};
use std::sync::Arc;
use futures::StreamExt;

use crate::models::{
    CategoryModel,
    InsertableCategoryModel,
    EditableCategoryModel,
};

macro_rules! doc_to_struct {
    ($_doc:expr) => {
        from_bson(BsonTypes::Document($_doc)).unwrap()
    }
}

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
        let cur: Cursor = self.col.find(None, None).await?;
        let result: Vec<MongoResult<bson::Document>> = cur.collect().await;
        Ok(result.into_iter().map(|res| doc_to_struct!(res.unwrap())).collect())
    }

    pub async fn find_one(&self, oid: ObjectId) -> Result<CategoryModel, MongoError> {
        let doc = self.col.find_one(doc! {"_id": oid}, None).await?;
        Ok(doc_to_struct!(doc.unwrap()))
    }

    pub async fn insert(
        &self,
        category: InsertableCategoryModel,
    ) -> Result<String, MongoError> {
        let bson = to_bson(&category)?;
        let doc: bson::Document = bson.as_document().unwrap().clone();
        let result: InsertOneResult = self.col.insert_one(doc, None).await?;
        Ok(result.inserted_id.as_object_id().unwrap().to_string())
    }

    pub async fn insert_many(
        &self,
        categories: Vec<InsertableCategoryModel>,
    ) -> Result<usize, MongoError> {
        let categories: Vec<bson::Document> = categories.into_iter()
            .map(|doc| {
                let bson = to_bson(&doc).unwrap();
                bson.as_document().unwrap().clone()
            }).collect();
        let result = self.col.insert_many(categories.into_iter(), None).await?;
        Ok(result.inserted_ids.len())
    }

    pub async fn delete_one(&self, oid: ObjectId) -> Result<(), MongoError> {
        self.col.delete_one(doc! {"_id": oid}, None).await?;
        Ok(())
    }

    pub async fn delete_all(&self) -> Result<i64, MongoError> {
        let result = self.col.delete_many(doc! {}, None).await?;
        Ok(result.deleted_count)
    }

    pub async fn update_one(&self, oid: ObjectId, changes: EditableCategoryModel) -> Result<(), MongoError> {
        self.col.update_one(
            doc! {"_id": oid},
            doc! {"$set": changes.get_changes()},
            None,
        ).await?;
        Ok(())
    }
}