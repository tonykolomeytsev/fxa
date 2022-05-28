use std::io::{stdout, Write};

use crossterm::{
    cursor,
    terminal::{self, ClearType},
    QueueableCommand,
};

const INDENT_SIZE: usize = 12usize;

pub trait Indentable {
    fn indent(&self) -> String;
}

impl Indentable for &str {
    fn indent(&self) -> String {
        let len = self.len();
        let indent = if len <= INDENT_SIZE {
            INDENT_SIZE - len
        } else {
            0usize
        };
        format!("{:indent$}{}", "", &self, indent = indent)
    }
}

pub trait Renderable {
    fn render(&self) -> String;
}

pub struct Renderer();

impl Renderer {
    pub fn render<V>(&self, view: V)
    where
        V: Renderable,
    {
        let mut stdout = stdout();
        stdout.queue(cursor::MoveToPreviousLine(1u16)).unwrap();
        stdout
            .queue(terminal::Clear(ClearType::CurrentLine))
            .unwrap();
        stdout.write(view.render().as_bytes()).unwrap();
        stdout.write(b"\n").unwrap();
        stdout.flush().unwrap();
    }

    pub fn new_line(&self) {
        let mut stdout = stdout();
        stdout.write(b"\n").unwrap();
        stdout.flush().unwrap();
    }
}
