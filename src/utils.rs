use async_std::fs::File;
use async_std::io::{ErrorKind, ReadExt, Result};

use crate::errors::{data_parse_error, path_error, url_get_error};
use crate::requests;
use crate::structures::{FileInfo, PathTree, RepoInfo};

pub async fn read_file(path: &str) -> Result<String> {
    let mut file = match File::open(path).await {
        Ok(file) => file,
        Err(error_info) => return path_error(ErrorKind::NotFound, path, error_info),
    };
    let mut contents = String::new();

    match file.read_to_string(&mut contents).await {
        Ok(_) => (),
        Err(error_info) => return path_error(ErrorKind::NotFound, path, error_info),
    };

    return Ok(contents);
}

pub async fn get_repo_structure(user: &str, repo: &str) -> Result<String> {
    let mut url = String::from(format!("https://api.github.com/repos/{user}/{repo}"));
    let mut response = match requests::get(&url).await {
        Ok(res) => res,
        Err(error_info) => return url_get_error(ErrorKind::NotFound, &url, error_info),
    };

    let repo_info: RepoInfo = match serde_json::from_str(&response) {
        Ok(info) => info,
        Err(error_info) => return data_parse_error(ErrorKind::InvalidData, &url, error_info),
    };
    let branch = repo_info.default_branch;

    url = String::from(format!(
        "https://api.github.com/repos/{user}/{repo}/git/trees/{branch}?recursive=1"
    ));
    response = match requests::get(&url).await {
        Ok(res) => res,
        Err(error_info) => return url_get_error(ErrorKind::NotFound, &url, error_info),
    };
    let path_tree: PathTree = match serde_json::from_str(&response) {
        Ok(tree) => tree,
        Err(error_info) => return data_parse_error(ErrorKind::InvalidData, &url, error_info),
    };
    let repo_structure = path_tree
        .tree
        .into_iter()
        .map(|item| item.path)
        .collect::<Vec<_>>()
        .join("\n");

    return Ok(repo_structure);
}

pub async fn get_file_details(user: &str, repo: &str, path: &str) -> Result<String> {
    let mut url = String::from(format!(
        "https://api.github.com/repos/{user}/{repo}/contents/{path}"
    ));
    let response = match requests::get(&url).await {
        Ok(res) => res,
        Err(error_info) => return url_get_error(ErrorKind::NotFound, &url, error_info),
    };
    let file_info: FileInfo = match serde_json::from_str(&response) {
        Ok(info) => info,
        Err(error_info) => return data_parse_error(ErrorKind::InvalidData, &url, error_info),
    };

    url = file_info.download_url;
    let file_details = match requests::get(&url).await {
        Ok(res) => res,
        Err(error_info) => return url_get_error(ErrorKind::NotFound, &url, error_info),
    };

    return Ok(file_details);
}
