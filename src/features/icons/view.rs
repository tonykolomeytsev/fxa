use crossterm::style::Stylize;

use crate::common::renderer::{Indentable, Renderable};

pub enum View {
    FetchingIcon(String, String),
    DownloadingIcon(String, String),
    IconDownloaded(String, String),
    ConvertingToXml(String, String),
    ConvertedToXml(String, String),
    IconExported(String, String),
    Error(String),
    Done { message: Option<String> },
}

impl Renderable for View {
    fn render(&self) -> String {
        match self {
            View::FetchingIcon(image_name, dir_name) => format!(
                "{} download url for icon {} ({})",
                "Fetching".indent().bold().cyan(),
                &image_name,
                &dir_name,
            ),
            View::DownloadingIcon(image_name, dir_name) => {
                format!(
                    "{} icon {} ({})",
                    "Downloading".indent().bold().cyan(),
                    &image_name,
                    &dir_name,
                )
            }
            View::IconDownloaded(image_name, dir_name) => {
                format!(
                    "{} icon {} ({})",
                    "Downloaded".indent().bold().green(),
                    &image_name,
                    &dir_name,
                )
            }
            View::ConvertingToXml(image_name, dir_name) => {
                format!(
                    "{} to Android Drawable XML image {} ({})",
                    "Converting".indent().bold().cyan(),
                    &image_name,
                    &dir_name,
                )
            }
            View::ConvertedToXml(image_name, dir_name) => {
                format!(
                    "{} to Android Drawable XML image {} ({})",
                    "Converted".indent().bold().green(),
                    &image_name,
                    &dir_name,
                )
            }
            View::IconExported(image_name, dir_name) => {
                format!(
                    "{} icon {} ({})",
                    "Exported".indent().bold().green(),
                    &image_name,
                    &dir_name,
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
