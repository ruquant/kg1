mod item;
mod map;
mod market_place;
mod player;
mod player_actions;
mod state;
mod storage;
use player_actions::PlayerMsg;
use state::State;
use storage::{load_player, load_state, update_player, update_state};
use tezos_smart_rollup_entrypoint::kernel_entry;
use tezos_smart_rollup_host::runtime::{Runtime, RuntimeError};

/*pub fn entry<R: Runtime>(rt: &mut R) {
    rt.write_debug("Hello world");
    // Read the inbox messages
    loop {
        let input = rt.read_input();
        if let Ok(Some(message)) = input {
            let player_msg = PlayerMsg::try_from(message.as_ref().to_vec());
            if let Ok(player_msg) = player_msg {
                rt.write_debug("Message is deserialized");
                let PlayerMsg {
                    public_key: player_address,
                    action: player_action,
                } = player_msg;

                let other_player = match &player_action {
                    player_actions::PlayerAction::Buy(player_address, _) => {
                        load_player(rt, player_address).ok()
                    }
                    _ => None,
                };

                let state: Result<State, RuntimeError> = load_state(rt, &player_address);

                if let Ok(state) = state {
                    rt.write_debug("Calling transtion");
                    let (next_state, player) =
                        state.transition(other_player, player_action.clone(), &player_address);
                    let _ = update_state(rt, &player_address, &next_state);
                    match player {
                        None => {}
                        Some(player) => {
                            if let player_actions::PlayerAction::Buy(address, _) = &player_action {
                                let _ = update_player(rt, address, &player);
                            }
                        }
                    }
                }
            }
        }
    }
}*/

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

                        let other_player = match &player_action {
                            player_actions::PlayerAction::Buy(player_address, _) => {
                                load_player(rt, player_address).ok()
                            }
                            _ => None,
                        };

                        let state: Result<State, RuntimeError> = load_state(rt, &player_address);
                        match state {
                            Ok(state) => {
                                rt.write_debug("Calling transtion");
                                let (next_state, player) = state.transition(
                                    other_player,
                                    player_action.clone(),
                                    &player_address,
                                );
                                let _ = update_state(rt, &player_address, &next_state);
                                match player {
                                    None => {}
                                    Some(player) => match &player_action {
                                        player_actions::PlayerAction::Buy(address, _) => {
                                            let _ = update_player(rt, &address, &player);
                                        }
                                        _ => {}
                                    },
                                }
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
