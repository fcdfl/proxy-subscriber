use axum::{routing::get, Router};

mod handler;
mod html;
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler::index));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
