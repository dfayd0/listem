#[macro_use]
extern crate rocket;
mod api;
mod database;
mod routes;

extern crate tera;

#[cfg(test)]
mod tests;

use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _
{
    rocket::build()
        .manage(database::database::init_pool())
        .mount("/", routes![routes::index, routes::about, routes::home])
        .mount("/static", rocket::fs::FileServer::from("static"))
        .mount(
            "/api",
            routes![
                api::add_task,
                api::edit_task,
                api::get_edit_task,
                api::delete_task,
                api::done_task,
                api::count_tasks,
                api::count_done_tasks,
                api::count_undone_tasks,
            ],
        )
        .register("/", catchers![routes::not_found])
        .attach(Template::custom(|engines| {
            routes::customize(&mut engines.tera);
        }))
    // .attach(Template::fairing())
}
