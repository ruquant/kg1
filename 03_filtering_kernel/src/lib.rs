use tezos_smart_rollup::{
    inbox::InboxMessage,
    kernel_entry,
    michelson::{Michelson, MichelsonUnit},
    prelude::*,
};

pub const MAGIC_BYTE: u8 = 0x1a;

fn read_inbox_message<Expr: Michelson>(host: &mut impl Runtime) {
    loop {
        match host.read_input() {
            Ok(None) => break,
            Err(_) => continue,
            Ok(Some(message)) => {
                // Parse the payload of the message
                match InboxMessage::<Expr>::parse(message.as_ref()) {
                    Ok(parsed_msg) => match parsed_msg {
                        // Only process external messages that begin with the magic byte
                        // associated with this kernel
                        (remaining, InboxMessage::External([MAGIC_BYTE, data @ ..])) => {
                            assert!(remaining.is_empty());
                            let message = String::from_utf8_lossy(data);
                            debug_msg!(host, "External message: \"{}\"\n", message);
                        }
                        _ => (),
                    },
                    Err(_) =>
                    // Error parsing the message. This could happend when parsing a message
                    // sent to a different rollup, which might have a different Michelson type.
                    {
                        continue
                    }
                }
            }
        }
    }
}

fn entry(host: &mut impl Runtime) {
    read_inbox_message::<MichelsonUnit>(host);
}

kernel_entry!(entry);
