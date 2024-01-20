/*
:project: gpts-code-analyst
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use anyhow::Context;
use std::error::Error;

use crate::env::GITHUB_TOKEN;
use crate::structures::{FileInfo, PathTree, RepoInfo};

macro_rules! error_location {
    ($var:expr, $context:expr) => {
        format!(
            "Error in {} line {} {}:\n{}",
            file!(),
            line!(),
            $var,
            $context,
        )
    };
}

pub fn create_client() -> Result<reqwest::Client, Box<dyn Error>> {
    let github_token = GITHUB_TOKEN.as_ref().context(error_location!(
        "create_client.github_token",
        "Need CMD arg --token."
    ))?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", github_token)
            .parse()
            .context(error_location!(
                "create_client.headers",
                "Parse Athorization error."
            ))?,
    );
    headers.insert(
        reqwest::header::USER_AGENT,
        "reqwest".parse().context(error_location!(
            "create_client.headers",
            "Parse User-Agent error."
        ))?,
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .context(error_location!(
            "create_client.build",
            "Build client error."
        ))?;

    return Ok(client);
}

pub async fn get_repo_structure(user: &str, repo: &str) -> Result<String, Box<dyn Error>> {
    let client = create_client()?;

    let url = String::from(format!("https://api.github.com/repos/{user}/{repo}"));
    let response = client.get(&url).send().await?;
    let content = response.text().await?;

    let repo_info: RepoInfo = serde_json::from_str(&content).context(content)?;
    let branch = repo_info.default_branch;

    let url = String::from(format!(
        "https://api.github.com/repos/{user}/{repo}/git/trees/{branch}?recursive=1"
    ));
    let response = client.get(&url).send().await?;
    let content = response.text().await?;

    let path_tree: PathTree = serde_json::from_str(&content).context(content)?;
    let repo_structure = path_tree
        .tree
        .into_iter()
        .map(|item| item.path)
        .collect::<Vec<_>>()
        .join("\n");

    return Ok(repo_structure);
}

pub async fn get_file_details(
    user: &str,
    repo: &str,
    path: &str,
) -> Result<String, Box<dyn Error>> {
    let client = create_client()?;

    let url = String::from(format!(
        "https://api.github.com/repos/{user}/{repo}/contents/{path}"
    ));
    let response = client.get(&url).send().await?;
    let content = response.text().await?;

    let file_info: FileInfo = serde_json::from_str(&content).context(content)?;
    let url = file_info.download_url;
    let file_details = client.get(&url).send().await?;
    let content = file_details.text().await?;

    return Ok(content);
}
