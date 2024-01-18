use actix_web::http::header;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{web, App, HttpServer};
use env_logger::Env;

mod handlers;
mod structures;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const HOST: &str = "0.0.0.0";
    const PORT: u16 = 8080;
    println!("Running on http://{}:{}", &HOST, &PORT);

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    return HttpServer::new(|| {
        App::new()
            .route("/structure", web::get().to(handlers::structure_handler))
            .route("/details", web::get().to(handlers::details_handler))
            .route("/privacy", web::get().to(handlers::privacy_handler))
            .route("/schema", web::get().to(handlers::schema_handler))
            .wrap(DefaultHeaders::new().add((header::CONTENT_TYPE, "text/plain; charset=utf-8")))
            .wrap(Logger::default())
    })
    .bind((HOST, PORT))?
    .run()
    .await;
}
