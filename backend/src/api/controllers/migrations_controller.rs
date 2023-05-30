use crate::MySqlPool;
use actix_web::{get, web, HttpResponse, Responder};

#[path = "../../data/handlers/migrations_handlers.rs"]
mod migrations_handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/migrations").service(run_migrations));
}

#[get("run")]
async fn run_migrations(db: web::Data<MySqlPool>) -> impl Responder {
    migrations_handlers::run_all_migrations(db);
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "migrations": "done",
    }))
}
