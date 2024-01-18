/*
:project: gpts-code-analyst
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use actix_web::{web, HttpResponse};
use std::error::Error;
use tokio::fs::read_to_string;

use crate::structures::{DetailsParams, StructureParams};
use crate::utils::{get_file_details, get_repo_structure};

pub async fn structure_handler(
    params: web::Query<StructureParams>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let content = get_repo_structure(&params.user, &params.repo).await?;
    return Ok(HttpResponse::Ok().body(content));
}

pub async fn details_handler(
    params: web::Query<DetailsParams>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let path_list = params.path_list.split(", ").collect::<Vec<&str>>();
    let mut result = String::new();
    for path in path_list {
        result = format!(
            "{}\n\n{}:\n{}",
            result,
            path,
            get_file_details(&params.user, &params.repo, &path).await?
        );
    }
    return Ok(HttpResponse::Ok().body(result));
}

pub async fn privacy_handler() -> Result<HttpResponse, Box<dyn Error>> {
    let content = read_to_string("static/privacy.txt").await?;
    return Ok(HttpResponse::Ok().body(content));
}

pub async fn schema_handler() -> Result<HttpResponse, Box<dyn Error>> {
    let content = read_to_string("static/schema.json").await?;
    return Ok(HttpResponse::Ok().body(content));
}
