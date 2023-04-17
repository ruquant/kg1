use tezos_smart_rollup::{
    inbox::InboxMessage,
    kernel_entry,
    michelson::{Michelson, MichelsonUnit},
    prelude::*,
};

pub const MAGIC_BYTE: u8 = 0x1a;

fn read_inbox_message<Expr: Michelson>(host: &mut impl Runtime) {
    let input = host.read_input();
    match input {
        Err(_) | Ok(None) => (),
        Ok(Some(message)) => {
            // Parse the payload of the message
            let parsed_msg = InboxMessage::<Expr>::parse(message.as_ref()).unwrap();
            match parsed_msg {
                // Only process external messages that begin with the magic byte
                // associated with this kernel
                (remaining, InboxMessage::External([MAGIC_BYTE, data @ ..])) => {
                    assert!(remaining.is_empty());
                    let message = String::from_utf8_lossy(data);
                    debug_msg!(host, "External message: \"{}\"\n", message);
                }
                _ => (),
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
// 2. octez-smart-rollup-wasm-debugger target/wasm32-unknown-unknown/release/filtering_kernel.wasm --inputs 03_filtering_kernel/inputs.json
// 'load inputs'
// 'step result'
// Expected output:
//   External message: "This message is for me"
//   Evaluation took 438301 ticks so far
//   Status: Evaluating
//   Internal_status: Evaluation succeeded
