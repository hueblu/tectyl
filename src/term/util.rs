use anyhow::Result;

pub struct ScreenBuffer {
    size: (usize, usize),

    pub cells: Vec<char>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScreenBufferDiff {
    size: (usize, usize),
    pub cells: Vec<Option<char>>,
}

impl ScreenBuffer {
    pub fn new(size: (usize, usize)) -> Self {
        Self {
            size,

            cells: vec![' '; size.0 * size.1],
        }
    }

    pub fn resize(&mut self, new_size: (usize, usize)) {
        self.size = new_size;
        self.cells.resize(new_size.0 * new_size.1, ' ');
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn insert_char(&mut self, c: char, idx: usize) {
        self.cells[idx] = c;
    }

    // get diff of two buffers,
    // for each cell returning None if they are the same
    // and Some(c) if they are different, with c being
    // the char in other
    pub fn diff(&self, other: &Self) -> Result<ScreenBufferDiff> {
        if self.size != other.size {
            anyhow::bail!("Input buffers must have the same length");
        }

        let diff_cells = self
            .cells
            .iter()
            .zip(other.cells.iter())
            .map(|(&c1, &c2)| if c1 == c2 { None } else { Some(c2) })
            .collect::<Vec<_>>();

        Ok(ScreenBufferDiff {
            size: self.size,
            cells: diff_cells,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl From<String> for ScreenBuffer {
        fn from(value: String) -> Self {
            Self {
                size: (value.len(), 1),
                cells: value.chars().collect(),
            }
        }
    }

    #[test]
    fn buff_diff_same() {
        let buff1 = ScreenBuffer::from("hello".to_string());
        let buff2 = ScreenBuffer::from("hello".to_string());

        let diff_result = buff1.diff(&buff2).unwrap();

        let expected_diff = ScreenBufferDiff {
            size: buff1.size,
            cells: vec![None, None, None, None, None],
        };

        assert_eq!(diff_result, expected_diff);
    }

    #[test]
    fn buff_diff_different() {
        let buff1 = ScreenBuffer::from("hello".to_string());
        let buff2 = ScreenBuffer::from("hxl0o".to_string());

        let diff_result = buff1.diff(&buff2).unwrap();

        let expected_diff = ScreenBufferDiff {
            size: buff1.size,
            cells: vec![None, Some('x'), None, Some('0'), None],
        };

        assert_eq!(diff_result, expected_diff);
    }

    #[test]
    #[should_panic(expected = "Input buffers must have the same length")]
    fn buff_diff_length() {
        let buff1 = ScreenBuffer::from("hello".to_string());
        let buff2 = ScreenBuffer::from("world!".to_string());

        let diff_result = buff1.diff(&buff2).unwrap();
    }
}
