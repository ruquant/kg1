mod player_actions;
mod state;
use crate::player_actions::PlayerAction;
use crate::state::State;
use tezos_smart_rollup::host::RuntimeError;
use tezos_smart_rollup::{kernel_entry, prelude::*};

pub fn entry<R: Runtime>(rt: &mut R) {
    rt.write_debug("Hello world");
    loop {
        let input = rt.read_input();
        match input {
            Ok(Some(message)) => {
                // message is a list of byte
                let bytes = message.as_ref();
                let player_action = match bytes {
                    // 0x00: internal, 0x01: external
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
                        todo!()
                    }
                    None => {}
                }
            }
            _ => break,
        }
    }
}

kernel_entry!(entry);
