use crossterm::style::Stylize;

use crate::common::renderer::{Indentable, Renderable};

pub enum View {
    ReadingConfig { path: String },
    ReceivedConfig { path: String },
    FetchingDom { url: String },
    DomFetched { url: String, from_cache: bool },
    ProcessingDom,
    FoundImages { frame_name: String },
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
                "{} config from file {}\n",
                "Loaded".indent().bold().green(),
                &path,
            ),
            View::FetchingDom { url } => format!(
                "{} figma file nodes from {}",
                "Fetching".indent().bold().cyan(),
                &url,
            ),
            View::DomFetched { url, from_cache } => {
                if *from_cache {
                    format!(
                        "{} figma file nodes {}\n",
                        "Fetched".indent().bold().green(),
                        "from cache".bold().white(),
                    )
                } else {
                    format!(
                        "{} figma file nodes from {}\n",
                        "Fetched".indent().bold().green(),
                        &url,
                    )
                }
            }
            View::ProcessingDom => format!(
                "{} {}",
                "Processing".indent().bold().cyan(),
                "figma file nodes..."
            ),
            View::FoundImages { frame_name } => format!(
                "{} figma frame `{}` with images\n",
                "Found".indent().bold().green(),
                &frame_name,
            ),
        }
    }
}
