use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
#[path = "./data/database_connection.rs"]
mod database_connection;
#[path = "./data/models/diesel_schema.rs"]
mod diesel_schema;
use database_connection::{new_pool, MySqlPool, MySqlPooledConnection};
use diesel;
#[path = "./api/controllers/health_check_controller.rs"]
mod health_check_controller;
#[path = "./api/controllers/migrations_controller.rs"]
mod migrations_controller;
#[path = "./api/controllers/user_controller.rs"]
mod user_controller;
#[path = "./data/models/user_models.rs"]
mod user_models;

#[macro_use]
extern crate diesel_migrations;
use diesel_migrations::EmbeddedMigrations;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./src/data/migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }
    env_logger::init();

    dotenv().ok().expect("no .env file found");

    let server_host_address =
        env::var("SERVER_HOST_ADDRESS").expect("SERVER_HOST_ADDRESS must be set");

    println!("🚀 Server started successfully");

    let my_sql_pool: web::Data<MySqlPool> = web::Data::new(new_pool());

    println!("Database pool created");

    HttpServer::new(move || {
        App::new()
            .service(health_check_controller::health_checker_handler)
            .app_data(my_sql_pool.clone())
            .service(
                web::scope("/api")
                    .configure(user_controller::config)
                    .configure(migrations_controller::config),
            )
            .wrap(Logger::default())
    })
    .bind((server_host_address, 8000))?
    .run()
    .await
}
