use std::io::stdout;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use crate::editor::Editor;

pub struct App {
    editor: Editor,
}

enum Mode {
    Normal,
    Insert,
}

impl App {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        let editor = Editor::new();

        Ok(Self { editor })
    }

    pub fn draw(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
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
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = stdout().execute(LeaveAlternateScreen);
    }
}
