use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
#[path = "./data/database_connection.rs"]
mod database_connection;
#[path = "./data/models/diesel_schema.rs"]
mod diesel_schema;
use database_connection::{new_pool, MySqlPool, MySqlPooledConnection};
use diesel;
#[path = "./api/controllers/db_health_check_controller.rs"]
mod db_health_check_controller;
#[path = "./api/controllers/health_check_controller.rs"]
mod health_check_controller;
#[path = "./api/controllers/user_controller.rs"]
mod user_controller;
#[path = "./data/models/user_models.rs"]
mod user_models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    let my_sql_pool: web::Data<MySqlPool> = web::Data::new(new_pool());

    HttpServer::new(move || {
        App::new()
            .service(health_check_controller::health_checker_handler)
            .service(db_health_check_controller::db_health_check)
            .app_data(my_sql_pool.clone())
            .service(web::scope("/api").configure(user_controller::config))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
