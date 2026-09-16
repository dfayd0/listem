mod common;

use diesel::{
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};
use listem::{
    db,
    models::{
        NewTodo,
        TodoForm,
    },
};

fn setup() -> (common::TestApp, Pool<ConnectionManager<SqliteConnection>>)
{
    let app = common::TestApp::new();
    let pool = app.pool.clone();
    (app, pool)
}

fn sample(id: i32) -> TodoForm
{
    TodoForm {
        id,
        title:       "updated".to_owned(),
        description: "desc".to_owned(),
        importance:  "high".to_owned(),
        completed:   false,
    }
}

#[test]
fn create_and_get_todo()
{
    let (app, pool) = setup();
    let todo = common::seed_todo(&pool, "buy milk");

    let mut conn = pool.get().unwrap();
    assert_eq!(db::get_todo_by_id(&mut conn, todo.id).unwrap().title, "buy milk");
    assert!(app._dir.path().exists());
}

#[test]
fn get_todos_lists_all()
{
    let (app, pool) = setup();
    let _keep = &app;
    common::seed_todo(&pool, "a");
    common::seed_todo(&pool, "b");

    let mut conn = pool.get().unwrap();
    let todos = db::get_todos(&mut conn).unwrap();

    assert_eq!(todos.len(), 2);
}

#[test]
fn toggle_flips_completed()
{
    let (app, pool) = setup();
    let todo = common::seed_todo(&pool, "t");

    let mut conn = pool.get().unwrap();
    let toggled = db::toggle_todo_by_id(&mut conn, todo.id).unwrap();
    assert!(toggled.completed);

    let toggled_back = db::toggle_todo_by_id(&mut conn, todo.id).unwrap();
    assert!(!toggled_back.completed);
}

#[test]
fn edit_updates_fields()
{
    let (app, pool) = setup();
    let todo = common::seed_todo(&pool, "before");

    let mut conn = pool.get().unwrap();
    let updated = db::edit_todo(&mut conn, sample(todo.id)).unwrap();

    assert_eq!(updated.title, "updated");
    assert_eq!(updated.importance, "high");
}

#[test]
fn delete_removes_todo()
{
    let (app, pool) = setup();
    let todo = common::seed_todo(&pool, "bye");

    let mut conn = pool.get().unwrap();
    assert_eq!(db::delete_todo_by_id(&mut conn, todo.id).unwrap(), 1);
    assert!(db::get_todo_by_id(&mut conn, todo.id).is_err());
}

#[test]
fn new_todo_rejects_bad_input()
{
    let cases = [
        (
            NewTodo {
                title:       "  ".to_owned(),
                description: String::new(),
                importance:  "low".to_owned(),
            },
            false,
        ),
        (
            NewTodo {
                title:       "x".repeat(101),
                description: String::new(),
                importance:  "low".to_owned(),
            },
            false,
        ),
        (
            NewTodo {
                title:       "ok".to_owned(),
                description: "y".repeat(501),
                importance:  "low".to_owned(),
            },
            false,
        ),
        (
            NewTodo {
                title:       "ok".to_owned(),
                description: String::new(),
                importance:  "urgent".to_owned(),
            },
            false,
        ),
    ];

    for (form, expected) in cases
    {
        assert_eq!(form.validate().is_ok(), expected);
    }
}

#[test]
fn debug_probe18_copy_delete()
{
    let (app, pool) = setup();
    let todo = common::seed_todo(&pool, "bye");
    let mut conn = pool.get().unwrap();
    assert_eq!(listem::db::delete_todo_by_id(&mut conn, todo.id).unwrap(), 1);
    assert!(listem::db::get_todo_by_id(&mut conn, todo.id).is_err());
    let _ = app;
}
