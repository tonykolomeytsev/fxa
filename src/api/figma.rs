use crate::common::fileutils::{create_temp_dir, FileUtilsError, TEMP_DIR_PATH};
use crate::models::config::ImageFormat;
use crate::models::figma::Document;
use reqwest::{
    blocking::{Client, Response},
    Error, StatusCode,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::BufReader;
use std::{collections::HashMap, fmt};

#[derive(Debug, Deserialize, Serialize)]
struct FigmaGetFileResponse {
    document: Document,
}

#[derive(Debug, Deserialize)]
struct FigmaGetImageResponse {
    images: HashMap<String, String>,
}

pub struct FigmaApi {
    client: Client,
}

#[derive(Debug)]
pub struct FigmaApiError {
    pub message: String,
    pub cause: String,
}

impl fmt::Display for FigmaApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\nCaused by: {}", &self.message, &self.cause)
    }
}

trait IntoFigmaApiError {
    fn map(self) -> FigmaApiError;
}

impl ImageFormat {
    fn as_download_extension(&self) -> String {
        match self {
            ImageFormat::Png => ".png".to_string(),
            ImageFormat::Svg => ".svg".to_string(),
            ImageFormat::Webp => ".png".to_string(),
        }
    }
}

pub const FIGMA_FILES_ENDPOINT: &str = "https://api.figma.com/v1/files/";
pub const FIGMA_IMAGES_ENDPOINT: &str = "https://api.figma.com/v1/images/";

impl FigmaApi {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn get_document(&self, file_id: &String) -> Result<Document, FigmaApiError> {
        load_from_cache::<FigmaGetFileResponse>(&file_id)
            .map(|response| response.document)
            .or_else(|_| {
                let url = format!("{}{}", FIGMA_FILES_ENDPOINT, &file_id);
                let response = self.client.get(&url).send();
                match_response_internal(response, &url, |response| {
                    match response.json::<FigmaGetFileResponse>() {
                        Ok(response) => {
                            save_to_cache(&response, &file_id).unwrap_or_default();
                            Ok(response.document)
                        }
                        Err(e) => {
                            let message = format!("while parsing json response from {}", &url);
                            let cause = format!("{}", e);
                            Err(FigmaApiError { message, cause })
                        }
                    }
                })
            })
    }

    pub fn get_image_download_url(
        &self,
        file_id: &String,
        node_id: &String,
        scale: f32,
    ) -> Result<String, FigmaApiError> {
        let url = format!("{}{}", FIGMA_IMAGES_ENDPOINT, &file_id);
        let response = self
            .client
            .get(&url)
            .query(&[("ids", node_id.clone())])
            .query(&[("scale", scale)])
            .query(&[("format", "png")])
            .send();
        match_response_internal(response, &url, |response| {
            match response.json::<FigmaGetImageResponse>() {
                Ok(response) => Ok(response.images.get(node_id).unwrap().clone()), // todo: unwrap safe
                Err(e) => {
                    let message = format!("while parsing json response from {}", &url);
                    let recomendation = "Check your VPN settings and make sure the \
                    address is reachable through your network"
                        .to_string();
                    let cause = format!("{}\n{}", e, recomendation);
                    Err(FigmaApiError { message, cause })
                }
            }
        })
    }

    pub fn get_image(
        &self,
        image_url: &String,
        image_name: &String,
        image_scale_name: &String,
        image_format: &ImageFormat,
    ) -> Result<String, FigmaApiError> {
        let response = self.client.get(image_url).send();
        match_response_internal(response, &image_url, |response| {
            response
                .bytes()
                .map_err(|e| {
                    let message = format!("while getting bytes of response: {}", &image_url);
                    let cause = format!("{}", e);
                    FigmaApiError { message, cause }
                })
                .and_then(|bytes| create_temp_dir().map_err(|e| e.map()).map(|()| bytes))
                .and_then(|bytes| {
                    let image_file_name = format!(
                        "{}/{}_{}{}",
                        TEMP_DIR_PATH,
                        &image_name,
                        &image_scale_name,
                        image_format.as_download_extension(),
                    );
                    fs::write(&image_file_name, bytes)
                        .map_err(|e| {
                            let message = "while writing to temp image file".to_string();
                            let cause = format!("{}", e);
                            FigmaApiError { message, cause }
                        })
                        .map(|()| image_file_name)
                })
        })
    }
}

fn match_response_internal<T, F>(
    response: Result<Response, Error>,
    url: &String,
    on_success: F,
) -> Result<T, FigmaApiError>
where
    F: Fn(Response) -> Result<T, FigmaApiError>,
{
    match response {
        Ok(response) => match response.status() {
            StatusCode::OK => on_success(response),
            _ => {
                let message = format!("while requesting {}", &url);
                let cause = format!("HTTP status code is {}", response.status());
                Err(FigmaApiError { message, cause })
            }
        },
        Err(e) => {
            let message = format!("while requesting {}", &url);
            let recomendation = "Check your VPN settings and make sure the \
            address is reachable through your network"
                .to_string();
            let cause = format!("{}\n{}", e, recomendation);
            Err(FigmaApiError { message, cause })
        }
    }
}

fn load_from_cache<T: DeserializeOwned>(id: &String) -> Result<T, FigmaApiError> {
    let file_name = format!(".fxn/cache_{}.json", &id);
    File::open(&file_name)
        .map_err(|e| e.map())
        .map(|file| BufReader::new(file))
        .map(|it| serde_json::from_reader(it).unwrap())
}

fn save_to_cache<T: Serialize>(value: T, id: &String) -> Result<(), FigmaApiError> {
    let file_name = format!(".fxn/cache_{}.json", &id);
    match create_temp_dir() {
        Ok(()) => match File::create(&file_name).map(|it| serde_json::to_writer(&it, &value)) {
            Ok(_) => Ok(()),
            Err(e) => Err(FigmaApiError {
                message: format!("while wrong writing in cache file {}", &file_name),
                cause: format!("{}", &e),
            }),
        },
        Err(e) => Err(e.map()),
    }
}

impl IntoFigmaApiError for FileUtilsError {
    fn map(self) -> FigmaApiError {
        FigmaApiError {
            message: self.message,
            cause: self.cause,
        }
    }
}

impl IntoFigmaApiError for std::io::Error {
    fn map(self) -> FigmaApiError {
        FigmaApiError {
            message: format!("{}", self),
            cause: String::new(),
        }
    }
}

impl IntoFigmaApiError for serde_json::Error {
    fn map(self) -> FigmaApiError {
        FigmaApiError {
            message: format!("{}", self),
            cause: String::new(),
        }
    }
}
