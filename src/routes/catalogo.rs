use actix_web::*;

pub async fn catalogo() -> HttpResponse {
    HttpResponse::Ok()
    .content_type("application/json; charset=utf-8")
    .body(r#"[
        {"id": "101", "descricao": "📘 Livro clean code"},
        {"id": "102", "descricao": "📗 Livro design patterns"},
        {"id": "103", "descricao": "📕 Livro design patterns"}
    ]"#)
}   