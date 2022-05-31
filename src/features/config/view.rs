use crossterm::style::Stylize;

use crate::common::renderer::{Indentable, Renderable};

pub enum View {
    Error(String),
    Created(String),
}

impl Renderable for View {
    fn render(&self) -> String {
        match self {
            View::Error(description) => {
                format!("{} {}", "Error".indent().bold().red(), &description)
            }
            View::Created(file_name) => {
                format!(
                    "{} config file {}",
                    "Created".indent().bold().green(),
                    &file_name,
                )
            }
        }
    }
}
