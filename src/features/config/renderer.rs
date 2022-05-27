use crossterm::style::Stylize;

use crate::common::renderer::Renderer;

pub struct FeatureConfigRenderer();

pub enum View {
    Error { description: String },
    Done { message: String },
}

impl Renderer<View> for FeatureConfigRenderer {
    fn render_internal(&mut self, view: View) -> String {
        match view {
            View::Error { description } => {
                format!("       {} {}\n", "Error".bold().red(), &description)
            }
            View::Done { message } => {
                format!("        {} {}\n", "Done".bold().green(), &message)
            }
        }
    }
}
