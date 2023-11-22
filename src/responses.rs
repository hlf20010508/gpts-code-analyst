use actix_web::http::header;
use actix_web::HttpResponse;

use crate::utils::read_file;

pub fn response_text(content: String) -> HttpResponse {
    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "text/plain; charset=utf-8"))
        .body(content)
}

pub fn response_internal_server_error(content: String) -> HttpResponse {
    HttpResponse::InternalServerError()
        .insert_header((header::CONTENT_TYPE, "text/plain; charset=utf-8"))
        .body(content)
}

pub async fn response_text_from_file(path: &str) -> HttpResponse {
    match read_file(&path).await {
        Ok(content) => response_text(content),
        Err(error_info) => response_internal_server_error(error_info.to_string()),
    }
}
