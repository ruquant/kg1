pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;

#[derive(Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Clone, PartialEq)]
pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: usize, y: usize) -> usize {
    (y * MAP_WIDTH) + x
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; MAP_WIDTH * MAP_HEIGHT],
        }
    }

    // player cannot walk off the edge of the map
    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < MAP_WIDTH && y < MAP_HEIGHT
    }

    // player can walk on floor but not through walls
    pub fn can_enter_tile(&self, x: usize, y: usize) -> bool {
        self.in_bounds(x, y) && self.tiles[map_idx(x, y)] == TileType::Floor
    }
}
