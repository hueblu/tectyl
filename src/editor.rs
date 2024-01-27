use std::{path::PathBuf, str::FromStr, usize};

use anyhow::Result;
use crossterm::{
    cursor::{MoveTo, MoveToColumn},
    event::{KeyCode, KeyEvent},
    style::Print,
    QueueableCommand,
};
use tokio::{fs::File, io::BufReader};

use crate::tui::Terminal;

pub struct Editor {
    documents: Vec<Document>,
    active_doc: usize,
    scroll: usize,

    mode: Mode,
}

struct Document {
    buffer: ropey::Rope,
    // (line_idx, line_offset)
    cursor_idx: usize,

    file: Option<PathBuf>,
}

#[derive(Clone, Copy)]
enum Mode {
    Normal,
    Input,
    Command,
}

impl Editor {
    //TODO remove need for async and result by not opening file in this method
    pub async fn new() -> Result<Self> {
        Ok(Self {
            documents: vec![Document::from_string("./src/main.rs")],
            active_doc: 0,
            scroll: 0,

            mode: Mode::Input,
        })
    }

    fn get_active_doc(&self) -> &Document {
        &self.documents[self.active_doc]
    }

    fn get_active_doc_mut(&mut self) -> &mut Document {
        &mut self.documents[self.active_doc]
    }

    pub fn handle_event(&mut self, event: KeyEvent) {
        match self.mode {
            Mode::Input => match event.code {
                KeyCode::Char(c) => {
                    self.get_active_doc_mut().insert(c);
                }
                KeyCode::Backspace => {
                    self.get_active_doc_mut().remove(1);
                }
                KeyCode::Enter => {
                    self.get_active_doc_mut().insert('\n');
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
        let lines = self.get_active_doc().buffer.lines_at(self.scroll);
        let size = terminal.size();
        let cursor_pos = self.get_active_doc().cursor_pos(None);

        for line in lines.take(size.1.into()) {
            terminal.out.queue(MoveToColumn(0))?.queue(Print(line))?;
        }

        terminal
            .out
            .queue(MoveTo(0, size.1))?
            .queue(Print(self.mode))?
            .queue(MoveTo(cursor_pos.0 - self.scroll as u16, cursor_pos.1))?;

        Ok(())
    }
}

impl Document {
    pub fn new() -> Self {
        Self {
            buffer: ropey::Rope::new(),
            cursor_idx: 0,
            file: None,
        }
    }

    pub fn from_string<S: ToString>(s: S) -> Self {
        let string = &*s.to_string();
        Self {
            buffer: ropey::Rope::from_str(string),
            cursor_idx: string.len(),
            file: None,
        }
    }

    pub async fn from_file<S: ToString>(s: S) -> Result<Self> {
        let path = PathBuf::from_str(&*s.to_string())?;
        let file = File::open(&path).await?;
        let buffer = ropey::Rope::from_reader(BufReader::new(file).buffer())?;

        Ok(Self {
            buffer,
            cursor_idx: 0,
            file: Some(path),
        })
    }

    pub fn insert<S: ToString>(&mut self, s: S) {
        let s = s.to_string();
        self.buffer.insert(self.cursor_idx, &*s);
        self.cursor_idx += s.len();
    }

    pub fn insert_char(&mut self, c: char) {
        self.buffer.insert_char(self.cursor_idx, c);
        self.cursor_idx += 1;
    }

    pub fn remove(&mut self, chars: usize) {
        self.buffer
            .remove(self.cursor_idx.saturating_sub(chars)..self.cursor_idx);

        self.cursor_idx = self.cursor_idx.saturating_sub(chars);
    }

    // wrap:
    //  - None if no wrap
    //  - Some(horizontal size of screen) if wrap
    //
    //  returns (x_offset, y_offset)
    pub fn cursor_pos(&self, wrap: Option<u16>) -> (u16, u16) {
        let line_idx = self.buffer.char_to_line(self.cursor_idx);
        let lines_to_cursor = self.buffer.lines().take(line_idx);

        if let Some(screen_size) = wrap {
            let y_offset = lines_to_cursor
                .map(|l| l.len_chars() as u16 / screen_size)
                .sum();
            let x_offset = self
                .buffer
                .lines()
                .take(line_idx)
                .last()
                .unwrap()
                .len_chars() as u16
                % screen_size;
            (x_offset, y_offset)
        } else {
            let y_offset = line_idx as u16;
            let x_offset = self.cursor_idx - lines_to_cursor.map(|l| l.len_chars()).sum::<usize>();

            (x_offset as u16, y_offset)
        }
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
