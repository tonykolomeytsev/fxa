use crate::common::error::AppError;
use crate::common::fileutils::{create_temp_dir, TEMP_DIR_PATH};
use crate::models::config::ImageFormat;
use crate::models::figma::Document;
use reqwest::{
    blocking::{Client, Response},
    Error, StatusCode,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufReader;

/// Response from Figma API.
///
/// Endpoint: `https://api.figma.com/v1/files/:file_key`
///
/// Details: https://www.figma.com/developers/api#get-files-endpoint
#[derive(Debug, Deserialize, Serialize)]
struct FigmaGetFileResponse {
    document: Document,
}

/// Response from Figma API.
///
/// Endpoint: `https://api.figma.com/v1/images/:file_key`
///
/// Details: https://www.figma.com/developers/api#get-images-endpoint
#[derive(Debug, Deserialize)]
struct FigmaGetImageResponse {
    images: HashMap<String, String>,
}

/// An `FigmaApi` to make requests to Figma API endpoints.
///
/// Use `FigmaApi::new(client)` to build new instance.
///
/// # Example
///
/// ```rust
/// let api = FigmaApi::new(create_http_client(&figma_personal_access_token));
/// let document = api.get_document(&file_id).unwrap();
/// println!("{}", document);
/// ```
pub struct FigmaApi {
    client: Client,
}

impl ImageFormat {
    fn download_extension(&self) -> String {
        match self {
            ImageFormat::Png => "png".to_string(),
            ImageFormat::Svg => "svg".to_string(),
            // We've returned PNG format because we will convert to WEBP manually
            ImageFormat::Webp => "png".to_string(),
        }
    }
}

pub const FIGMA_FILES_ENDPOINT: &str = "https://api.figma.com/v1/files/";
pub const FIGMA_IMAGES_ENDPOINT: &str = "https://api.figma.com/v1/images/";

impl FigmaApi {
    /// Create new `FigmaApi` instance to make requests to Figma API endpoints.
    ///
    /// # Arguments
    ///
    /// * `client` - An instance of [reqwest::blocking::Client] to make requests with.
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Load Figma [Document] from the cache if the cache is not empty. Otherwise,
    /// load the document from the remote. Returns a tuple ([Document], bool), where
    /// bool value is true, if the document have loaded from cache.
    ///
    /// Endpoint: `https://api.figma.com/v1/files/:file_key`
    ///
    /// # Arguments
    ///
    /// * `file_id` - Figma file identifier. To obtain a file id, open the file in the browser.
    /// The file id will be present in the URL after the word file and before the file name.
    pub fn get_document(&self, file_id: &String) -> Result<(Document, bool), AppError> {
        load_from_cache::<FigmaGetFileResponse>(&file_id)
            .map(|response| (response.document, true))
            .or_else(|_| {
                let url = format!("{}{}", FIGMA_FILES_ENDPOINT, &file_id);
                let response = self.client.get(&url).send();
                match_response_internal(response, &url, |response| {
                    match response.json::<FigmaGetFileResponse>() {
                        Ok(response) => {
                            save_to_cache(&response, &file_id).unwrap_or_default();
                            Ok((response.document, false))
                        }
                        Err(_) => Err(AppError::FetchDomResponseParsing(url.clone())),
                    }
                })
            })
    }

    /// Get url of exported Figma frame to download.
    ///
    /// Endpoint: `https://api.figma.com/v1/images/:file_key`
    ///
    /// # Arguments
    ///
    /// * `file_id` - Figma file identifier. To obtain a file id, open the file in the browser.
    /// The file id will be present in the URL after the word file and before the file name.
    /// * `node_id` - node identifier inside Figma file. You can obtain node ids from [Document].
    /// Learn more about nodes: https://www.figma.com/developers/api#files
    /// * `scale` - The scale of the exported image, from 0.5 to 4.
    /// * `format` - Format of the exported image. Figma API supports only JPEG, PNG, SVG and
    /// PDF formats.
    pub fn get_image_download_url(
        &self,
        file_id: &String,
        node_id: &String,
        scale: f32,
        format: &ImageFormat,
    ) -> Result<String, AppError> {
        let url = format!("{}{}", FIGMA_IMAGES_ENDPOINT, &file_id);
        let response = self
            .client
            .get(&url)
            .query(&[("ids", node_id.clone())])
            .query(&[("scale", scale)])
            .query(&[("format", format.download_extension())])
            .send();
        match_response_internal(response, &url, |response| {
            match response.json::<FigmaGetImageResponse>() {
                Ok(response) => Ok(response.images.get(node_id).unwrap().clone()), // todo: unwrap safe
                Err(_) => Err(AppError::GetImageDownloadUrl(url.clone())),
            }
        })
    }

    /// Download an image from remote.
    ///
    /// # Arguments
    ///
    /// * `image_url` - Url to download image.
    /// * `image_name` - The name with which the image will be saved to the temporary directory.
    /// * `image_scale_name` - _Optional_. Will be used as image name suffix.
    /// * `image_format` - Format with which the image will be saved to the temporary directory.
    pub fn get_image(
        &self,
        image_url: &String,
        image_name: &String,
        image_scale_name: &String,
        image_format: &ImageFormat,
    ) -> Result<String, AppError> {
        let response = self.client.get(image_url).send();
        match_response_internal(response, &image_url, |response| {
            let bytes = response.bytes().map_err(|_| AppError::GetImageByteStream)?;
            create_temp_dir().map_err(|_| AppError::CreateTempDir)?;
            let image_file_name = format!(
                "{}/{}_{}.{}",
                TEMP_DIR_PATH,
                &image_name,
                &image_scale_name,
                image_format.download_extension(),
            );
            fs::write(&image_file_name, bytes)
                .map_err(|_| AppError::GetImageTemporarySave)
                .map(|_| image_file_name)
        })
    }
}

fn match_response_internal<T, F>(
    response: Result<Response, Error>,
    url: &String,
    on_success: F,
) -> Result<T, AppError>
where
    F: FnOnce(Response) -> Result<T, AppError>,
{
    match response {
        Ok(response) => match response.status() {
            StatusCode::OK => on_success(response),
            StatusCode::FORBIDDEN => Err(AppError::RequestUnauthorized(response.status())),
            _ => Err(AppError::RequestHttpStatus(url.clone(), response.status())),
        },
        Err(_) => Err(AppError::RequestMaybeVPN(url.clone())),
    }
}

fn load_from_cache<T: DeserializeOwned>(id: &String) -> Result<T, AppError> {
    let file_name = format!("{}/cache_{}.json", TEMP_DIR_PATH, &id);
    File::open(&file_name)
        .map_err(|_| AppError::LoadFromCache)
        .map(|file| BufReader::new(file))
        .map(|it| serde_json::from_reader(it).unwrap())
}

fn save_to_cache<T: Serialize>(value: T, id: &String) -> Result<(), AppError> {
    let file_name = format!("{}/cache_{}.json", TEMP_DIR_PATH, &id);
    match create_temp_dir() {
        Ok(()) => match File::create(&file_name).map(|it| serde_json::to_writer(&it, &value)) {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::SaveToCache),
        },
        Err(_) => Err(AppError::CreateTempDir),
    }
}
