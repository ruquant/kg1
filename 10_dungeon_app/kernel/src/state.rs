use crate::map::{Map, TileType, MAP_HEIGHT, MAP_WIDTH};
use crate::player::Player;
use crate::player_actions::PlayerAction;

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

    pub fn transition(self, player_action: PlayerAction) -> State {
        let next_player = match player_action {
            PlayerAction::MoveRight => self.player.move_right(),
            PlayerAction::MoveLeft => self.player.move_left(),
            PlayerAction::MoveUp => self.player.move_up(),
            PlayerAction::MoveDown => self.player.move_down(),
        };
        self
    }
}
