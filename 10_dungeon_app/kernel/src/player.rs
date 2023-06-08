use crate::item::Item;

pub const MAX_ITEMS: usize = 2;

#[derive(Clone, PartialEq)]
pub struct Player {
    pub address: String,
    pub x_pos: usize,
    pub y_pos: usize,
    pub inventory: Vec<Item>,
    pub gold: usize,
}

impl Player {
    pub fn new(x_pos: usize, y_pos: usize, address: String) -> Self {
        Self {
            address,
            x_pos,
            y_pos,
            inventory: Vec::new(),
            // default 1000 gold for each player
            gold: 1000,
        }
    }
}
