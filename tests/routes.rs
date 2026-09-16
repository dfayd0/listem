mod common;

use axum::{
    body::Body,
    http::{
        Method,
        Request,
        StatusCode,
    },
};
use listem::csrf;

#[tokio::test]
async fn index_redirects_to_home()
{
    let mut app = common::TestApp::new();
    let (status, _) = app.request(app.get("/")).await;

    assert!(status.is_redirection());
}

#[tokio::test]
async fn home_renders_app_name()
{
    let mut app = common::TestApp::new();
    let (status, body) = app.request(app.get("/home")).await;

    assert_eq!(status, StatusCode::OK);
    assert!(String::from_utf8_lossy(&body).contains("Listem"));
}

#[tokio::test]
async fn todolist_renders_seeded_todos()
{
    let mut app = common::TestApp::new();
    let todo = common::seed_todo(&app.pool, "from test");
    let (status, body) = app.request(app.get("/todolist")).await;

    assert_eq!(status, StatusCode::OK);
    assert!(String::from_utf8_lossy(&body).contains("from test"));
    assert!(String::from_utf8_lossy(&body).contains(&todo.id.to_string()));
}

#[tokio::test]
async fn unknown_route_is_404()
{
    let mut app = common::TestApp::new();
    let (status, _) = app.request(app.get("/nope")).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn first_get_sets_csrf_cookie()
{
    let mut app = common::TestApp::new();
    let (status, _, token) = app.get_with_csrf("/home").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(token.len(), csrf::CSRF_TOKEN_BYTES * 2);
}

#[tokio::test]
async fn unsafe_request_without_csrf_is_forbidden()
{
    let mut app = common::TestApp::new();
    let todo = common::seed_todo(&app.pool, "t");
    let req = Request::builder()
        .method(Method::DELETE)
        .uri(format!("/delete_todo/{}", todo.id))
        .body(Body::empty())
        .unwrap();
    let (status, _) = app.request(req).await;

    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn unsafe_request_with_mismatched_csrf_is_forbidden()
{
    let mut app = common::TestApp::new();
    let (_, _, token) = app.get_with_csrf("/home").await;
    let req = Request::builder()
        .method(Method::POST)
        .uri("/add_todo")
        .header(axum::http::header::COOKIE, format!("{}={token}", listem::csrf::CSRF_COOKIE))
        .header(listem::csrf::CSRF_HEADER, token.repeat(2))
        .body(Body::empty())
        .unwrap();
    let (status, _) = app.request(req).await;

    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn unsafe_request_with_valid_csrf_is_allowed()
{
    let mut app = common::TestApp::new();
    let (_, _, token) = app.get_with_csrf("/home").await;
    let body = common::form(&[("title", "csrf ok"), ("description", ""), ("importance", "low")]);
    let req = app.unsafe_request(Method::POST, "/add_todo", &token, body);
    let (status, res_body) = app.request(req).await;

    assert_eq!(status, StatusCode::OK);
    assert!(String::from_utf8_lossy(&res_body).contains("csrf ok"));
}

#[tokio::test]
async fn add_todo_rejects_invalid_form()
{
    let mut app = common::TestApp::new();
    let (_, _, token) = app.get_with_csrf("/home").await;
    let body = common::form(&[("title", "   "), ("description", ""), ("importance", "low")]);
    let req = app.unsafe_request(Method::POST, "/add_todo", &token, body);
    let (status, res_body) = app.request(req).await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(String::from_utf8_lossy(&res_body).contains("title cannot be empty"));
}

#[tokio::test]
async fn toggle_todo_updates_todo()
{
    let mut app = common::TestApp::new();
    let todo = common::seed_todo(&app.pool, "toggle me");
    let (_, _, token) = app.get_with_csrf("/home").await;
    let req = app.unsafe_request(Method::PUT, &format!("/toggle_todo/{}", todo.id), &token, Body::empty());
    let (status, res_body) = app.request(req).await;

    assert_eq!(status, StatusCode::OK);
    assert!(String::from_utf8_lossy(&res_body).contains("checked"));
}

#[tokio::test]
async fn delete_todo_is_404_for_unknown_id()
{
    let mut app = common::TestApp::new();
    let (_, _, token) = app.get_with_csrf("/home").await;
    let req = app.unsafe_request(Method::DELETE, "/delete_todo/999", &token, Body::empty());
    let (status, _) = app.request(req).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn edit_todo_updates_todo()
{
    let mut app = common::TestApp::new();
    let todo = common::seed_todo(&app.pool, "before");
    let (_, _, token) = app.get_with_csrf("/home").await;
    let body = common::form(&[("id", &todo.id.to_string()), ("title", "after"), ("description", ""), ("importance", "high"), ("completed", "false")]);
    let req = app.unsafe_request(Method::POST, &format!("/edit/{}", todo.id), &token, body);
    let (status, res_body) = app.request(req).await;

    assert_eq!(status, StatusCode::OK);
    assert!(String::from_utf8_lossy(&res_body).contains("after"));
}

#[tokio::test]
async fn edit_todo_rejects_bad_importance()
{
    let mut app = common::TestApp::new();
    let todo = common::seed_todo(&app.pool, "before");
    let (_, _, token) = app.get_with_csrf("/home").await;
    let body = common::form(&[("id", &todo.id.to_string()), ("title", "after"), ("description", ""), ("importance", "urgent"), ("completed", "false")]);
    let req = app.unsafe_request(Method::POST, &format!("/edit/{}", todo.id), &token, body);
    let (status, res_body) = app.request(req).await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(String::from_utf8_lossy(&res_body).contains("importance must be low, medium or high"));
}
