#[macro_use]
extern crate rocket;
mod api;
mod database;
mod tera_alpha;

extern crate tera;
use tera::Tera;

#[cfg(test)]
mod tests;

use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    // let tera: Tera = Tera::new("templates/**/*").expect("Tera initialization failed");
    // println!("{:?}", tera);
    
    rocket::build()
        .manage(database::database::init_pool())
        // .manage(tera)
        .mount(
            "/",
            routes![tera_alpha::index, tera_alpha::hello, tera_alpha::about, tera_alpha::home, tera_alpha::get_tasks],
        )
        .mount("/static", rocket::fs::FileServer::from("static"))
        .mount("/api", routes![api::add_task, api::get_todos])
        .register("/", catchers![tera_alpha::not_found])
        .attach(Template::custom(|engines| {
            tera_alpha::customize(&mut engines.tera);
        }))
}
