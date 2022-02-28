use std::sync::Arc;

use axum::{routing::get, AddExtensionLayer, Router};
use dotenv::dotenv;

mod args;
mod config;
mod db;
mod error;
mod form;
mod handler;
mod helper;
mod html;
mod model;
mod types;

type Result<T> = std::result::Result<T, error::AppError>;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "proxy_subscriber=debug");
    }
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let cfg = config::Config::from_env().expect("读取配置失败");
    let pool = cfg
        .pg
        .create_pool(None, tokio_postgres::NoTls)
        .expect("数据库初始化失败");

    let frontend = Router::new()
        .route("/", get(handler::index))
        .route("/s/:uuid", get(handler::subscriber));
    let admin_group = Router::new()
        .route("/", get(handler::group_index))
        .route("/add", get(handler::group_add_ui).post(handler::group_add))
        .route("/del/:id", get(handler::group_del));
    let admin_node = Router::new()
        .route("/", get(handler::node_index))
        .route("/add", get(handler::node_add_ui).post(handler::node_add))
        .route(
            "/edit/:id",
            get(handler::node_edit_ui).post(handler::node_edit),
        )
        .route("/del/:id", get(handler::node_del));
    let admin = Router::new()
        .nest("/group", admin_group)
        .nest("/node", admin_node);

    let app = Router::new()
        .nest("", frontend)
        .nest("/admin", admin)
        .layer(AddExtensionLayer::new(Arc::new(model::AppState { pool })));

    tracing::info!("Web服务运行于：http://{}", &cfg.web.addr);
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
