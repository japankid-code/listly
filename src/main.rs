use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

#[path = "./data/database_connection.rs"]
mod database_connection;
#[path = "./api/controllers/db_health_check_controller.rs"]
mod db_health_check_controller;
#[path = "./api/controllers/health_check_controller.rs"]
mod health_check_controller;
#[path = "./api/config/user_routes.rs"]
mod user_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    let mySqlPool: database_connection::MySqlPool = database_connection::new_pool();

    HttpServer::new(move || {
        App::new()
            .service(health_check_controller::health_checker_handler)
            .service(db_health_check_controller::db_health_check)
            .app_data(mySqlPool.clone())
            .service(web::scope("/api").configure(user_routes::config))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
