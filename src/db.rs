use ::dotenvy::dotenv;
use ::std::env;
use diesel::{
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};

use crate::models::{
    NewTodo,
    Todo,
    TodoForm,
};

pub fn establish_connection() -> SqliteConnection
{
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_db_pool() -> Pool<ConnectionManager<SqliteConnection>>
{
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create pool")
}

// Diesel can insert more than one record in a single query. Just pass a Vec
// or slice to insert_into, and then call get_results instead of get_result.
// If you don’t actually want to do anything with the row that was just
// inserted, call .execute instead. The compiler won’t complain at you, that
// way. :)

pub fn create_todo<'a>(conn: &mut SqliteConnection, new_todo: NewTodo) -> Todo
{
    use crate::schema::todos;

    let new_todo = new_todo;

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

pub fn delete_todo_by_id(conn: &mut SqliteConnection, todo_id: i32)
{
    use crate::schema::todos::dsl::*;

    diesel::delete(todos.filter(id.eq(todo_id)))
        .execute(conn)
        .expect("Error deleting todo");
}

pub fn get_todo_by_id(conn: &mut SqliteConnection, todo_id: i32) -> Todo
{
    use crate::schema::todos::dsl::*;

    todos
        .find(todo_id)
        .first::<Todo>(conn)
        .expect("Error loading todo")
}

pub fn toggle_todo_by_id(conn: &mut SqliteConnection, todo_id: i32) -> Todo
{
    use crate::schema::todos::dsl::*;

    let todo = todos
        .find(todo_id)
        .first::<Todo>(conn)
        .expect("Error loading todo");

    let new_status = !todo.completed;

    diesel::update(todos.find(todo_id))
        .set(completed.eq(new_status))
        .returning(Todo::as_returning())
        .get_result(conn)
        .expect("Error updating todo")
}

pub fn edit_todo(conn: &mut SqliteConnection, todo: TodoForm) -> Todo
{
    use crate::schema::todos::dsl::*;

    diesel::update(todos.find(todo.id))
        .set((
            title.eq(&todo.title),
            description.eq(&todo.description),
            importance.eq(&todo.importance),
        ))
        .returning(Todo::as_returning())
        .get_result(conn)
        .expect("Error updating todo")
}
