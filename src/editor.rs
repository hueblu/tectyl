use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    event::{KeyCode, KeyEvent},
    style::Print,
    terminal::size,
    QueueableCommand,
};

use crate::tui::Terminal;

pub struct Editor {
    buf: String,
    cursor: usize,
    mode: Mode,
}

#[derive(Clone, Copy)]
enum Mode {
    Normal,
    Input,
    Command,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buf: String::new(),
            cursor: 0,
            mode: Mode::Input,
        }
    }

    pub fn handle_event(&mut self, event: KeyEvent) {
        match self.mode {
            Mode::Input => match event.code {
                KeyCode::Char(c) => {
                    self.buf.insert(self.cursor, c);
                    self.cursor += 1;
                }
                KeyCode::Backspace => {
                    self.buf.remove(self.cursor - 1);
                    self.cursor -= 1;
                }
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
                }
                _ => {}
            },
            Mode::Normal => match event.code {
                KeyCode::Char('i') => self.mode = Mode::Input,

                _ => {}
            },

            _ => {}
        }
    }

    pub fn draw(&self, terminal: &mut Terminal) -> Result<()> {
        terminal.out.queue(Print(&self.buf))?;

        let size = terminal.size();

        let cursor_pos = (
            self.buf.len() as u16 % size.0,
            self.buf.len() as u16 / size.0,
        );

        terminal
            .out
            .queue(MoveTo(0, size.1))?
            .queue(Print(self.mode))?;

        terminal.out.queue(MoveTo(cursor_pos.0, cursor_pos.1))?;

        Ok(())
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Mode::Normal => "NORMAL",
            Mode::Input => "INPUT",
            Mode::Command => "COMMAND",
        })
    }
}
