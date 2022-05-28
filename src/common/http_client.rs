use std::time::Duration;

use reqwest::blocking::Client;

/// Create blocking http client to make requests to Figma API.
///
/// `X-FIGMA-TOKEN` header with a personal access token
/// will be added to each request through this client.
///
/// # Arguments
///
/// * `token` - Figma personal access token. More details: https://www.figma.com/developers/api#authentication
pub fn create_http_client(token: &String) -> Client {
    let mut auth_headers = reqwest::header::HeaderMap::new();
    auth_headers.insert("X-FIGMA-TOKEN", token.parse().unwrap());
    reqwest::blocking::Client::builder()
        .timeout(Some(Duration::new(15, 0)))
        .default_headers(auth_headers)
        .build()
        .unwrap()
}
