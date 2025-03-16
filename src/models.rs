use diesel::prelude::*;
use serde::Deserialize;

use crate::schema::todos;

#[derive(Queryable, Selectable, Debug, Deserialize, Clone)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo
{
    pub id:          i32,
    pub title:       String,
    pub description: String,
    pub importance:  String,
    pub completed:   bool,
    pub created_at:  chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = todos)]
pub struct NewTodo
{
    pub title:       String,
    pub description: String,
    pub importance:  String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TodoForm
{
    pub id:          i32,
    pub title:       String,
    pub description: String,
    pub importance:  String,
    pub completed:   bool,
}
