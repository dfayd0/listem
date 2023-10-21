use crate::database::database::*;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, State};
use rocket::response::content::RawHtml;

use rocket::form::Form;
extern crate tera;
use tera::{Context, Tera};


#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct TaskInput {
    task: String,
}

#[post("/add", data = "<task_input>")]
pub fn add_task(task_input: Form<TaskInput>, pool: &State<DbPool>) -> status::Accepted<String> {
    let mut conn = pool
        .get()
        .expect("Failed to get a database connection from the pool");

    let task = task_input.task.clone(); // Access the task field from the deserialized struct

    create_todo(&mut conn, task.as_str());
    status::Accepted(None)
}

use crate::database::database;
use crate::database::models::Todo;

#[get("/todos")]
pub fn get_todos(pool: &State<DbPool>) -> Result<Json<Vec<Todo>>, Status> {
    let mut conn = pool
        .get()
        .expect("Failed to get a database connection from the pool");

    match database::get_all_todos(&mut conn) {
        Ok(todos) => Ok(Json(todos)),
        Err(err) => {
            eprintln!("Error fetching todos: {}", err);
            Err(Status::NotFound)
        }
    }
}

#[get("/tasks")]
pub fn get_tasks(tera: &State<Tera>, pool: &State<DbPool>) -> Result<RawHtml<String>, Status> {
    let mut context = Context::new(); 
    
    let mut conn = pool
        .get()
        .expect("Failed to get a database connection from the pool");
    
    let tasks = match database::get_all_todos(&mut conn) {
        Ok(result) => result,
        Err(_) => return Err(Status::new(402)),
    };

     println!("{:?}", tasks);
    context.insert("tasks", &tasks);
    println!("{:?}", context);
    let rendered: String = match tera.render("tasks_loop.html.tera", &context) {
        Ok(result) => result,
        Err(e) => {
            println!("{:?}", e);
            return Err(Status::new(443))
        }
    };
    Ok(RawHtml(rendered))
}
