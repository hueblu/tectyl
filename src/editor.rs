pub struct Editor {
    buffer: ropey::Rope,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffer: ropey::Rope::new(),
        }
    }
}
