use tezos_smart_rollup_entrypoint::kernel_entry;
use tezos_smart_rollup_host::{
    path::RefPath,
    runtime::{Runtime, RuntimeError},
};

const MAP_WIDTH: usize = 32;
const MAP_HEIGHT: usize = 32;

const MAP_PATH: RefPath = RefPath::assert_from(b"/state/map");
const X_POS_PATH: RefPath = RefPath::assert_from(b"/state/player/x_pos");
const Y_POS_PATH: RefPath = RefPath::assert_from(b"/state/player/y_pos");

#[derive(Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

// Map
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

    // function to test if a map coordinate is valid.
    // if it is then return Some(index), otherwise returns None
    pub fn try_idx(&self, x: usize, y: usize) -> Option<usize> {
        if !self.in_bounds(x, y) {
            None
        } else {
            Some(map_idx(x, y))
        }
    }
}

// Player
#[derive(Clone)]
pub struct Player {
    pub x_pos: usize,
    pub y_pos: usize,
}

impl Player {
    pub fn new(x_pos: usize, y_pos: usize) -> Self {
        Self { x_pos, y_pos }
    }

    pub fn move_up(&self) -> Player {
        usize::checked_sub(self.y_pos, 1)
            .map(|y_pos| Self {
                y_pos,
                x_pos: self.x_pos,
            })
            .unwrap_or(self.clone())
    }

    pub fn move_down(&self) -> Player {
        Self {
            y_pos: self.y_pos + 1,
            x_pos: self.x_pos,
        }
    }

    pub fn move_left(&self) -> Player {
        usize::checked_sub(self.x_pos, 1)
            .map(|x_pos| Self {
                x_pos,
                y_pos: self.y_pos,
            })
            .unwrap_or(self.clone())
    }

    pub fn move_right(&self) -> Player {
        Self {
            x_pos: self.x_pos + 1,
            y_pos: self.y_pos,
        }
    }
}

// State
pub struct State {
    map: Map,
    player_position: Player,
}

pub enum PlayerAction {
    MoveRight,
    MoveLeft,
    MoveUp,
    MoveDown,
}

impl State {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            player_position: Player::new(MAP_WIDTH / 2, MAP_HEIGHT / 2),
        }
    }

    pub fn transition(self, player_action: PlayerAction) -> State {
        let next_player = match player_action {
            PlayerAction::MoveRight => self.player_position.move_right(),
            PlayerAction::MoveLeft => self.player_position.move_left(),
            PlayerAction::MoveUp => self.player_position.move_up(),
            PlayerAction::MoveDown => self.player_position.move_down(),
        };

        if self
            .map
            .can_enter_tile(next_player.x_pos, next_player.y_pos)
        {
            Self {
                player_position: next_player,
                ..self
            }
        } else {
            self
        }
    }
}

/// Read and write data from/to duable state of kernel, using the Runtime
fn load_state<R: Runtime>(rt: &mut R) -> Result<State, RuntimeError> {
    // Check whether or not there is existing the state inside the path
    // if not then return the default state
    let map_exists = rt.store_has(&MAP_PATH);
    let x_pos_exists = rt.store_has(&X_POS_PATH);
    let y_pos_exists = rt.store_has(&Y_POS_PATH);

    // we match them to check if they exist in the storage
    match (map_exists, x_pos_exists, y_pos_exists) {
        // if there is none state, create a new one
        (Ok(None), Ok(None), Ok(None)) => {
            let state = State::new();
            Ok(state)
        }
        (Err(err), _, _) | (_, Err(err), _) | (_, _, Err(err)) => Err(err),
        (Ok(Some(_)), Ok(Some(_)), Ok(Some(_))) => {
            // we have the path, now we read the data from it
            // store_read: know the size of the data, the offset is 0: starting of the bytes (from 0 to max_bytes)
            // store_read_slice: do not know the size of the data, will return the buffer
            let map_bytes = rt.store_read(&MAP_PATH, 0, MAP_WIDTH * MAP_HEIGHT)?;
            // the size max is only know at the run
            let x_pos = rt.store_read(&X_POS_PATH, 0, std::mem::size_of::<usize>())?;
            let y_pos = rt.store_read(&Y_POS_PATH, 0, std::mem::size_of::<usize>())?;

            // convert
            // map each bytes to idendify which bytes is a Floor or a Wall
            let tiles: Vec<TileType> = map_bytes
                .iter()
                .filter_map(|bytes| match bytes {
                    0x01 => Some(TileType::Floor),
                    0x02 => Some(TileType::Wall),
                    _ => None,
                })
                .collect();

            // define the map with the tiles
            let map = Map { tiles };

            // convert player from usize, catch the list into an array
            let x_pos = usize::from_be_bytes(x_pos.try_into().unwrap());
            let y_pos = usize::from_be_bytes(y_pos.try_into().unwrap());

            // Define a player position
            let player_position = Player { x_pos, y_pos };

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
            TileType::Floor => 0x01,
            TileType::Wall => 0x02,
        })
        .collect();

    // start to write from 0
    let () = rt.store_write(&MAP_PATH, &tiles, 0)?;

    // convert vector back to bytes, and store it in the x_pos_path
    let x_pos = usize::to_be_bytes(state.player_position.x_pos);
    let () = rt.store_write(&X_POS_PATH, &x_pos, 0)?;

    let y_pos = usize::to_be_bytes(state.player_position.y_pos);
    let () = rt.store_write(&Y_POS_PATH, &y_pos, 0)?;

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
                    _ => None,
                };
                match player_action {
                    Some(player_action) => {
                        rt.write_debug("Message is deserialized");
                        let state: Result<State, RuntimeError> = load_state(rt);
                        // match the state
                        match state {
                            Err(err) => rt.write_debug(&format!("error: {:?}", err)),
                            Ok(state) => {
                                rt.write_debug("Calling transtion");

                                let next_state = state.transition(player_action);
                                let res = update_state(rt, &next_state);
                                match res {
                                    Ok(_) => {}
                                    Err(err) => {
                                        rt.write_debug(&format!("State is not saved: {:?}", err));
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        rt.write_debug("Message is NOT deserialized");
                    }
                }
            }
            _ => break,
        }
    }
}

kernel_entry!(entry);
