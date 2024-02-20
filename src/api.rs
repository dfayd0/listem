use rocket::{
    form::Form,
    get,
    http::Status,
    post,
    response::status,
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use rocket_dyn_templates::{context, Template};
extern crate tera;
use crate::database::{database, database::*, models::Task};

#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct TaskInput
{
    title: String,
    description: Option<String>,
    due_date: Option<String>,
}

#[post("/add", data = "<task_input>")]
pub fn add_task(
    task_input: Form<TaskInput>,
    pool: &State<DbPool>,
) -> Template
{
    let mut conn = pool
        .get()
        .expect("Failed to get a database connection from the pool");

    let title = task_input.title.clone();
    let description = task_input.description.clone();
    let due_date = task_input.due_date.clone();
    let task = create_task(&mut conn, title.as_str(), description, due_date);

    Template::render(
        "task/item_unchecked",
        context! {
            task: task
        },
    )
}

#[derive(Responder)]
#[response(status = 202, content_type = "html")]
pub struct Html(String);

#[get("/count")]
pub fn count_tasks<'r, 'o>(pool: &State<DbPool>) -> Html
{
    let mut conn = pool
        .get()
        .expect("Failed to get a database connection from the pool");

    let count = database::count_tasks(&mut conn).to_string();
    Html(count)
}

#[get("/count/done")]
pub fn count_done_tasks<'r, 'o>(pool: &State<DbPool>) -> Html
{
    let mut conn = pool
        .get()
        .expect("Failed to get a database connection from the pool");

    let count = database::count_done_tasks(&mut conn, true).to_string();
    Html(count)
}

#[get("/count/undone")]
pub fn count_undone_tasks<'r, 'o>(pool: &State<DbPool>) -> Html
{
    let mut conn = pool
        .get()
        .expect("Failed to get a database connection from the pool");

    let count = database::count_done_tasks(&mut conn, false).to_string();
    Html(count)
}

#[delete("/delete/<id>")]
pub fn delete_task(
    id: i32,
    pool: &State<DbPool>,
) -> status::Accepted<String>
{
    let mut conn = pool
        .get()
        .expect("Failed to get a database connection from the pool");

    database::delete_task_by_id(&mut conn, id);
    status::Accepted(None)
}

#[put("/edit/<id>", data = "<task_input>")]
pub fn edit_task(
    id: i32,
    task_input: Form<TaskInput>,
    pool: &State<DbPool>,
) -> Template
{
    let mut conn = pool
        .get()
        .expect("Failed to get a database connection from the pool");

    let task =
        database::update_task_by_id(&mut conn, id, task_input.title.clone());
    if task.done {
        Template::render(
            "task/item_checked",
            context! {
                task: task
            },
        )
    } else {
        Template::render(
            "task/item_unchecked",
            context! {
                task: task
            },
        )
    }
}

#[get("/gedit/<id>")]
pub fn get_edit_task(
    id: i32,
    pool: &State<DbPool>,
) -> Template
{
    let mut conn = pool
        .get()
        .expect("Failed to get a database connection from the pool");

    let task = database::get_task_by_id(&mut conn, id);
    Template::render(
        "task/edit",
        context! {
            task: task
        },
    )
}

#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
pub struct CheckValidInput
{
    complete: bool,
    value: Option<String>,
}

#[put("/done/<id>", data = "<check_valid_input>")]
pub fn done_task(
    id: i32,
    check_valid_input: Form<CheckValidInput>,
    pool: &State<DbPool>,
) -> Template
{
    let mut conn = pool
        .get()
        .expect("Failed to get a database connection from the pool");

    match check_valid_input.value {
        Some(_) => {
            let task_done =
                database::update_done_task_by_id(&mut conn, id, true);
            Template::render(
                "task/item_checked",
                context! {
                    task: task_done
                },
            )
        }
        None => {
            let task_undone =
                database::update_done_task_by_id(&mut conn, id, false);
            Template::render(
                "task/item_unchecked",
                context! {
                    task: task_undone
                },
            )
        }
    }
}

#[get("/tasks")]
pub fn _get_tasks(pool: &State<DbPool>) -> Result<Json<Vec<Task>>, Status>
{
    let mut conn = pool
        .get()
        .expect("Failed to get a database connection from the pool");

    match database::get_all_tasks(&mut conn) {
        Ok(tasks) => Ok(Json(tasks)),
        Err(err) => {
            eprintln!("Error fetching tasks: {}", err);
            Err(Status::NotFound)
        }
    }
}
