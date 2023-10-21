use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

use crate::database::models::{NewTodo, Todo};
use crate::database::schema::todos;

// use super::schema::todos::description;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// we set a r2d2 pool for an easier access to the database variable
pub fn init_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool")
}

pub fn create_todo(conn: &mut SqliteConnection, title: &str) -> Todo {
    // let new_todo = NewTodo { title, description };
    let new_todo = NewTodo { title, description: &"I'm a description" };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_all_todos(conn: &mut SqliteConnection) -> Result<Vec<Todo>, diesel::result::Error> {
    todos::table.load(conn)
}
