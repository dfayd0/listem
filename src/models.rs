use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo
{
    pub id:          i32,
    pub title:       String,
    pub description: String,
    pub completed:   bool,
}

use crate::schema::todos;

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a>
{
    pub title:       &'a str,
    pub description: &'a str,
}
