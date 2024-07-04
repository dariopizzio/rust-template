use bootstrap::get_app_state;
use config::Config;
use database::get_connection_pool;
use routes::init_router;

mod api_responses;
mod bootstrap;
mod config;
mod controllers;
mod database;
mod health;
mod models;
mod repositories;
mod routes;
mod schema;
mod services;

#[tokio::main]
async fn main() {
    let config = Config::init();

    let db_pool = get_connection_pool(&config);
    let app_state = get_app_state(db_pool);

    let router = init_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
