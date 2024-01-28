use anyhow::Result;
use crossterm::event::Event;

use super::{terminal::Terminal, util::ScreenBuffer};

pub struct Tui {
    terminal: Terminal,

    buffers: [ScreenBuffer; 2],
}

impl Tui {
    pub fn new() -> Result<Self> {
        let terminal = Terminal::new()?;

        Ok(Self {
            terminal: Terminal::new()?,
            buffers: [
                ScreenBuffer::new(terminal.size()),
                ScreenBuffer::new(terminal.size()),
            ],
        })
    }

    // writes the diff of the stored
    // buffers to the terminal, switching
    // them.
    pub fn flush(&mut self) -> Result<()> {
        Ok(())
    }

    pub async fn recv_event(&mut self) -> Option<Event> {
        self.terminal.recv_event().await
    }

    pub fn exit(&mut self) -> Result<()> {
        self.terminal.exit()
    }
}
