use std::io::{Stdout, Write};

pub struct TermUI {
    stdout: Stdout,
}

impl TermUI {
    pub fn new() -> TermUI {
        TermUI {
            stdout: std::io::stdout(),
        }
    }

    pub fn write(&mut self, str: String) {
        write!(
            self.stdout,
            "{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            str,
            termion::cursor::Hide
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }
}