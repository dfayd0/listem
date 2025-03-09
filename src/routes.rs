use axum::response::{Html, Response};

pub async fn home() -> Html<&'static str>
{
    Html("<p>Hello, World!</p>")
}
