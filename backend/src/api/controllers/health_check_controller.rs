use actix_web::{get, HttpResponse, Responder};

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Healthy :)";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}
