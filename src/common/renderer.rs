use std::io::{stdout, Write};

use crossterm::{
    cursor,
    terminal::{self, ClearType},
    QueueableCommand,
};

pub trait Renderer<V> {
    fn render(&mut self, view: V) {
        let mut stdout = stdout();
        stdout.queue(cursor::MoveToPreviousLine(1u16)).unwrap();
        stdout
            .queue(terminal::Clear(ClearType::CurrentLine))
            .unwrap();
        stdout.write(self.render_internal(view).as_bytes()).unwrap();
        stdout.flush().unwrap();
    }

    fn new_line(&mut self) {
        let mut stdout = stdout();
        stdout.write("\n".as_bytes()).unwrap();
        stdout.flush().unwrap();
    }

    fn render_internal(&mut self, view: V) -> String;
}
