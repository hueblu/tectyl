use anyhow::Result;
use crossterm::event::Event;

use crate::math::Rect;

use super::{terminal::Terminal, util::ScreenBuffer};

pub struct Tui {
    terminal: Terminal,

    buffers: [ScreenBuffer; 2],
    active_buffer: usize,
}

pub struct Frame<'a> {
    buf: &'a mut ScreenBuffer,

    size: (usize, usize),
}

pub trait Renderable {
    fn render(&self, frame: &mut Frame, size: Rect);
}

impl<T> Renderable for T
where
    T: Fn(&mut Frame, Rect),
{
    fn render(&self, frame: &mut Frame, size: Rect) {
        self(frame, size)
    }
}

impl Tui {
    pub async fn new() -> Result<Self> {
        let terminal = Terminal::new()?;
        let terminal_size = terminal.size().await;

        Ok(Self {
            terminal,
            buffers: [
                ScreenBuffer::new(terminal_size),
                ScreenBuffer::new(terminal_size),
            ],
            active_buffer: 0,
        })
    }

    pub fn draw(&mut self, f: impl Renderable) {
        let mut frame = Frame::new(&mut self.buffers[flip_buffer(self.active_buffer)]);
        let frame_size = frame.size();

        f.render(&mut frame, Rect::from(frame_size));

        frame.finish();
    }

    // writes the diff of the stored
    // buffers to the terminal, switching
    // them.
    pub fn flush(&mut self) -> Result<()> {
        // get diff of two buffers
        // write diff to screen

        let diff = self.get_active_buffer().diff(self.get_inactive_buffer())?;

        Ok(())
    }

    fn get_active_buffer(&self) -> &ScreenBuffer {
        &self.buffers[self.active_buffer]
    }

    fn get_active_buffer_mut(&mut self) -> &mut ScreenBuffer {
        &mut self.buffers[self.active_buffer]
    }

    fn get_inactive_buffer(&self) -> &ScreenBuffer {
        &self.buffers[flip_buffer(self.active_buffer)]
    }

    fn get_inactive_buffer_mut(&mut self) -> &mut ScreenBuffer {
        &mut self.buffers[flip_buffer(self.active_buffer)]
    }

    pub async fn recv_event(&mut self) -> Option<Event> {
        self.terminal.recv_event().await
    }

    pub fn exit(&mut self) -> Result<()> {
        self.terminal.exit()
    }
}

impl<'a> Frame<'a> {
    // api for widgets to draw to the buffer
    // with helper functions

    fn new(buf: &'a mut ScreenBuffer) -> Frame<'a> {
        let size = buf.size();

        Self { buf, size }
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    fn finish(&mut self) {
        // render
    }
}

fn flip_buffer(index: usize) -> usize {
    index ^ 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_buffer() {
        assert_eq!(flip_buffer(1), 0);
        assert_eq!(flip_buffer(0), 1);
    }
}
