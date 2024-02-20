use rocket::{get, response::Redirect, Request, State};
use rocket_dyn_templates::{context, tera::Tera, Template};

use crate::database::{database::DbPool, *};

#[get("/")]
pub fn index() -> Redirect
{
    Redirect::to(uri!("/", home()))
}

#[get("/home")]
pub fn home(pool: &State<DbPool>) -> Template
{
    let mut conn = pool
        .get()
        .expect("Failed to get a database connection from the pool");

    let tasks: Vec<models::Task> = match database::get_all_tasks(&mut conn) {
        Ok(result) => result,
        Err(_) => vec![],
    };

    Template::render(
        "index",
        context! {
            title: "Home",
            tasks: tasks
        },
    )
}

#[get("/about")]
pub fn about() -> Template
{
    Template::render(
        "about.html",
        context! {
            title: "About",
        },
    )
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template
{
    Template::render(
        "error/404",
        context! {
            uri: req.uri()
        },
    )
}

pub fn customize(tera: &mut Tera)
{
    tera.add_raw_template(
        "about.html",
        r#"
        {% extends "fragments/base" %}

        {% block content %}
            <section id="about">
              <h1>About - Here's another page!</h1>
            </section>
        {% endblock content %}
    "#,
    )
    .expect("valid Tera template");
}
