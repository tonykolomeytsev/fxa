use crossterm::{
    cursor,
    style::Stylize,
    terminal::{self, ClearType},
    QueueableCommand,
};
use std::io::{stdout, Stdout, Write};

pub struct FeatureConfigRenderer {
    stdout: Stdout,
}

pub enum View {
    Error { description: String },
    Done { message: String },
}

impl FeatureConfigRenderer {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    pub fn render(&mut self, view: View) {
        match view {
            View::Error { description } => {
                self.apply(|| format!("       {} {}\n", "Error".bold().red(), &description,))
            }
            View::Done { message } => {
                self.apply(|| format!("        {} {}\n", "Done".bold().green(), &message))
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
