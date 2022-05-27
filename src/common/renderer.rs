use std::io::{stdout, Write};

use crossterm::{
    cursor,
    terminal::{self, ClearType},
    QueueableCommand,
};

pub trait Renderable {
    fn render(self) -> String;
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
        stdout.flush().unwrap();
    }

    pub fn new_line(&self) {
        let mut stdout = stdout();
        stdout.write("\n".as_bytes()).unwrap();
        stdout.flush().unwrap();
    }
}
