use axum::{routing::get, Router};

use crate::db::Db;

pub mod todo;

pub fn build_todo_routes() -> Router<Db> {
    Router::new().route("/", get(todo::get_todo).post(todo::create_todo))
}
