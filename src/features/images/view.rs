use crossterm::style::Stylize;

use crate::common::renderer::Renderable;

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
            View::FoundImages(frame_name) => format!(
                "       {} figma frame `{}` with images\n",
                "Found".bold().green(),
                &frame_name,
            ),
            View::FetchingImage(image_name, scale) => format!(
                "    {} download url for image {} ({})\n",
                "Fetching".bold().cyan(),
                &image_name,
                &scale,
            ),
            View::DownloadingImage(image_name, scale) => format!(
                " {} image {} ({})\n",
                "Downloading".bold().cyan(),
                &image_name,
                &scale,
            ),
            View::ImageDownloaded(image_name, scale) => format!(
                "  {} image {} ({})\n",
                "Downloaded".bold().green(),
                &image_name,
                &scale,
            ),
            View::ConvertingToWebp(image_name, scale) => format!(
                "  {} to WEBP image {} ({})...\n",
                "Converting".bold().cyan(),
                &image_name,
                &scale,
            ),
            View::ConvertedToWebp(image_name, scale) => format!(
                "   {} to WEBP image {} ({})\n",
                "Converted".bold().green(),
                &image_name,
                &scale,
            ),
            View::ImageExported(image_name, scale) => format!(
                "    {} image {} ({})\n",
                "Exported".bold().green(),
                &image_name,
                &scale,
            ),
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
