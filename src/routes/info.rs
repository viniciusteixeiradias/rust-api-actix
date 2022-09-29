use actix_web::*;

pub async fn info() -> HttpResponse {
    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(format!("🌏 Versão 0.1.0 <br>Author: Vinícius" ))
}