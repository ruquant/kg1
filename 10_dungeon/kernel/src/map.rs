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
        let mut map = vec![
            // add sword in the beginning on the floor
            TileType::Floor(None);
            MAP_WIDTH * MAP_HEIGHT
        ];

        // place wall at the 0
        map[0] = TileType::Wall;
        // place the sword on the 48 on the map
        map[48] = TileType::Floor(Some(Item::Sword));
        map[970] = TileType::Floor(Some(Item::Potion));

        Self { tiles: map }
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

    pub fn get_tile(&self, x: usize, y: usize) -> Option<TileType> {
        // cloned: inner value of an option and not an option itself
        self.tiles.get(map_idx(x, y)).cloned()
    }

    // player can walk on floor but not through walls
    pub fn can_enter_tile(&self, x: usize, y: usize) -> bool {
        // using [get] to avoid of [out_of_bound] in the [map_idx] function
        match self.get_tile(x, y) {
            Some(TileType::Wall) => false,
            Some(TileType::Floor(_)) => true,
            None => false,
        }
    }

    // remove item from the map
    pub fn remove_item(self, x_pos: usize, y_pos: usize) -> Self {
        match self.get_tile(x_pos, y_pos) {
            Some(TileType::Floor(Some(_))) => {
                let mut tiles = self.tiles;
                tiles[map_idx(x_pos, y_pos)] = TileType::Floor(None);
                Self { tiles }
            }
            _ => self,
        }
    }
}
