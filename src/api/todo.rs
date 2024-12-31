use crate::db::Db;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Todo {
    id: i32,
    title: String,
    done: i8,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

pub struct ServerError(anyhow::Error);

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for ServerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub async fn get_todo(State(db): State<Db>) -> Result<Json<Vec<Todo>>, ServerError> {
    let mysql_pool = db.mysql_pool;

    let todos = sqlx::query_as!(Todo, "SELECT * FROM Todo")
        .fetch_all(&mysql_pool)
        .await?;

    Ok(Json(todos))
}

pub async fn create_todo(
    State(db): State<Db>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, ServerError> {
    let title = payload.title;
    let mysql_pool = db.mysql_pool;

    let result = sqlx::query!("INSERT INTO Todo (title, done) VALUES (?, ?)", title, false)
        .execute(&mysql_pool)
        .await?;

    let todo = Todo {
        id: result.last_insert_id() as i32,
        title,
        done: 0,
    };

    Ok(Json(todo))
}
