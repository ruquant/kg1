#[derive(Clone, PartialEq)]
pub struct Item {
    pub x_pos_item: usize,
    pub y_pos_item: usize,
}

impl Item {
    #[allow(dead_code)]
    pub fn new(x_pos_item: usize, y_pos_item: usize) -> Self {
        Self {
            x_pos_item,
            y_pos_item,
        }
    }

    // use reference for reading the data
    pub fn get_x(&self) -> usize {
        self.x_pos_item
    }

    pub fn get_y(&self) -> usize {
        self.y_pos_item
    }
}
