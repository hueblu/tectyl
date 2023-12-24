use std::cmp::{max, min};

/// A rectangle defined on the grid
///
/// See [Point]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rect {
    top_left: Point,
    bottom_right: Point,
}

/// A point defined on the terminal
///
/// points are based off of (0, 0)
/// being in the topleft corner of
/// the terminal, or the fourth
/// quadrant of the graph.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: u16,
    y: u16,
}

impl Rect {
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }

    #[inline]
    pub fn area(&self) -> u16 {
        self.width() * self.height()
    }

    #[inline]
    pub fn width(&self) -> u16 {
        self.bottom_right.x - self.top_left.x
    }

    #[inline]
    pub fn height(&self) -> u16 {
        self.bottom_right.y - self.top_left.y
    }

    #[inline]
    pub fn intersects(&self, other: &Rect) -> bool {
        !(other.area() == 0
            || self.area() == 0
            || self.top_left.x > other.bottom_right.x
            || other.top_left.x > self.bottom_right.x
            || self.top_left.y > other.bottom_right.y
            || other.top_left.y > self.bottom_right.y)
    }

    #[inline]
    pub fn get_intersection(&self, other: &Rect) -> Option<Rect> {
        if !self.intersects(other) {
            return None;
        }

        Some(Rect {
            top_left: Point {
                x: max(self.top_left.x, other.top_left.x),
                y: max(self.top_left.y, other.top_left.y),
            },
            bottom_right: Point {
                x: min(self.bottom_right.x, other.bottom_right.x),
                y: min(self.bottom_right.y, other.bottom_right.y),
            },
        })
    }
}

impl From<(Point, Point)> for Rect {
    fn from(value: (Point, Point)) -> Self {
        Self {
            top_left: value.0,
            bottom_right: value.1,
        }
    }
}

impl From<(u16, u16, u16, u16)> for Rect {
    fn from(value: (u16, u16, u16, u16)) -> Self {
        Self {
            top_left: Point::from((value.0, value.1)),
            bottom_right: Point::from((value.2, value.3)),
        }
    }
}

impl From<(u16, u16)> for Point {
    fn from(value: (u16, u16)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_intersects() {}

    #[test]
    fn rect_get_intersections() {}
}
