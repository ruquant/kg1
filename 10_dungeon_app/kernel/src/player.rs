pub const MAX_ITEMS: usize = 2;

#[derive(Clone, PartialEq)]
pub struct Player {
    pub x_pos: usize,
    pub y_pos: usize,
}

impl Player {
    pub fn new(x_pos: usize, y_pos: usize) -> Self {
        Self { x_pos, y_pos }
    }
}
