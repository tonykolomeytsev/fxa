use crossterm::style::Stylize;

use crate::common::renderer::{Indentable, Renderable};

pub enum View {
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