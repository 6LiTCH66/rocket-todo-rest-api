use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Todo{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub todo_title: String,
    pub todo_description: String,

}