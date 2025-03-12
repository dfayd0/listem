use askama::Template;
use askama_derive_axum::IntoResponse;
use axum::{
    extract::State,
    response::Redirect,
};

use crate::{
    db,
    models::Todo,
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
pub async fn about(State(state): State<AppState>) -> AboutTemplate
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
    todos: Vec<Todo>,
}

#[axum::debug_handler]
pub async fn todolist(State(state): State<AppState>) -> TodoListTemplate
{
    let mut conn = state
        .db_pool
        .get()
        .expect("Failed to get DB connection from pool");

    let todos = db::get_todos(&mut conn);

    TodoListTemplate {
        todos,
    }
}

#[derive(Template, IntoResponse)]
#[template(path = "todo.html")]
pub struct TodoTemplate
{
    todo: Todo,
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoForm
{
    pub title:       String,
    pub importance:  String,
    pub description: String,
}

use axum::extract::Form;
#[axum::debug_handler]
pub async fn add_todo(
    State(state): State<AppState>, Form(form): Form<CreateTodoForm>,
) -> TodoTemplate
{
    let mut conn = state
        .db_pool
        .get()
        .expect("Failed to get DB connection from pool");

    let new_todo = db::create_todo(&mut conn, &form.title, &form.description);

    TodoTemplate {
        todo: new_todo
    }
}
