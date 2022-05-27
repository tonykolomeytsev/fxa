use crossterm::{
    cursor,
    style::Stylize,
    terminal::{self, ClearType},
    QueueableCommand,
};
use std::io::{stdout, Stdout, Write};

pub struct FeatureIconsRenderer {
    stdout: Stdout,
}

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

impl FeatureIconsRenderer {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    pub fn render(&mut self, view: View) {
        match view {
            View::ReadingConfig { path } => self.apply(|| {
                format!(
                    "     {} config from file {}\n",
                    "Loading".bold().cyan(),
                    &path,
                )
            }),
            View::ReceivedConfig { path } => self.apply(|| {
                format!(
                    "      {} config from file {}\n",
                    "Loaded".bold().green(),
                    &path,
                )
            }),
            View::FetchingDom { url } => self.apply(|| {
                format!(
                    "    {} figma file nodes from {}\n",
                    "Fetching".bold().cyan(),
                    &url,
                )
            }),
            View::DomFetched { url, from_cache } => self.apply(|| {
                format!(
                    "     {} figma file nodes from {}\n",
                    "Fetched".bold().green(),
                    if !from_cache { &url } else { "cache" },
                )
            }),
            View::ProcessingDom => self.apply(|| {
                format!(
                    "  {} {}\n",
                    "Processing".bold().cyan(),
                    "figma file nodes..."
                )
            }),
            View::FoundIcons(frame_name) => self.apply(|| {
                format!(
                    "       {} figma frame `{}` with icons\n",
                    "Found".bold().green(),
                    &frame_name,
                )
            }),
            View::FetchingIcon(image_name) => self.apply(|| {
                format!(
                    "    {} download url for icon {}\n",
                    "Fetching".bold().cyan(),
                    &image_name,
                )
            }),
            View::DownloadingIcon(image_name) => {
                self.apply(|| format!(" {} icon {}\n", "Downloading".bold().cyan(), &image_name,))
            }
            View::IconDownloaded(image_name) => {
                self.apply(|| format!("  {} icon {}\n", "Downloaded".bold().green(), &image_name,))
            }
            View::IconExported(image_name) => {
                self.apply(|| format!("    {} icon {}\n", "Exported".bold().green(), &image_name,))
            }
            View::Error { description } => {
                self.apply(|| format!("       {} {}\n", "Error".bold().red(), &description))
            }
            View::Done { message } => {
                if let Some(m) = message {
                    self.apply(|| format!("        {} {}\n", "Done".bold().green(), &m))
                } else {
                    self.apply(|| format!("        {}\n", "Done".bold().green()))
                }
            }
        }
    }

    pub fn new_line(&mut self) {
        self.stdout.write("\n".as_bytes()).unwrap();
        self.stdout.flush().unwrap();
    }

    fn apply<F>(&mut self, source: F)
    where
        F: Fn() -> String,
    {
        self.stdout.queue(cursor::MoveToPreviousLine(1u16)).unwrap();
        self.stdout
            .queue(terminal::Clear(ClearType::CurrentLine))
            .unwrap();
        self.stdout.write(source().as_bytes()).unwrap();
        self.stdout.flush().unwrap();
    }
}
