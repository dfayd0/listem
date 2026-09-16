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

type ApiResult<T> = Result<T, (axum::http::StatusCode, String)>;

fn db_err<E>(_: E) -> (axum::http::StatusCode, String)
{
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        "database error".into(),
    )
}

fn db_not_found(_: diesel::result::Error) -> (axum::http::StatusCode, String)
{
    (axum::http::StatusCode::NOT_FOUND, "todo not found".into())
}

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
    Redirect::to("/home")
}

#[derive(Template, IntoResponse)]
#[template(path = "404.html")]
pub struct NotFoundTemplate {}

#[axum::debug_handler]
pub async fn not_found() -> NotFoundTemplate
{
    NotFoundTemplate {}
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
    sort:            String,
}

#[axum::debug_handler]
pub async fn todolist(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> TodoListTemplate
{
    let mut conn = state.db_pool.get().expect("Failed to get DB connection");
    let mut todos = db::get_todos(&mut conn).expect("Error loading todos");

    let sort = params
        .get("sort")
        .cloned()
        .unwrap_or_else(|| "none".to_string());

    match sort.as_str() {
        "title" => todos.sort_by_key(|t| t.title.to_lowercase()),
        "importance" => todos.sort_by_key(|t| match t.importance.as_str() {
            "high" => 0,
            "medium" => 1,
            _ => 2,
        }),
        _ => {}
    }

    let total_completed = todos.iter().filter(|t| t.completed).count() as i64;
    let total_remaining = todos.len() as i64 - total_completed;
    TodoListTemplate {
        todos,
        total_completed,
        total_remaining,
        sort,
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
) -> ApiResult<TodoTemplate>
{
    if let Err(msg) = form.validate()
    {
        return Err((axum::http::StatusCode::UNPROCESSABLE_ENTITY, msg));
    }

    let mut conn = state.db_pool.get().map_err(db_err)?;
    let new_todo: Todo = db::create_todo(&mut conn, form).map_err(db_err)?;

    Ok(TodoTemplate {
        todo: new_todo,
    })
}

#[axum::debug_handler]
pub async fn delete_todo(
    State(state): State<AppState>, Path(todo_id): Path<i32>,
) -> ApiResult<axum::http::StatusCode>
{
    let mut conn = state.db_pool.get().map_err(db_err)?;

    let deleted = db::delete_todo_by_id(&mut conn, todo_id).map_err(db_err)?;
    if deleted == 0
    {
        return Err(db_not_found(diesel::result::Error::NotFound));
    }

    Ok(axum::http::StatusCode::OK)
}

#[axum::debug_handler]
pub async fn toggle_todo(
    State(state): State<AppState>, Path(todo_id): Path<i32>,
) -> ApiResult<TodoTemplate>
{
    let mut conn = state.db_pool.get().map_err(db_err)?;
    let todo = db::toggle_todo_by_id(&mut conn, todo_id).map_err(db_not_found)?;

    Ok(TodoTemplate {
        todo,
    })
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
) -> ApiResult<EditTodoTemplate>
{
    let mut conn = state.db_pool.get().map_err(db_err)?;
    let todo = db::get_todo_by_id(&mut conn, todo_id).map_err(db_not_found)?;

    Ok(EditTodoTemplate {
        todo,
    })
}

#[axum::debug_handler]
pub async fn edit_todo(
    State(state): State<AppState>, Form(todo): Form<TodoForm>,
) -> ApiResult<TodoTemplate>
{
    if let Err(msg) = todo.validate()
    {
        return Err((axum::http::StatusCode::UNPROCESSABLE_ENTITY, msg));
    }

    let mut conn = state.db_pool.get().map_err(db_err)?;
    let new_todo = db::edit_todo(&mut conn, todo).map_err(db_not_found)?;

    Ok(TodoTemplate {
        todo: new_todo,
    })
}
