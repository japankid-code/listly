use crate::database_connection::MySqlPool;
use crate::user_models::NewUser;
use actix_web::{get, post, web, HttpResponse, Responder};
// #[path = "../errors.rs"]
// mod errors;
#[path = "../../data/handlers/user_handlers.rs"]
mod user_handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_users)
            .service(get_user)
            .service(add_user),
    );
}

#[get("")]
pub async fn get_users(db: web::Data<MySqlPool>) -> impl Responder {
    let _users = web::block(move || user_handlers::get_all_users(db))
        .await
        .map(|user| serde_json::json!(user))
        .map_err(|_| HttpResponse::InternalServerError());

    if _users.is_ok() {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "users": _users.ok(),
        }))
    } else {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error"
        }))
    }
}

#[get("/{user_id}")]
pub async fn get_user(db: web::Data<MySqlPool>, path: web::Path<String>) -> impl Responder {
    let user_id: String = path.into_inner();
    let _user = web::block(move || user_handlers::get_user_by_id(db, user_id))
        .await
        .map(|user| serde_json::json!(user))
        .map_err(|_| HttpResponse::InternalServerError());

    if _user.is_ok() {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "user": _user.ok(),
        }))
    } else {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error"
        }))
    }
}

#[post("")]
pub async fn add_user(db: web::Data<MySqlPool>, item: web::Json<NewUser>) -> impl Responder {
    let _user = web::block(move || user_handlers::add_single_user(db, item))
        .await
        .map(|user| serde_json::json!(user))
        .map_err(|_| HttpResponse::InternalServerError());

    if _user.is_ok() {
        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "user": _user.ok(),
        }))
    } else {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error"
        }))
    }
}
