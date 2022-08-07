use std::env;
extern crate dotenv;
use dotenv::dotenv;
use std::error::Error as std_error;

use mongodb::{bson::{extjson::de::Error}, bson, results::{InsertOneResult}, sync::{Client, Collection}};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{DeleteResult, UpdateResult};
use crate::models::todo_model::Todo;

pub struct MongoRepo{
    col: Collection<Todo>
}

impl MongoRepo{

    pub fn init() -> Result<Self, Box<dyn std_error>>{
        dotenv().ok();
        let uri = match env::var("DATABASEURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Cannot connect to database! Invalid URI"),
        };
        let client = Client::with_uri_str(uri)?;
        let db = client.database("rustTodoAPI");
        let col: Collection<Todo> = db.collection("Todo");
        Ok(MongoRepo{col})
    }

    pub fn create_todo(&self, new_todo: Todo) -> Result<InsertOneResult, Error>{
        let new_doc = Todo{
            _id: None,
            todo_title: new_todo.todo_title,
            todo_description: new_todo.todo_description
        };
        let todo = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating todo");
        Ok(todo)
    }
    pub fn get_todo_by_id(&self, id: &String) -> Result<Todo, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"_id": obj_id};
        let todo_detail = self
            .col
            .find_one(filter, None).ok().expect("Error getting todo!");
        Ok(todo_detail.unwrap())
    }

    pub fn update_todo(&self, id: &String, new_todo: Todo) -> Result<UpdateResult, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc!{"_id": obj_id};
        let new_doc = doc!{
            "$set":{
                "todo_title": new_todo.todo_title,
                "todo_description": new_todo.todo_description
            },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating todo!");
        Ok(updated_doc)
    }

    pub fn delete_todo(&self, id: &String) -> Result<DeleteResult, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let deleted_doc = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting todo!");
        Ok(deleted_doc)
    }

    pub fn get_all_todos(&self) -> Result<Vec<Todo>, Error>{
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting all todos");
        let todos = cursors.map(|todo| todo.unwrap()).collect::<Vec<Todo>>();
        Ok(todos)
    }
}