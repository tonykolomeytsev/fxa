use crossterm::style::Stylize;

use crate::common::renderer::Renderable;

pub enum View {
    Error { description: String },
    Done { message: String },
}

impl Renderable for View {
    fn render(self) -> String {
        match self {
            View::Error { description } => {
                format!("       {} {}\n", "Error".bold().red(), &description)
            }
            View::Done { message } => {
                format!("        {} {}\n", "Done".bold().green(), &message)
            }
        }
    }
}
