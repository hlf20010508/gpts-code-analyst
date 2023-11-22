use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder};
use env_logger::Env;

mod errors;
mod requests;
mod responses;
mod structures;
mod utils;

use responses::{response_internal_server_error, response_text, response_text_from_file};
use structures::{DetailsParams, StructureParams};
use utils::{get_file_details, get_repo_structure};

#[get("/structure")]
async fn structure(params: web::Query<StructureParams>) -> impl Responder {
    return match get_repo_structure(&params.user, &params.repo).await {
        Ok(repo_structure) => response_text(repo_structure),
        Err(err) => response_internal_server_error(err.to_string()),
    };
}

#[get("/details")]
async fn details(params: web::Query<DetailsParams>) -> impl Responder {
    let path_list = params.path_list.split(", ").collect::<Vec<&str>>();
    let mut result = String::new();
    for path in path_list {
        result = format!(
            "{}\n\n{}:\n{}",
            result,
            path,
            match get_file_details(&params.user, &params.repo, &path).await {
                Ok(details) => details,
                Err(error_info) => return response_internal_server_error(error_info.to_string()),
            }
        );
    }
    return response_text(result);
}

#[get("/privacy")]
async fn privacy() -> impl Responder {
    return response_text_from_file("static/privacy.txt").await;
}

#[get("/schema")]
async fn schema() -> impl Responder {
    return response_text_from_file("static/schema.json").await;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const HOST: &str = "0.0.0.0";
    const PORT: u16 = 8080;
    println!("Running on http://{}:{}", &HOST, &PORT);

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    return HttpServer::new(|| {
        App::new()
            .service(structure)
            .service(details)
            .service(privacy)
            .service(schema)
            .wrap(Logger::default())
    })
    .bind((HOST, PORT))?
    .run()
    .await;
}
