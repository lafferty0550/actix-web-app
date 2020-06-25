use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

pub struct Image {
    _id: ObjectId,
    upload_date: DateTime<Utc>,
}