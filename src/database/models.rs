use diesel::prelude::*;
use rocket::serde::Serialize;
use chrono::NaiveDateTime;

use crate::database::schema::tasks;

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task
{
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub due_date: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a>
{
    pub title: &'a str,
    pub description: &'a str,
    pub created_at: NaiveDateTime,
    pub due_date: Option<NaiveDateTime>,
}
