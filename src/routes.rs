use askama::Template;
use askama_derive_axum::IntoResponse;
use axum::{
    extract::State,
    response::Redirect,
};

use crate::{
    models::Todo,
    AppState,
};

pub async fn index() -> Redirect
{
    Redirect::permanent("/home")
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
    TodoListTemplate {
        todos: vec![]
    }
}

#[derive(Template, IntoResponse)]
#[template(path = "about.html")]
pub struct AboutTemplate {}

#[axum::debug_handler]
pub async fn about(State(state): State<AppState>) -> AboutTemplate
{
    AboutTemplate {}
}
