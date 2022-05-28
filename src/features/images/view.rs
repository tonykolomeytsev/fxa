use crossterm::style::Stylize;

use crate::common::renderer::{Indentable, Renderable};

pub enum View {
    ReadingConfig { path: String },
    ReceivedConfig { path: String },
    FetchingDom { url: String },
    DomFetched { url: String, from_cache: bool },
    ProcessingDom,
    FoundImages(String),
    FetchingImage(String, String),
    DownloadingImage(String, String),
    ImageDownloaded(String, String),
    ConvertingToWebp(String, String),
    ConvertedToWebp(String, String),
    ImageExported(String, String),
    Error { description: String },
    Done { message: Option<String> },
}

impl Renderable for View {
    fn render(&self) -> String {
        match self {
            View::ReadingConfig { path } => format!(
                "{} config from file {}",
                "Loading".indent().bold().cyan(),
                &path,
            ),
            View::ReceivedConfig { path } => format!(
                "{} config from file {}",
                "Loaded".indent().bold().green(),
                &path,
            ),
            View::FetchingDom { url } => format!(
                "{} figma file nodes from {}",
                "Fetching".indent().bold().cyan(),
                &url,
            ),
            View::DomFetched { url, from_cache } => format!(
                "{} figma file nodes from {}",
                "Fetched".indent().bold().green(),
                if !from_cache { &url } else { "cache" },
            ),
            View::ProcessingDom => format!(
                "{} {}",
                "Processing".indent().bold().cyan(),
                "figma file nodes..."
            ),
            View::FoundImages(frame_name) => format!(
                "{} figma frame `{}` with images",
                "Found".indent().bold().green(),
                &frame_name,
            ),
            View::FetchingImage(image_name, scale) => format!(
                "{} download url for image {} ({})",
                "Fetching".indent().bold().cyan(),
                &image_name,
                &scale,
            ),
            View::DownloadingImage(image_name, scale) => format!(
                "{} image {} ({})",
                "Downloading".indent().bold().cyan(),
                &image_name,
                &scale,
            ),
            View::ImageDownloaded(image_name, scale) => format!(
                "  {} image {} ({})",
                "Downloaded".indent().bold().green(),
                &image_name,
                &scale,
            ),
            View::ConvertingToWebp(image_name, scale) => format!(
                "{} to WEBP image {} ({})...",
                "Converting".indent().bold().cyan(),
                &image_name,
                &scale,
            ),
            View::ConvertedToWebp(image_name, scale) => format!(
                "{} to WEBP image {} ({})",
                "Converted".indent().bold().green(),
                &image_name,
                &scale,
            ),
            View::ImageExported(image_name, scale) => format!(
                "{} image {} ({})",
                "Exported".indent().bold().green(),
                &image_name,
                &scale,
            ),
            View::Error { description } => {
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
