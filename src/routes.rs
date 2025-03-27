#![allow(unexpected_cfgs)]
use ::axum::extract::Path;
use askama::Template;
use askama_derive_axum::IntoResponse;
use axum::{
    extract::{
        Form,
        State,
    },
    response::Redirect,
};

use crate::{
    db,
    models::{
        NewTodo,
        Todo,
        TodoForm,
    },
    AppState,
};

pub async fn index() -> Redirect
{
    Redirect::permanent("/home")
}

#[derive(Template, IntoResponse)]
#[template(path = "about.html")]
pub struct AboutTemplate {}

#[axum::debug_handler]
pub async fn about(State(_state): State<AppState>) -> AboutTemplate
{
    AboutTemplate {}
}

#[derive(Template, IntoResponse)]
#[template(path = "home.html")]
pub struct HomeTemplate
{
    name: String,
}

#[axum::debug_handler]
pub async fn home(State(state): State<AppState>) -> HomeTemplate
{
    HomeTemplate {
        name: state.app_name.clone(),
    }
}

#[derive(Template, IntoResponse)]
#[template(path = "todolist.html")]
pub struct TodoListTemplate
{
    todos:           Vec<Todo>,
    total_completed: i64,
    total_remaining: i64,
}

#[axum::debug_handler]
pub async fn todolist(State(state): State<AppState>) -> TodoListTemplate
{
    let mut conn = state.db_pool.get().expect("Failed to get DB connection");
    let todos = db::get_todos(&mut conn);

    let total_completed = todos.iter().filter(|t| t.completed).count() as i64;
    let total_remaining = todos.len() as i64 - total_completed;
    TodoListTemplate {
        todos,
        total_completed,
        total_remaining,
    }
}

#[derive(Template, IntoResponse)]
#[template(path = "todo.html")]
pub struct TodoTemplate
{
    todo: Todo,
}

#[axum::debug_handler]
pub async fn add_todo(
    State(state): State<AppState>, Form(form): Form<NewTodo>,
) -> TodoTemplate
{
    let mut conn = state.db_pool.get().expect("Failed to get DB connection");
    let new_todo: Todo = db::create_todo(&mut conn, form);

    TodoTemplate {
        todo: new_todo
    }
}

#[axum::debug_handler]
pub async fn delete_todo(
    State(state): State<AppState>, Path(todo_id): Path<i32>,
) -> impl axum::response::IntoResponse
{
    let mut conn = state.db_pool.get().expect("Failed to get DB connection");

    db::delete_todo_by_id(&mut conn, todo_id);

    axum::http::StatusCode::OK
}

#[axum::debug_handler]
pub async fn toggle_todo(
    State(state): State<AppState>, Path(todo_id): Path<i32>,
) -> TodoTemplate
{
    let mut conn = state.db_pool.get().expect("Failed to get DB connection");
    let todo = db::toggle_todo_by_id(&mut conn, todo_id);

    TodoTemplate {
        todo,
    }
}

#[derive(Template, IntoResponse)]
#[template(path = "edit-todo-form.html")]
pub struct EditTodoTemplate
{
    todo: Todo,
}

#[axum::debug_handler]
pub async fn edit_todo_form(
    State(state): State<AppState>, Path(todo_id): Path<i32>,
) -> EditTodoTemplate
{
    let mut conn = state.db_pool.get().expect("Failed to get DB connection");
    let todo = db::get_todo_by_id(&mut conn, todo_id);

    EditTodoTemplate {
        todo,
    }
}

#[axum::debug_handler]
pub async fn edit_todo(
    State(state): State<AppState>, Form(todo): Form<TodoForm>,
) -> TodoTemplate
{
    let mut conn = state.db_pool.get().expect("Failed to get DB connection");
    let new_todo = db::edit_todo(&mut conn, todo);

    TodoTemplate {
        todo: new_todo
    }
}
