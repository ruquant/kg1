use tezos_smart_rollup_encoding::inbox::{InboxMessage, InternalInboxMessage};
use tezos_smart_rollup_encoding::michelson::{Michelson, MichelsonUnit};
use tezos_smart_rollup_entrypoint::kernel_entry;
use tezos_smart_rollup_host::runtime::Runtime;

fn read_inbox_message<Expr: Michelson>(host: &mut impl Runtime) {
    let input = host.read_input();
    match input {
        Err(_) | Ok(None) => (),
        Ok(Some(message)) => {
            // Show the inbox level of the message
            host.write_debug(&format!("Inbox level: {} ", message.level.to_string()));
            // Parse the payload of the message
            let parsed_msg = InboxMessage::<Expr>::parse(message.as_ref()).unwrap();
            match parsed_msg {
                (remaining, InboxMessage::Internal(msg)) => {
                    assert!(remaining.is_empty());
                    match msg {
                        InternalInboxMessage::StartOfLevel => {
                            host.write_debug("Internal message: start of level\n")
                        }
                        InternalInboxMessage::InfoPerLevel(info) => {
                            host.write_debug(&format!(
                                "Internal message: level info \
                                          (block predecessor: {}, predecessor_timestamp: {}\n",
                                info.predecessor, info.predecessor_timestamp
                            ));
                        }
                        InternalInboxMessage::EndOfLevel => {
                            host.write_debug("Internal message: end of level\n")
                        }
                        InternalInboxMessage::Transfer(_) => {
                            host.write_debug("Internal message: transfer\n")
                        }
                    }
                }
                (remaining, InboxMessage::External(msg)) => {
                    assert!(remaining.is_empty());
                    let message = String::from_utf8_lossy(&msg);
                    host.write_debug(&format!("External message: \"{}\"\n", message));
                }
            }
            // Continue as long as there are messages in the inbox
            read_inbox_message::<MichelsonUnit>(host);
        }
    }
}

fn read_inbox_message_direct(host: &mut impl Runtime) {
    let input = host.read_input();
    match input {
        Err(_) | Ok(None) => (),
        Ok(Some(message)) => {
            let data = message.as_ref();
            match data {
                [0x00, ..] => {
                    host.write_debug("Internal message\n");
                }
                [0x01, ..] => {
                    let bytes: Vec<u8> = data.iter().skip(1).copied().collect();
                    let message = String::from_utf8_lossy(&bytes);
                    host.write_debug(&format!("External message: \"{}\"\n", message));
                }
                _ => (),
            }
            read_inbox_message_direct(host);
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
