use crate::{
    item::Item,
    map::Map,
    map::TileType,
    map::MAP_HEIGHT,
    map::MAP_WIDTH,
    player::{Player, MAX_ITEMS},
};

// Define State
#[derive(Clone, PartialEq)]
pub struct State {
    pub map: Map,
    pub player: Player,
}

impl State {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(MAP_WIDTH / 2, MAP_HEIGHT / 2),
        }
    }

    fn update_player(self, player: Player) -> State {
        if self.map.can_enter_tile(player.x_pos, player.y_pos) {
            State { player, ..self }
        } else {
            self
        }
    }
}
