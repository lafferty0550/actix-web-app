use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize)]
pub struct CategoryModel {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    //pub images: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct InsertableCategoryModel {
    pub name: String,
    pub description: String,
    //pub images: Vec<String>,
}