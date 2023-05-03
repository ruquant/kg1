use crate::item::Item;

pub const MAX_ITEMS: usize = 2;

#[derive(Clone, PartialEq)]
pub struct Player {
    pub x_pos: usize,
    pub y_pos: usize,
    pub inventory: Vec<Item>,
}

impl Player {
    pub fn new(x_pos: usize, y_pos: usize) -> Self {
        Self {
            x_pos,
            y_pos,
            inventory: Vec::new(),
        }
    }

    pub fn move_up(&self) -> Player {
        usize::checked_sub(self.y_pos, 1)
            .map(|y_pos| Self {
                y_pos,
                x_pos: self.x_pos,
                inventory: self.inventory.clone(),
            })
            .unwrap_or(self.clone())
    }

    pub fn move_down(&self) -> Player {
        Self {
            y_pos: self.y_pos + 1,
            x_pos: self.x_pos,
            inventory: self.inventory.clone(),
        }
    }

    pub fn move_left(&self) -> Player {
        usize::checked_sub(self.x_pos, 1)
            .map(|x_pos| Self {
                x_pos,
                y_pos: self.y_pos,
                inventory: self.inventory.clone(),
            })
            .unwrap_or(self.clone())
    }

    pub fn move_right(&self) -> Player {
        Self {
            x_pos: self.x_pos + 1,
            y_pos: self.y_pos,
            inventory: self.inventory.clone(),
        }
    }

    // add item to inventory
    pub fn add_item(self, item: Item) -> Player {
        let inventory_len = self.inventory.len();

        if inventory_len <= MAX_ITEMS {
            let mut inventory = self.inventory;

            inventory.push(item);

            Player { inventory, ..self }
        } else {
            self
        }
    }
}
