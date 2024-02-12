#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    x: usize,
    y: usize,

    width: usize,
    height: usize,
}

impl From<(u16, u16)> for Rect {
    fn from(value: (u16, u16)) -> Self {
        Self {
            x: 0,
            y: 0,
            width: value.0 as usize,
            height: value.1 as usize,
        }
    }
}

impl From<(usize, usize)> for Rect {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: 0,
            y: 0,
            width: value.0,
            height: value.1,
        }
    }
}
