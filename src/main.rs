use axum::Router;
use std::env::var;

use api::build_todo_routes;
use db::Db;

mod api;
mod db;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let port: String = var("PORT").expect("PORT is not set");
    let db = Db::init().await;

    let app = Router::new()
        .nest("/todo", build_todo_routes())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    println!("server listening on port {port}");
    axum::serve(listener, app).await.unwrap();
}
