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
const INVENTORY_PATH: RefPath = RefPath::assert_from(b"/state/player/inventory");
const X_POS_ITEM_PATH: RefPath = RefPath::assert_from(b"/state/item/x_pos_item");
const Y_POS_ITEM_PATH: RefPath = RefPath::assert_from(b"/state/item/y_pos_item");

const MAX_ITEMS: usize = 2;

#[derive(Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

// Map
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

// Item

#[derive(Clone, PartialEq)]
pub struct Item {
    pub x_pos_item: usize,
    pub y_pos_item: usize,
}

impl Item {
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

// Player
#[derive(Clone, PartialEq)]
pub struct Player {
    pub x_pos: usize,
    pub y_pos: usize,
    pub inventory: Vec<Item>,
}

impl Player {
    pub fn new(x_pos: usize, y_pos: usize) -> Self {
        Self {
            x_pos,
            y_pos,
            inventory: Vec::new(),
        }
    }

    pub fn move_up(&self) -> Player {
        usize::checked_sub(self.y_pos, 1)
            .map(|y_pos| Self {
                y_pos,
                x_pos: self.x_pos,
                inventory: self.inventory.clone(),
            })
            .unwrap_or(self.clone())
    }

    pub fn move_down(&self) -> Player {
        Self {
            y_pos: self.y_pos + 1,
            x_pos: self.x_pos,
            inventory: self.inventory.clone(),
        }
    }

    pub fn move_left(&self) -> Player {
        usize::checked_sub(self.x_pos, 1)
            .map(|x_pos| Self {
                x_pos,
                y_pos: self.y_pos,
                inventory: self.inventory.clone(),
            })
            .unwrap_or(self.clone())
    }

    pub fn move_right(&self) -> Player {
        Self {
            x_pos: self.x_pos + 1,
            y_pos: self.y_pos,
            inventory: self.inventory.clone(),
        }
    }

    // add item to inventory
    pub fn add_item(self, item: Item) -> Player {
        let inventory_len = self.inventory.len();

        if inventory_len <= MAX_ITEMS {
            let x_pos_item = item.x_pos_item;
            let y_pos_item = item.y_pos_item;
            let item = Item {
                x_pos_item,
                y_pos_item,
            };

            let mut inventory = self.inventory;

            // add item to inventory
            // let inventory: Vec<u8> = self
            //     .inventory
            //     .iter()
            //     .map(|inventory| match inventory {
            //         Item {
            //             x_pos_item,
            //             y_pos_item,
            //         } => 0x06,
            //     })
            //     .collect();
            // let () = rt.store_write(&INVENTORY_PATH, &inventory, 0)?;

            inventory.push(item);

            Player { inventory, ..self }
        } else {
            self
        }
    }
}

// State
#[derive(Clone, PartialEq)]
pub struct State {
    map: Map,
    player_position: Player,
    item_position: Option<Item>,
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
            item_position: Some(Item {
                x_pos_item: MAP_WIDTH / 3,
                y_pos_item: MAP_HEIGHT / 3,
            }),
        }
    }

    // TODO pick up item
    // pickup takes: player, state as parameter
    // when item picked up the state of its is None
    // the item is then added into the inventory of player (check the max)
    //
    pub fn pick_up(self) -> State {
        let x_pos = self.player_position.x_pos;
        let y_pos = self.player_position.y_pos;

        let x_pos_item: usize = match &self.item_position {
            Some(item) => item.get_x(),
            _ => todo!(),
        };
        let y_pos_item: usize = match &self.item_position {
            Some(item) => item.get_y(),
            _ => todo!(),
        };

        if x_pos == x_pos_item && y_pos == y_pos_item {
            // add item to player inventory
            let item = Item {
                x_pos_item,
                y_pos_item,
            };
            let player = self.player_position.add_item(item);

            // update the state to none
            State {
                item_position: None,
                player_position: player,
                ..self
            }
        } else {
            self
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

            // item position, check if the path of the item is existed or not
            let item_position = match (
                rt.store_has(&X_POS_ITEM_PATH)?,
                rt.store_has(&Y_POS_ITEM_PATH)?,
            ) {
                (Some(_), Some(_)) => {
                    let x_pos_item_bytes =
                        rt.store_read(&X_POS_ITEM_PATH, 0, std::mem::size_of::<usize>())?;
                    let y_pos_item_bytes =
                        rt.store_read(&Y_POS_ITEM_PATH, 0, std::mem::size_of::<usize>())?;

                    // convert item position
                    let x_pos_item = usize::from_be_bytes(x_pos_item_bytes.try_into().unwrap());
                    let y_pos_item = usize::from_be_bytes(y_pos_item_bytes.try_into().unwrap());

                    // Define a player position
                    let item_position = Item {
                        x_pos_item,
                        y_pos_item,
                    };
                    Some(item_position)
                }
                _ => None,
            };

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

            // convert vector of inventory to bytes
            let inventory: Vec<Item> = inventory_bytes
                .iter()
                .filter_map(|bytes| match bytes {
                    0x06 => Some(Item {
                        // NOTE: the item maybe not need to have the position
                        x_pos_item: 0,
                        y_pos_item: 0,
                    }),
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
                item_position,
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

    // player position: convert vector back to bytes, and store it in the x_pos_path
    let x_pos = usize::to_be_bytes(state.player_position.x_pos);
    let () = rt.store_write(&X_POS_PATH, &x_pos, 0)?;

    let y_pos = usize::to_be_bytes(state.player_position.y_pos);
    let () = rt.store_write(&Y_POS_PATH, &y_pos, 0)?;

    // inventory
    let inventory: Vec<u8> = state
        .player_position
        .inventory
        .iter()
        .map(|inventory| match inventory {
            Item {
                x_pos_item: _,
                y_pos_item: _,
            } => 0x06,
        })
        .collect();

    let () = rt.store_write(&INVENTORY_PATH, &inventory, 0)?;

    // item

    match &state.item_position {
        None => {
            let () = rt.store_delete(&X_POS_ITEM_PATH)?;
            let () = rt.store_delete(&Y_POS_ITEM_PATH)?;
        }
        Some(item_position) => {
            let x_pos_item_bytes = usize::to_be_bytes(item_position.x_pos_item).to_vec();
            // to_state = 0x00, item position encoded in 4 bytes
            let () = rt.store_write(&X_POS_ITEM_PATH, &x_pos_item_bytes, 0)?;

            // save for y position, we need to have to_save define again
            let y_pos_item_bytes = usize::to_be_bytes(item_position.y_pos_item).to_vec();
            // 0x00, item position encoded in 4 bytes
            let () = rt.store_write(&Y_POS_ITEM_PATH, &y_pos_item_bytes, 0)?;
        }
    }
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
                    Err(_) => {}
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
