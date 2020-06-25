use mongodb::{
    Collection,
    bson::doc,
    error::Error as MongoError,
    bson::oid::ObjectId,
};
use std::sync::Arc;
use chrono::Utc;

pub struct ImagesService {
    col: Arc<Collection>,
}

impl ImagesService {
    pub fn new(col: Arc<Collection>) -> Self {
        Self { col }
    }

    pub async fn generate(&self) -> Result<ObjectId, MongoError> {
        let result = self.col.insert_one(doc! {"upload_date": Utc::now()}, None).await?;
        Ok(result.inserted_id.as_object_id().unwrap().clone())
    }
}