use std::ops::{Deref, DerefMut};

use anyhow::Result;

use crate::math::Rect;

pub trait Drawable {
    fn draw(&self, rect: Rect, buffer: &mut ScreenBuffer) -> Result<()>;
}

pub struct Painter {
    buffers: [ScreenBuffer; 2],
    current_buf: usize, // current buffer index
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ScreenBuffer {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct Cell {
    inner: char,
}

impl Painter {
    pub fn new() -> Result<Self> {
        let size: (u16, u16) = crossterm::terminal::size()?;

        Ok(Self {
            buffers: [
                ScreenBuffer::new(size.0.into(), size.1.into()),
                ScreenBuffer::new(size.0.into(), size.1.into()),
            ],
            current_buf: 0,
        })
    }

    pub fn get_inactive_buffer(&mut self) -> &mut ScreenBuffer {
        &mut self.buffers[self.current_buf % 2]
    }

    pub fn draw(&mut self) -> Result<()> {
        Ok(())
    }
}

impl ScreenBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![Cell::default(); width * height];

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn raw(&self) -> String {
        self.cells
            .clone()
            .into_iter()
            .map(|c| *c)
            .collect::<String>()
    }

    // for each cell, gives None if they are the same
    // and returns the Cell in ScreenBuffer other if
    // they are different
    pub fn get_diff<'a, 'b>(&'a self, other: &'b ScreenBuffer) -> Option<Vec<Option<&'b Cell>>>
    where
        'b: 'a,
    {
        if !(self.width == other.width && self.height == other.height) {
            return None;
        }

        Some(
            self.cells
                .iter()
                .zip(other.cells.iter())
                .map(|(a, b)| if a == b { None } else { Some(b) })
                .collect::<Vec<_>>(),
        )
    }
}

impl Cell {
    pub fn empty() -> Self {
        Self { inner: ' ' }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Self { inner: value }
    }
}

impl Deref for Cell {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Cell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl From<String> for ScreenBuffer {
        fn from(value: String) -> Self {
            let mut cells: Vec<Cell> = Vec::with_capacity(value.len());

            for c in value.chars() {
                cells.push(c.into());
            }

            Self {
                width: value.len(),
                height: 1,
                cells,
            }
        }
    }

    #[test]
    fn screen_buffer_from_string() {
        let string = String::from("Hello World");
        let buf = ScreenBuffer::from(string.clone());

        assert_eq!(string, buf.raw());
    }

    #[test]
    fn screen_buffer_diff() {
        let buff1 = ScreenBuffer::from("balls balls".to_string());
        let buff2 = ScreenBuffer::from("beooe aalls".to_string());
        let expected = vec![
            None,
            Some('e'),
            Some('o'),
            Some('o'),
            Some('e'),
            None,
            Some('a'),
            None,
            None,
            None,
            None,
        ];

        let diff = buff1
            .get_diff(&buff2)
            .unwrap()
            .into_iter()
            .map(|c| c.map(|f| **f))
            .collect::<Vec<Option<char>>>();

        assert_eq!(diff, expected);
    }
}
