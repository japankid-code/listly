use actix_web::{get, HttpResponse, Responder};

#[path = "../../data/database_connection.rs"]
mod database_connection;

#[get("/api/db_healthchecker")]
async fn db_health_check() -> impl Responder {
    let connection = database_connection::establish_connection();

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": "it connected :)"}))
}
