use crate::item::Item;
use crate::map::{Map, TileType, MAP_HEIGHT, MAP_WIDTH};
use crate::player::{Player, MAX_ITEMS};

use crate::state::State;
use tezos_smart_rollup_host::path::{concat, OwnedPath, RefPath};
use tezos_smart_rollup_host::runtime::{Runtime, RuntimeError};

const MAP_PATH: RefPath = RefPath::assert_from(b"/state/map");
const X_POS_PATH: RefPath = RefPath::assert_from(b"/state/player/x_pos");
const Y_POS_PATH: RefPath = RefPath::assert_from(b"/state/player/y_pos");

pub fn load_player<R: Runtime>(rt: &mut R) -> Result<Player, RuntimeError> {
    let x_pos_exists = rt.store_has(&X_POS_PATH);
    let y_pos_exists = rt.store_has(&Y_POS_PATH);

    match (x_pos_exists, y_pos_exists) {
        (Ok(None), Ok(None)) => {
            let player = Player::new(MAP_WIDTH / 2, MAP_HEIGHT / 2);
            Ok(player)
        }
        (Err(err), _) | (_, Err(err)) => Err(err),
        (Ok(Some(_)), Ok(Some_)) => {
            let x_pos = rt.store_read(&X_POS_PATH, 0, std::mem::size_of::<usize>())?;
            let y_pos = rt.store_read(&Y_POS_PATH, 0, std::mem::size_of::<usize>())?;

            // convert player from usize, catch the list into an array
            let x_pos = usize::from_be_bytes(x_pos.try_into().unwrap());
            let y_pos = usize::from_be_bytes(y_pos.try_into().unwrap());

            let player = Player { x_pos, y_pos };
            Ok(player)
        }
    }
}

fn load_map<R: Runtime>(rt: &mut R) -> Result<Map, RuntimeError> {
    // checking in the durable storage to see if there is a map or not
    let map_exists = rt.store_has(&MAP_PATH)?;
    match map_exists {
        // if not create a new one
        None => Ok(Map::new()),
        // if there is something then read it from the durable storage
        Some(_) => {
            let map_bytes = rt.store_read(&MAP_PATH, 0, MAP_WIDTH * MAP_HEIGHT)?;

            let tiles: Vec<TileType> = map_bytes
                .iter()
                .filter_map(|bytes| match bytes {
                    0x01 => Some(TileType::Floor(Some(Item::Sword))),
                    0x02 => Some(TileType::Floor(Some(Item::Potion))),
                    0x03 => Some(TileType::Floor(None)),
                    0x04 => Some(TileType::Wall),
                    _ => None,
                })
                .collect();

            // define the map with the tiles
            let map = Map { tiles };
            Ok(map)
        }
    }
}

pub fn load_state<R: Runtime>(rt: &mut R) -> Result<State, RuntimeError> {
    let player = load_player(rt)?;
    let map = load_map(rt)?;
    Ok(State { player, map })
}

pub fn update_player<R: Runtime>(rt: &mut R, player: &Player) -> Result<(), RuntimeError> {
    // player position: convert vector back to bytes, and store it in the x_pos_path
    let x_pos = usize::to_be_bytes(player.x_pos);
    let () = rt.store_write(&player_x_pos(&player_address), &x_pos, 0)?;

    let y_pos = usize::to_be_bytes(player.y_pos);
    let () = rt.store_write(&player_y_pos(&player_address), &y_pos, 0)?;

    Ok(())
}

fn update_map<R: Runtime>(rt: &mut R, map: &Map) -> Result<(), RuntimeError> {
    let tiles: Vec<u8> = map
        .tiles
        .iter()
        .map(|tile_type| match tile_type {
            TileType::Floor(Some(Item::Sword)) => 0x01,
            TileType::Floor(Some(Item::Potion)) => 0x02,
            TileType::Floor(None) => 0x03,
            TileType::Wall => 0x04,
        })
        .collect();

    let () = rt.store_write(&MAP_PATH, &tiles, 0)?;

    Ok(())
}

pub fn update_state<R: Runtime>(rt: &mut R, state: &State) -> Result<(), RuntimeError> {
    update_player(rt, &state.player)?;
    update_map(rt, &state.map)?;
    Ok(())
}
