mod map;
mod player;

use crate::TileType::Floor;
use map::{Map, TileType, MAP_HEIGHT, MAP_WIDTH};
use player::{Player, MAX_ITEMS};
mod item;
mod player_actions;
mod state;
mod storage;
use crate::{item::Item, Item::Potion, Item::Sword};
use player_actions::PlayerAction;
use state::State;
use storage::{load_state, update_state};
use tezos_smart_rollup_entrypoint::kernel_entry;
use tezos_smart_rollup_host::{
    path::RefPath,
    runtime::{Runtime, RuntimeError},
};

// TODO: add wallet
const PLAYER_ADDRESS: &str = "tz1cBUPLRLzM77p5iQKTUxfDUp3vwPp9BKfQ";

// Entry
pub fn entry<R: Runtime>(rt: &mut R) {
    rt.write_debug("Hello world");

    // Read the inbox messages
    loop {
        let input = rt.read_input();
        match input {
            Ok(Some(message)) => {
                let state: Result<State, RuntimeError> = load_state(rt, PLAYER_ADDRESS);
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
                            // drop item, this action takes 3 bytes: 0x00 is the first place
                            // 0x01 is the 2nd position in the inventory
                            [0x01, 0x06, 0x00] => Some(PlayerAction::Drop(0)),
                            [0x01, 0x06, 0x01] => Some(PlayerAction::Drop(1)),
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
                        let _ = update_state(rt, PLAYER_ADDRESS, &next_state);
                    }
                }
            }
            _ => break,
        }
    }
}

kernel_entry!(entry);
