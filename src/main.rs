use axum::{
    routing::{get, post},
    Extension, Router,
};
use blog::{
    config::Config,
    handler::{admin, front, login::login, register::register, topic::topic},
    AppState,
};
use deadpool_postgres::Runtime;
use redis::Client;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cfg = Config::new().expect("Config init failed");
    let pool = cfg
        .pg
        .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .expect("DB pool creation failed");
    let rdc = Client::open(cfg.redis.dsn).expect("Redis client creation failed");
    let front_router = front::router();
    let admin_router = admin::router();
    let app = Router::new()
        .nest("/user/:user", front_router)
        .nest("/admin/:user", admin_router)
        .route("/topic/:id", get(topic))
        .route("/register", post(register))
        .route("/login", post(login))
        .layer(Extension(Arc::new(AppState { pool, rdc })));
    tracing::info!("Server start: {}", &cfg.web.addr);
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
