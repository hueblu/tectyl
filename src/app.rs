use std::io::stdout;

use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};

pub struct App {
    buffer: ropey::Rope,
    mode: Mode,
}

enum Mode {
    Normal,
    Insert,
}

impl App {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        Ok(Self {
            buffer: ropey::Rope::new(),
            mode: Mode::Insert,
        })
    }

    pub fn draw(&mut self) -> Result<()> {
        let mut out = stdout();

        let size = crossterm::terminal::size()?;

        out.queue(MoveTo(0, 0))?;

        for line in self.buffer.lines().take(size.1 as usize) {
            out.queue(Print(line))?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            println!("{}", self.buffer);

            if let Ok(event) = event::read() {
                match event {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }) => {
                        break;
                    }

                    Event::Key(keyevent) => self.handle_key(keyevent)?,

                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn handle_key(&mut self, event: KeyEvent) -> Result<()> {
        match self.mode {
            Mode::Normal => match event.code {
                _ => {}
            },
            Mode::Insert => {
                if event.code == KeyCode::Esc {
                    self.mode = Mode::Normal
                } else if let KeyCode::Char(c) = event.code {
                    self.buffer.insert(0, &c.to_string());
                }
            }
        }

        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = stdout().execute(LeaveAlternateScreen);
    }
}
