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

impl NewTodo
{
    /// Returns Err with a message if the form data is invalid.
    pub fn validate(&self) -> Result<(), String>
    {
        let title = self.title.trim();
        if title.is_empty()
        {
            return Err("title cannot be empty".into());
        }
        if title.len() > 100
        {
            return Err("title must be 100 characters or less".into());
        }
        if self.description.len() > 500
        {
            return Err("description must be 500 characters or less".into());
        }
        match self.importance.as_str()
        {
            "low" | "medium" | "high" => Ok(()),
            _ => Err("importance must be low, medium or high".into()),
        }
    }
}

impl TodoForm
{
    /// Returns Err with a message if the form data is invalid.
    pub fn validate(&self) -> Result<(), String>
    {
        let title = self.title.trim();
        if title.is_empty()
        {
            return Err("title cannot be empty".into());
        }
        if title.len() > 100
        {
            return Err("title must be 100 characters or less".into());
        }
        if self.description.len() > 500
        {
            return Err("description must be 500 characters or less".into());
        }
        match self.importance.as_str()
        {
            "low" | "medium" | "high" => Ok(()),
            _ => Err("importance must be low, medium or high".into()),
        }
    }
}
