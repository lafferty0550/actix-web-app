use serde::{Serialize, Deserialize};
use mongodb::{
    bson::Document,
    bson::oid::ObjectId
};

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

#[derive(Serialize, Deserialize, Clone)]
pub struct EditableCategoryModel {
    pub name: Option<String>,
    pub description: Option<String>,
    //pub images: Vec<String>,
}

impl EditableCategoryModel {
    pub fn get_changes(&self) -> Document {
        let mut doc = Document::new();
        if self.name.is_some() {
            doc.insert("name", self.name.as_ref().unwrap());
        }
        if self.description.is_some() {
            doc.insert("description", self.description.as_ref().unwrap());
        }
        doc
    }
}