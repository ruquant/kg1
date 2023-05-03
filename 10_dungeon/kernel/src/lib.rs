mod map;
mod player;

use crate::TileType::Floor;
use map::{map_idx, Map, TileType, MAP_HEIGHT, MAP_WIDTH};
use player::{Player, MAX_ITEMS};
mod item;
use crate::{item::Item, Item::Potion, Item::Sword};
use tezos_smart_rollup_entrypoint::kernel_entry;
use tezos_smart_rollup_host::{
    path::RefPath,
    runtime::{Runtime, RuntimeError},
};

const MAP_PATH: RefPath = RefPath::assert_from(b"/state/map");

const X_POS_PATH: RefPath = RefPath::assert_from(b"/state/player/x_pos");
const Y_POS_PATH: RefPath = RefPath::assert_from(b"/state/player/y_pos");
const INVENTORY_PATH: RefPath = RefPath::assert_from(b"/state/player/inventory");

// Define State
#[derive(Clone, PartialEq)]
pub struct State {
    map: Map,
    player_position: Player,
}

pub enum PlayerAction {
    MoveRight,
    MoveLeft,
    MoveUp,
    MoveDown,
    PickUp,
}

impl State {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            player_position: Player::new(MAP_WIDTH / 2, MAP_HEIGHT / 2),
        }
    }

    pub fn pick_up(self) -> State {
        let x_pos = self.player_position.x_pos;
        let y_pos = self.player_position.y_pos;

        let idx = &self.map.tiles[map_idx(x_pos, y_pos)];

        match idx {
            Floor(Some(Potion)) => {
                let player_position = self.player_position.add_item(Potion);
                State {
                    player_position,
                    ..self
                }
            }
            Floor(Some(Sword)) => {
                let player_position = self.player_position.add_item(Sword);
                State {
                    player_position,
                    ..self
                }
            }
            Floor(None) => self,
            _ => self,
        }
    }

    fn update_player(self, player: Player) -> State {
        if self.map.can_enter_tile(player.x_pos, player.y_pos) {
            State {
                player_position: player,
                ..self
            }
        } else {
            self
        }
    }

    pub fn transition(self, player_action: PlayerAction) -> State {
        match player_action {
            PlayerAction::MoveRight => {
                let player = self.player_position.clone();
                self.update_player(player.move_right())
            }
            PlayerAction::MoveLeft => {
                let player = self.player_position.clone();
                self.update_player(player.move_left())
            }
            PlayerAction::MoveUp => {
                let player = self.player_position.clone();
                self.update_player(player.move_up())
            }
            PlayerAction::MoveDown => {
                let player = self.player_position.clone();
                self.update_player(player.move_down())
            }
            PlayerAction::PickUp => self.pick_up(),
        }
    }
}

/// Read and write data from/to duable state of kernel, using the Runtime
fn load_state<R: Runtime>(rt: &mut R) -> Result<State, RuntimeError> {
    // Check whether or not there is existing the state inside the path
    // if not then return the default state
    let map_exists = rt.store_has(&MAP_PATH);

    // player position
    let x_pos_exists = rt.store_has(&X_POS_PATH);
    let y_pos_exists = rt.store_has(&Y_POS_PATH);

    // inventory
    let inventory_exists = rt.store_has(&INVENTORY_PATH);
    // we match them to check if they exist in the storage
    match (map_exists, x_pos_exists, y_pos_exists, inventory_exists) {
        // if there is none state, create a new one
        (Ok(None), Ok(None), Ok(None), Ok(None)) => {
            rt.write_debug("Should be called only one time\n");
            let state = State::new();
            Ok(state)
        }
        (Err(err), _, _, _) | (_, Err(err), _, _) | (_, _, Err(err), _) | (_, _, _, Err(err)) => {
            Err(err)
        }
        (Ok(Some(_)), Ok(Some(_)), Ok(Some(_)), Ok(Some(_))) => {
            // we have the path, now we read the data from it
            // store_read: know the size of the data, the offset is 0: starting of the bytes (from 0 to max_bytes)
            // store_read_slice: do not know the size of the data, will return the buffer
            // the size max is only know at the run

            let map_bytes = rt.store_read(&MAP_PATH, 0, MAP_WIDTH * MAP_HEIGHT)?;

            // player position
            let x_pos = rt.store_read(&X_POS_PATH, 0, std::mem::size_of::<usize>())?;
            let y_pos = rt.store_read(&Y_POS_PATH, 0, std::mem::size_of::<usize>())?;

            // inventory
            let inventory_bytes = rt.store_read(&INVENTORY_PATH, 0, MAX_ITEMS)?;

            // convert
            // map each bytes to idendify which bytes is a Floor or a Wall
            let tiles: Vec<TileType> = map_bytes
                .iter()
                .filter_map(|bytes| match bytes {
                    0x01 => Some(TileType::Floor(Some(Sword))),
                    0x02 => Some(TileType::Floor(Some(Potion))),
                    0x03 => Some(TileType::Floor(None)),
                    0x04 => Some(TileType::Wall),
                    _ => None,
                })
                .collect();

            // define the map with the tiles
            let map = Map { tiles };

            // convert player from usize, catch the list into an array
            let x_pos = usize::from_be_bytes(x_pos.try_into().unwrap());
            let y_pos = usize::from_be_bytes(y_pos.try_into().unwrap());

            // convert vector of inventory to bytes
            let inventory: Vec<Item> = inventory_bytes
                .iter()
                .filter_map(|bytes| match bytes {
                    0x05 => Some(Item::Sword),
                    0x06 => Some(Item::Potion),
                    _ => None,
                })
                .collect();

            // Define a player position
            let player_position = Player {
                x_pos,
                y_pos,
                inventory: inventory,
            };

            // Define a State
            Ok(State {
                map,
                player_position,
            })
        }
        // other cases just create new state
        _ => Ok(State::new()),
    }
}

// the udpate state is the opposite of the load_state, we need to convert
// back the load_state
fn update_state<R: Runtime>(rt: &mut R, state: &State) -> Result<(), RuntimeError> {
    // convert map from vector to bytes
    let tiles: Vec<u8> = state
        .map
        .tiles
        .iter()
        .map(|tile_type| match tile_type {
            TileType::Floor(Some(Sword)) => 0x01,
            TileType::Floor(Some(Potion)) => 0x02,
            TileType::Floor(None) => 0x03,
            TileType::Wall => 0x04,
        })
        .collect();

    // start to write from 0
    let () = rt.store_write(&MAP_PATH, &tiles, 0)?;

    // player position: convert vector back to bytes, and store it in the x_pos_path
    let x_pos = usize::to_be_bytes(state.player_position.x_pos);
    let () = rt.store_write(&X_POS_PATH, &x_pos, 0)?;

    let y_pos = usize::to_be_bytes(state.player_position.y_pos);
    let () = rt.store_write(&Y_POS_PATH, &y_pos, 0)?;

    let inventory: Vec<u8> = state
        .player_position
        .inventory
        .iter()
        .map(|item| match item {
            Item::Sword => 0x05,
            Item::Potion => 0x06,
        })
        .collect();

    let () = rt.store_write(&INVENTORY_PATH, &inventory, 0)?;

    // item
    Ok(())
}

// Entry
pub fn entry<R: Runtime>(rt: &mut R) {
    rt.write_debug("Hello world");

    // Read the inbox messages
    loop {
        let input = rt.read_input();
        match input {
            Ok(Some(message)) => {
                let state: Result<State, RuntimeError> = load_state(rt);
                match state {
                    Err(err) => {
                        rt.write_debug(&format!("Error {:?}", err));
                    }
                    Ok(state) => {
                        // message is a list of byte
                        let bytes = message.as_ref();
                        let player_action = match bytes {
                            // First element or an array: 0x00: internal, 0x01: external
                            // second element define the bytes of player action
                            // move up
                            [0x01, 0x01] => Some(PlayerAction::MoveUp),
                            // move down
                            [0x01, 0x02] => Some(PlayerAction::MoveDown),
                            // move left
                            [0x01, 0x03] => Some(PlayerAction::MoveLeft),
                            // move right
                            [0x01, 0x04] => Some(PlayerAction::MoveRight),
                            // pickup
                            [0x01, 0x05] => Some(PlayerAction::PickUp),
                            _ => None,
                        };

                        // player and item
                        let next_state = match player_action {
                            Some(player_action) => {
                                rt.write_debug("Message is deserialized");
                                rt.write_debug("Calling transtion");
                                state.transition(player_action)
                            }
                            None => {
                                rt.write_debug("Message is NOT deserialized");
                                state
                            }
                        };
                        let _ = update_state(rt, &next_state);
                    }
                }
            }
            _ => break,
        }
    }
}

kernel_entry!(entry);
