use async_std::io::{Error, ErrorKind, Result};

pub fn path_error(error_kind: ErrorKind, path: &str, error_info: Error) -> Result<String> {
    return Err(Error::new(
        error_kind,
        format!("path: {}\nerror: {}", path, error_info.to_string()),
    ));
}

pub fn url_get_error(error_kind: ErrorKind, url: &str, error_info: Error) -> Result<String> {
    return Err(Error::new(
        error_kind,
        format!("url: {}\nerror: {}", url, error_info.to_string()),
    ));
}

pub fn data_parse_error(
    error_kind: ErrorKind,
    url: &str,
    error_info: serde_json::Error,
) -> Result<String> {
    return Err(Error::new(
        error_kind,
        format!("url: {}\nerror: {}", url, error_info.to_string()),
    ));
}
