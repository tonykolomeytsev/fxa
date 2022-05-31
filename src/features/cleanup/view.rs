use crossterm::style::Stylize;

use crate::common::renderer::{Indentable, Renderable};

pub enum View {
    Error(String),
    Done,
}

impl Renderable for View {
    fn render(&self) -> String {
        match self {
            View::Error(description) => {
                format!("{} {}", "Error".indent().bold().red(), &description)
            }
            View::Done => {
                format!("{}", "Done".indent().bold().green())
            }
        }
    }
}
