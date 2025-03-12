use std::env;

use diesel::prelude::*;
use dotenvy::dotenv;

pub fn establish_connection() -> SqliteConnection
{
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use crate::models::{
    NewTodo,
    Todo,
};

// Diesel can insert more than one record in a single query. Just pass a Vec
// or slice to insert_into, and then call get_results instead of get_result.
// If you don’t actually want to do anything with the row that was just
// inserted, call .execute instead. The compiler won’t complain at you, that
// way. :)

pub fn create_todo<'a>(
    conn: &mut SqliteConnection, title: &'a str, description: &'a str,
) -> Todo
{
    use crate::schema::todos;

    let new_todo = NewTodo {
        title,
        description,
    };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(conn)
        .expect("Error saving new todo")
}

pub fn get_todos(conn: &mut SqliteConnection) -> Vec<Todo>
{
    use crate::schema::todos::dsl::*;

    todos.load::<Todo>(conn).expect("Error loading todos")
}
