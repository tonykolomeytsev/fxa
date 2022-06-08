use crossterm::style::Stylize;

use crate::common::renderer::{Indentable, Renderable};

pub enum View {
    GatheringStarted,
    FoundSimple(String),
    FoundThemed(String),
    NotFound(String, String),
    NotFoundButSuggestions(String, String, Vec<String>),
}

impl Renderable for View {
    fn render(&self) -> String {
        match self {
            View::GatheringStarted => {
                format!(
                    "{} resource names...\n",
                    "Gathering".indent().bold().green(),
                )
            }
            View::FoundSimple(name) => format!(
                "{} resource with name `{}`\n",
                "Found".indent().bold().green(),
                name
            ),
            View::FoundThemed(name) => format!(
                "{} resource with name `{}` for light and dark theme\n",
                "Found".indent().bold().green(),
                name
            ),
            View::NotFound(resource_name, frame_name) => {
                format!(
                    "{} A resource with name `{}` is missing in frame `{}`\n",
                    "Error".indent().bold().red(),
                    resource_name,
                    frame_name,
                )
            }
            View::NotFoundButSuggestions(resource_name, frame_name, suggestions) => {
                let suggestions = suggestions
                    .iter()
                    .map(|s| format!("{:i$} `{}`", "", s, i = 12))
                    .collect::<Vec<String>>()
                    .join("\n");
                format!(
                    "{} resource with name `{}` in frame `{}`,\n {}{}\n{}\n",
                    "Missing".indent().bold().red(),
                    resource_name,
                    frame_name,
                    "".indent(),
                    "but there are resources with similar names:",
                    suggestions,
                )
            }
        }
    }
}
