use std::time::Duration;

use reqwest::blocking::Client;

pub fn create_http_client(token: &String) -> Client {
    let mut auth_headers = reqwest::header::HeaderMap::new();
    auth_headers.insert("X-FIGMA-TOKEN", token.parse().unwrap());
    reqwest::blocking::Client::builder()
        .timeout(Some(Duration::new(15, 0)))
        .default_headers(auth_headers)
        .build()
        .unwrap()
}
