use crossterm::style::Stylize;

use crate::common::renderer::{Indentable, Renderable};

pub enum View {
    ReadingConfig { path: String },
    ReceivedConfig { path: String },
    FetchingDom { url: String },
    DomFetched { url: String, from_cache: bool },
    ProcessingDom,
    FoundIcons(String),
    FetchingIcon(String),
    DownloadingIcon(String),
    IconDownloaded(String),
    IconExported(String),
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
            View::ProcessingDom => {
                format!(
                    "{} {}",
                    "Processing".indent().bold().cyan(),
                    "figma file nodes..."
                )
            }
            View::FoundIcons(frame_name) => format!(
                "{} figma frame `{}` with icons",
                "Found".indent().bold().green(),
                &frame_name,
            ),
            View::FetchingIcon(image_name) => format!(
                "{} download url for icon {}",
                "Fetching".indent().bold().cyan(),
                &image_name,
            ),
            View::DownloadingIcon(image_name) => {
                format!(
                    "{} icon {}",
                    "Downloading".indent().bold().cyan(),
                    &image_name
                )
            }
            View::IconDownloaded(image_name) => {
                format!(
                    "{} icon {}",
                    "Downloaded".indent().bold().green(),
                    &image_name
                )
            }
            View::IconExported(image_name) => {
                format!(
                    "{} icon {}",
                    "Exported".indent().bold().green(),
                    &image_name
                )
            }
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
