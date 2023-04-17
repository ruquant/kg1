use tezos_smart_rollup::{
    inbox::{InboxMessage, InternalInboxMessage},
    kernel_entry,
    michelson::{Michelson, MichelsonUnit},
    prelude::*,
};

fn read_inbox_message<Expr: Michelson>(host: &mut impl Runtime) {
    let input = host.read_input();
    match input {
        Err(_) | Ok(None) => (),
        Ok(Some(message)) => {
            // Show the inbox level of the message
            debug_msg!(host, "Inbox level: {} ", message.level.to_string());
            // Parse the payload of the message
            let parsed_msg = InboxMessage::<Expr>::parse(message.as_ref()).unwrap();
            match parsed_msg {
                (remaining, InboxMessage::Internal(msg)) => {
                    assert!(remaining.is_empty());
                    match msg {
                        InternalInboxMessage::StartOfLevel => {
                            debug_msg!(host, "Internal message: start of level\n")
                        }
                        InternalInboxMessage::InfoPerLevel(info) => {
                            debug_msg!(
                                host,
                                "Internal message: level info \
                                          (block predecessor: {}, predecessor_timestamp: {}\n",
                                info.predecessor,
                                info.predecessor_timestamp
                            );
                        }
                        InternalInboxMessage::EndOfLevel => {
                            debug_msg!(host, "Internal message: end of level\n")
                        }
                        InternalInboxMessage::Transfer(_) => {
                            debug_msg!(host, "Internal message: transfer\n")
                        }
                    }
                }
                (remaining, InboxMessage::External(msg)) => {
                    assert!(remaining.is_empty());
                    let message = String::from_utf8_lossy(&msg);
                    debug_msg!(host, "External message: \"{}\"\n", message);
                }
            }
            // Continue as long as there are messages in the inbox
            read_inbox_message::<MichelsonUnit>(host);
        }
    }
}

fn entry(host: &mut impl Runtime) {
    read_inbox_message::<MichelsonUnit>(host);
}

kernel_entry!(entry);

// To run:
// 1. cargo build --release --target wasm32-unknown-unknown
// 2. octez-smart-rollup-wasm-debugger target/wasm32-unknown-unknown/release/inbox_kernel.wasm --inputs 02_inbox_kernel/inputs.json
// 'load inputs'
// 'step result'
// Expected output:
//   Inbox level: 0 Internal message: start of level
//   Inbox level: 0 Internal message: level info (block predecessor: BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M, predecessor_timestamp: 1970-01-01T00:00:00Z
//   Inbox level: 0 External message: "This is an external message"
//   Inbox level: 0 External message: "And here's another one"
//   Inbox level: 0 Internal message: end of level
//   Evaluation took 1106833 ticks so far
//   Status: Evaluating
//   Internal_status: Evaluation succeeded
