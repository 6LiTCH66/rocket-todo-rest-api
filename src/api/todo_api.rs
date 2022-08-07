use dotenv::Error;
use mongodb::bson;
use crate::{models::todo_model::Todo, repository::mongodb_repo::MongoRepo};
use mongodb::results::{InsertOneResult, UpdateResult, DeleteResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/todo", data = "<new_todo>")]
pub fn create_todo(db: &State<MongoRepo>, new_todo: Json<Todo>) -> Result<Json<InsertOneResult>, Status>{
    let data = Todo{
        _id: None,
        todo_title: new_todo.todo_title.to_string(),
        todo_description: new_todo.todo_description.to_string(),
    };
    let todo_detail = db.create_todo(data);

    match todo_detail {
        Ok(todo) => Ok(Json(todo)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/todo/<path>")]
pub fn get_todo(db: &State<MongoRepo>, path: String) -> Result<Json<Todo>, Status>{
    let id = path;

    if id.is_empty(){
        return Err(Status::BadRequest);
    };
    let todo_detail = db.get_todo_by_id(&id);
    match todo_detail {
        Ok(todo) => Ok(Json(todo)),
        Err(_) => Err(Status::InternalServerError)
    }
}
#[put("/todo/<path>", data = "<new_todo>")]
pub fn update_todo(db: &State<MongoRepo>, path: String, new_todo: Json<Todo>) -> Result<Json<Todo>, Status>{
    let id = path;

    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let updated_todo = Todo{
        _id: None,
        todo_title: new_todo.todo_title.to_string(),
        todo_description: new_todo.todo_description.to_string(),
    };
    let todo_details = db.update_todo(&id, updated_todo);
    match todo_details {
        Ok(todo) => {
            if todo.matched_count == 1{
                let todo_details = db.get_todo_by_id(&id);
                return match todo_details {
                    Ok(todo_info) => Ok(Json(todo_info)),
                    Err(_) => Err(Status::InternalServerError)
                }
            }
            else{
                return Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::InternalServerError)
    }
}

#[delete("/todo/<path>")]
pub fn delete_todo(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status>{
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let todo_detail = db.delete_todo(&id);
    match todo_detail {
        Ok(todo) => {
            if todo.deleted_count == 1{
                return Ok(Json("Todo was successfuly deleted!"))
            }else{
                return Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/todos")]
pub fn get_all_todos(db: &State<MongoRepo>) -> Result<Json<Vec<Todo>>, Status>{
    let todos = db.get_all_todos();
    match todos {
        Ok(todo) => Ok(Json(todo)),
        Err(_) => Err(Status::InternalServerError)
    }
}