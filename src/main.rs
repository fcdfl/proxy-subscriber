use axum::{routing::get, Router};

mod db;
mod error;
mod form;
mod handler;
mod html;
mod types;

type Result<T> = std::result::Result<T, error::AppError>;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "proxy_subscriber=debug");
    }
    tracing_subscriber::fmt::init();

    let admin = Router::new()
        .route("/", get(handler::index))
        .route(
            "/group/add",
            get(handler::group_add_ui).post(handler::group_add),
        )
        .route(
            "/node/add",
            get(handler::node_add_ui).post(handler::node_add),
        );

    let app = Router::new().nest("/admin", admin);

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
