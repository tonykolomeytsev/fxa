use crossterm::style::Stylize;

use crate::common::renderer::Renderable;

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
    fn render(self) -> String {
        match self {
            View::ReadingConfig { path } => format!(
                "     {} config from file {}\n",
                "Loading".bold().cyan(),
                &path,
            ),
            View::ReceivedConfig { path } => format!(
                "      {} config from file {}\n",
                "Loaded".bold().green(),
                &path,
            ),
            View::FetchingDom { url } => format!(
                "    {} figma file nodes from {}\n",
                "Fetching".bold().cyan(),
                &url,
            ),
            View::DomFetched { url, from_cache } => format!(
                "     {} figma file nodes from {}\n",
                "Fetched".bold().green(),
                if !from_cache { &url } else { "cache" },
            ),
            View::ProcessingDom => format!(
                "  {} {}\n",
                "Processing".bold().cyan(),
                "figma file nodes..."
            ),
            View::FoundIcons(frame_name) => format!(
                "       {} figma frame `{}` with icons\n",
                "Found".bold().green(),
                &frame_name,
            ),
            View::FetchingIcon(image_name) => format!(
                "    {} download url for icon {}\n",
                "Fetching".bold().cyan(),
                &image_name,
            ),
            View::DownloadingIcon(image_name) => {
                format!(" {} icon {}\n", "Downloading".bold().cyan(), &image_name)
            }
            View::IconDownloaded(image_name) => {
                format!("  {} icon {}\n", "Downloaded".bold().green(), &image_name)
            }
            View::IconExported(image_name) => {
                format!("    {} icon {}\n", "Exported".bold().green(), &image_name)
            }
            View::Error { description } => {
                format!("       {} {}\n", "Error".bold().red(), &description)
            }
            View::Done { message } => {
                if let Some(m) = message {
                    format!("        {} {}\n", "Done".bold().green(), &m)
                } else {
                    format!("        {}\n", "Done".bold().green())
                }
            }
        }
    }
}
