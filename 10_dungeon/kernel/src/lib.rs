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

#[derive(Clone)]
pub enum TileType {
    Wall,
    Floor,
}

// Map
pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: usize, y: usize) -> usize {
    y * MAP_WIDTH + x
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; MAP_HEIGHT * MAP_HEIGHT],
        }
    }
}

// Player
pub struct Player {
    pub x_pos: usize,
    pub y_pos: usize,
}

impl Player {
    pub fn new(x_pos: usize, y_pos: usize) -> Self {
        Self { x_pos, y_pos }
    }

    pub fn move_up(&self) -> Player {
        Self {
            y_pos: self.y_pos - 1,
            x_pos: self.x_pos,
        }
    }

    pub fn move_down(&self) -> Player {
        Self {
            y_pos: self.x_pos + 1,
            x_pos: self.x_pos,
        }
    }

    pub fn move_left(&self) -> Player {
        Self {
            x_pos: self.x_pos - 1,
            y_pos: self.y_pos,
        }
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
    player: Player,
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
            player: Player::new(MAP_WIDTH / 2, MAP_HEIGHT / 2),
        }
    }

    pub fn transition(self, player_action: PlayerAction) -> State {
        let next_player = match player_action {
            PlayerAction::MoveRight => self.player.move_right(),
            PlayerAction::MoveLeft => self.player.move_left(),
            PlayerAction::MoveUp => self.player.move_up(),
            PlayerAction::MoveDown => self.player.move_down(),
        };

        if in_bounds(next_player.x_pos, next_player.y_pos) {
            Self {
                player: next_player,
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

            // Define a player
            let player = Player { x_pos, y_pos };

            // Define a State
            Ok(State { map, player })
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

    // convert vector back to bytes
    let x_pos = usize::to_be_bytes(state.player.x_pos);
    let () = rt.store_write(&X_POS_PATH, &x_pos, 0)?;

    let y_pos = usize::to_be_bytes(state.player.y_pos);
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
                        let state: Result<State, RuntimeError> = load_state(rt);
                        // match the state
                        match state {
                            Err(_) => {}
                            Ok(state) => {
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
                    None => {}
                }
            }
            _ => break,
        }
    }
}

kernel_entry!(entry);
