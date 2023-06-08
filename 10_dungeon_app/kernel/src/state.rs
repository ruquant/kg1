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
    pub market_place: MarketPlace,
}

impl State {
    pub fn new(player_address: String) -> Self {
        Self {
            map: Map::new(),
            player: Player::new(MAP_WIDTH / 2, MAP_HEIGHT / 2, player_address),
            market_place: MarketPlace::new(),
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
