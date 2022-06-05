use crossterm::style::Stylize;

use crate::common::renderer::{Indentable, Renderable};

pub enum View {
    FetchingIcon(String),
    DownloadingIcon(String),
    IconDownloaded(String),
    ConvertingToXml(String),
    ConvertedToXml(String),
    IconExported(String),
    ErrorWithSuggestions(String, Vec<String>),
    Error(String),
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
            View::ConvertingToXml(image_name) => {
                format!(
                    "{} to Android Drawable XML image {}",
                    "Converting".indent().bold().cyan(),
                    &image_name,
                )
            }
            View::ConvertedToXml(image_name) => {
                format!(
                    "{} to Android Drawable XML image {}",
                    "Converted".indent().bold().green(),
                    &image_name,
                )
            }
            View::IconExported(image_name) => {
                format!(
                    "{} icon {}",
                    "Exported".indent().bold().green(),
                    &image_name
                )
            }
            View::ErrorWithSuggestions(description, suggestions) => {
                let suggestions = suggestions
                    .iter()
                    .map(|s| format!("{:i$} `{}`", "", s, i = 12))
                    .collect::<Vec<String>>()
                    .join("\n");
                format!(
                    "{} {}\n{}",
                    "Error".indent().bold().red(),
                    &description,
                    suggestions,
                )
            }
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
