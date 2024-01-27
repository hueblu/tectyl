use anyhow::Result;

use super::{terminal::Terminal, util::ScreenBuffer};

pub struct Tui {
    terminal: Terminal,

    buffers: [ScreenBuffer; 2],
}

impl Tui {
    // writes the diff of the stored
    // buffers to the terminal, switching
    // them.
    pub fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
