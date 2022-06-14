use crossterm::style::Stylize;

use crate::common::renderer::{Indentable, Renderable};

pub enum View {
    FetchingImage(String, String),
    DownloadingImage(String, String),
    ConvertingToWebp(String, String),
    ConvertedToWebp(String, String),
    ImageExported(String, String),
    Error(String),
    Done { message: Option<String> },
}

impl Renderable for View {
    fn render(&self) -> String {
        match self {
            View::FetchingImage(image_name, scale) => format!(
                "{} download url for image {} for {}",
                "Fetching".indent().bold().cyan(),
                &image_name,
                &scale,
            ),
            View::DownloadingImage(image_name, scale) => format!(
                "{} image {} for {}",
                "Downloading".indent().bold().cyan(),
                &image_name,
                &scale,
            ),
            View::ConvertingToWebp(image_name, scale) => format!(
                "{} to WEBP image {} for {}...",
                "Converting".indent().bold().cyan(),
                &image_name,
                &scale,
            ),
            View::ConvertedToWebp(image_name, scale) => format!(
                "{} to WEBP image {} for {}",
                "Converted".indent().bold().green(),
                &image_name,
                &scale,
            ),
            View::ImageExported(image_name, scale) => format!(
                "{} image {} to {}",
                "Exported".indent().bold().green(),
                &image_name,
                &scale,
            ),
            View::Error(description) => {
                format!("{} {}", "Error".indent().bold().red(), &description)
            }
            View::Done { message } => {
                if let Some(m) = message {
                    format!("{} {}", "Done".indent().bold().green(), &m)
                } else {
                    format!("{}", "Done".indent().bold().green())
                }
            }
        }
    }
}
