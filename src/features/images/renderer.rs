use crossterm::{
    cursor,
    style::Stylize,
    terminal::{self, ClearType},
    QueueableCommand,
};
use std::{
    fmt::format,
    io::{stdout, Stdout, Write},
};

pub struct FeatureImagesRenderer {
    stdout: Stdout,
}

pub enum View {
    ReadingConfig { path: String },
    ReceivedConfig { path: String },
    FetchingDom { url: String },
    DomFetched { url: String },
    ProcessingDom,
    FoundImages(String),
    FetchingImage(String, f32),
    DownloadingImage(String, f32),
    ImageDownloaded(String, f32),
    ConvertingToWebp(String, f32),
    ConvertedToWebp(String, f32),
    Error { description: String },
    Done { message: Option<String> },
}

impl FeatureImagesRenderer {
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
            View::DomFetched { url } => self.apply(|| {
                format!(
                    "     {} figma file nodes from {}\n",
                    "Fetched".bold().green(),
                    &url,
                )
            }),
            View::ProcessingDom => self.apply(|| {
                format!(
                    "  {} {}\n",
                    "Processing".bold().cyan(),
                    "figma file nodes..."
                )
            }),
            View::FoundImages(frame_name) => self.apply(|| {
                format!(
                    "       {} figma frame `{}` with images\n",
                    "Found".bold().green(),
                    &frame_name,
                )
            }),
            View::FetchingImage(image_name, scale) => self.apply(|| {
                format!(
                    "    {} download url for image {} in scale {}\n",
                    "Fetching".bold().cyan(),
                    &image_name,
                    &scale,
                )
            }),
            View::DownloadingImage(image_name, scale) => self.apply(|| {
                format!(
                    " {} image {} in scale {}\n",
                    "Downloading".bold().cyan(),
                    &image_name,
                    &scale,
                )
            }),
            View::ImageDownloaded(image_name, scale) => self.apply(|| {
                format!(
                    "  {} image {} in scale {}\n",
                    "Downloaded".bold().green(),
                    &image_name,
                    &scale,
                )
            }),
            View::ConvertingToWebp(image_name, scale) => self.apply(|| {
                format!(
                    "  {} to WEBP image {} in scale {}...\n",
                    "Converting".bold().cyan(),
                    &image_name,
                    &scale,
                )
            }),
            View::ConvertedToWebp(image_name, scale) => self.apply(|| {
                format!(
                    "   {} to WEBP image {} in scale {}\n",
                    "Converted".bold().green(),
                    &image_name,
                    &scale,
                )
            }),
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
