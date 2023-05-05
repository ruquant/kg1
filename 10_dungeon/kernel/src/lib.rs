mod map;
mod player;

mod item;
mod player_actions;
mod state;
mod storage;
use player_actions::PlayerMsg;
use state::State;
use storage::{load_state, update_state};
use tezos_smart_rollup_entrypoint::kernel_entry;
use tezos_smart_rollup_host::runtime::{Runtime, RuntimeError};

// Entry
pub fn entry<R: Runtime>(rt: &mut R) {
    rt.write_debug("Hello world");

    // Read the inbox messages
    loop {
        let input = rt.read_input();
        match input {
            Ok(Some(message)) => {
                let player_msg = PlayerMsg::try_from(message.as_ref().to_vec());
                match player_msg {
                    Ok(player_msg) => {
                        rt.write_debug("Message is deserialized");
                        let PlayerMsg {
                            public_key: player_address,
                            action: player_action,
                        } = player_msg;
                        let state: Result<State, RuntimeError> = load_state(rt, &player_address);
                        match state {
                            Ok(state) => {
                                rt.write_debug("Calling transtion");
                                let next_state = state.transition(player_action);
                                let _ = update_state(rt, &player_address, &next_state);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => break,
        }
    }
}

kernel_entry!(entry);
