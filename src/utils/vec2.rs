#[derive(Clone, Copy, Default)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl Vec2 {
    pub fn new(x: usize, y: usize) -> Self {
        Vec2 { x, y }
    }
}
