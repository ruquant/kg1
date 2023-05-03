use std::vec;

use crate::{item::Item, Item::Potion, Item::Sword};

pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;

#[derive(Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor(Option<Item>),
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
            tiles: vec![
                // add sword in the beginning on the floor
                TileType::Floor(std::option::Option::Some(Sword));
                MAP_WIDTH * MAP_HEIGHT
            ],
        }
    }

    #[allow(dead_code)]
    pub fn get_sword(self) -> Item {
        let sword = TileType::Floor(std::option::Option::Some(Sword));
        match sword {
            TileType::Floor(Some(sword)) => sword,
            TileType::Floor(None) => todo!(),
            TileType::Wall => todo!(),
        }
    }

    #[allow(dead_code)]
    pub fn get_potion() -> Item {
        let potion = TileType::Floor(std::option::Option::Some(Potion));
        match potion {
            TileType::Floor(Some(potion)) => potion,
            TileType::Floor(None) => todo!(),
            TileType::Wall => todo!(),
        }
    }

    // player cannot walk off the edge of the map
    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < MAP_WIDTH && y < MAP_HEIGHT
    }

    // player can walk on floor but not through walls, and place the player
    // on the non item index
    pub fn can_enter_tile(&self, x: usize, y: usize) -> bool {
        self.in_bounds(x, y)
            && self.tiles[map_idx(x, y)] == TileType::Floor(std::option::Option::None)
    }
}
