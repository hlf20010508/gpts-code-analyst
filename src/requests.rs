use async_std::io::{Error, ErrorKind, Result};
use pico_args::Arguments;

pub async fn get(url: &str) -> Result<String> {
    let client = awc::Client::builder()
        .connector(awc::Connector::new())
        .finish();

    let mut request = client.get(url).insert_header(("User-Agent", "Actix-web"));

    let mut args = Arguments::from_env();
    if let Ok(github_token) = args.value_from_str::<&str, String>("--token") {
        request = request.insert_header(("Authorization", format!("Bearer {}", github_token)));
    } else {
        return Err(Error::new(
            ErrorKind::NotFound,
            "GITHUB_TOKEN not found in system env",
        ));
    }

    let mut response = match request.send().await {
        Ok(res) => res,
        Err(error_info) => {
            return Err(Error::new(
                ErrorKind::ConnectionRefused,
                error_info.to_string(),
            ))
        }
    };

    let response_body = match response.body().await {
        Ok(body) => body,
        Err(error_info) => return Err(Error::new(ErrorKind::NotFound, error_info.to_string())),
    };

    let body_content = match String::from_utf8(response_body.to_vec()) {
        Ok(content) => content,
        Err(error_info) => return Err(Error::new(ErrorKind::InvalidData, error_info.to_string())),
    };

    return Ok(body_content);
}
