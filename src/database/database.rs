use std::env;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;
pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

use crate::database::{
    models::{NewTask, Task},
    schema::tasks,
};

// we set a r2d2 pool for an easier access to the database variable
pub fn init_pool() -> DbPool
{
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool")
}

pub fn create_task(
    conn: &mut SqliteConnection,
    title: &str,
    description: Option<String>,
    due_date: Option<String>,
) -> Task
{
    let new_task = NewTask {
        title,
        description: description.as_deref().unwrap_or(""),
        created_at: chrono::Local::now().naive_local(),
        due_date: None,
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .returning(Task::as_returning())
        .get_result(conn)
        .expect("Error saving new task")
}

pub fn count_tasks(conn: &mut SqliteConnection) -> i64
{
    tasks::table
        .count()
        .get_result(conn)
        .expect("Error counting tasks")
}

pub fn get_task_by_id(
    conn: &mut SqliteConnection,
    id: i32,
) -> Task
{
    tasks::table
        .find(id)
        .first(conn)
        .expect("Error getting task")
}

pub fn update_task_by_id(
    conn: &mut SqliteConnection,
    id: i32,
    title: String,
) -> Task
{
    diesel::update(tasks::table.filter(tasks::id.eq(id)))
        .set(tasks::title.eq(title))
        .get_result(conn)
        .expect("Error updating task")
}

pub fn update_done_task_by_id(
    conn: &mut SqliteConnection,
    id: i32,
    done: bool,
) -> Task
{
    println!(
        "The database is updating the task with id: {} with the value of {}",
        id, done
    );
    diesel::update(tasks::table.filter(tasks::id.eq(id)))
        .set(tasks::done.eq(done))
        .get_result(conn)
        .expect("Error updating task")
}

pub fn delete_task_by_id(
    conn: &mut SqliteConnection,
    id: i32,
) -> usize
{
    diesel::delete(tasks::table.filter(tasks::id.eq(id)))
        .execute(conn)
        .expect("Error deleting task")
}

pub fn get_all_tasks(
    conn: &mut SqliteConnection
) -> Result<Vec<Task>, diesel::result::Error>
{
    tasks::table.load(conn)
}

pub fn count_done_tasks(
    conn: &mut SqliteConnection,
    done: bool,
) -> i64
{
    tasks::table
        .filter(tasks::done.eq(done))
        .count()
        .get_result(conn)
        .expect("Error counting tasks")
}
