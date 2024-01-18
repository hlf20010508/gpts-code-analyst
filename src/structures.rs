/*
:project: gpts-code-analyst
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use serde::Deserialize;

#[derive(Deserialize)]
pub struct StructureParams {
    pub user: String,
    pub repo: String,
}

#[derive(Deserialize)]
pub struct DetailsParams {
    pub user: String,
    pub repo: String,
    pub path_list: String,
}

#[derive(Deserialize)]
pub struct RepoInfo {
    pub default_branch: String,
}

#[derive(Deserialize)]
pub struct TreeItem {
    pub path: String,
}

#[derive(Deserialize)]
pub struct PathTree {
    pub tree: Vec<TreeItem>,
}

#[derive(Deserialize)]
pub struct FileInfo {
    pub download_url: String,
}
