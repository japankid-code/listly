use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
#[path = "./api/controllers/health_check.rs"] mod health_check;
#[path = "./api/controllers/db_health_check.rs"] mod db_health_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .service(health_check::health_checker_handler)
            .service(db_health_check::db_health_checker_handler)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}