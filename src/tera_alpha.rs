// use rocket::post;
// use rocket::response::status;
use rocket::Request;
use rocket::response::Redirect;

// use rocket::serde::{json::Json, Serialize};

use rocket_dyn_templates::{Template, tera::Tera, context};

// use crate::api;

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/", home()))
}

#[get("/home")]
pub fn home() -> Template {
    let tasks: Vec<i32> = vec![1, 22, 34, 45 ,5];
    Template::render("index", context! {
        title: "Home",
        tasks: tasks
    })
}

use crate::database::database::DbPool;
use crate::database::*;
use rocket::{get, State};


#[get("/tasks")]
pub fn get_tasks(pool: &State<DbPool>) -> Template {
    
    let mut conn = pool
        .get()
        .expect("Failed to get a database connection from the pool");
    
    let tasks: Vec<models::Todo> = match database::get_all_todos(&mut conn) {
        Ok(result) => result,
        Err(_) => vec![],
    };

    let context =  context! {
        tasks: &tasks
    };

    Template::render("tasks_loop", context)
}

#[get("/hello/<name>")]
pub fn hello(name: &str) -> Template {
    Template::render("index", context! {
        title: "Hello",
        name: Some(name),
        items: vec!["One", "Two", "Three"],
    })
}

#[get("/about")]
pub fn about() -> Template {
    Template::render("about.html", context! {
        title: "About",
    })
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("error/404", context! {
        uri: req.uri()
    })
}

pub fn customize(tera: &mut Tera) {
    tera.add_raw_template("about.html", r#"
        {% extends "base" %}

        {% block content %}
            <section id="about">
              <h1>About - Here's another page!</h1>
            </section>
        {% endblock content %}
    "#).expect("valid Tera template");
}
