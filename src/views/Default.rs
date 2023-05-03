use actix_web::{HttpResponse, Responder, http::StatusCode};

pub async fn not_found() -> impl Responder {
    HttpResponse::Ok()
        .status(StatusCode::NOT_FOUND)
        .body("PAGE NOT FOUND")
}