use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(
                web::resource("")
                    .route(web::get().to(|| async { HttpResponse::Ok().body("users") }))
                    .route(web::post().to(|| async { HttpResponse::Ok().body("user created") })),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(|| async { HttpResponse::Ok().body("user") }))
                    .route(web::delete().to(|| async { HttpResponse::Ok().body("user deleted") })),
            ),
    );
}
