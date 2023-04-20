use tezos_smart_rollup::{
    inbox::InboxMessage,
    kernel_entry,
    michelson::{Michelson, MichelsonUnit},
    prelude::*,
};

// The rollups inbox contains all messages addressed to all rollups.
// This kernel shows how to filter external messages in order to only handle
// those addressed to this rollup. For an example covering internal messages,
// see `outbox-kernel`.

// Unlike internal messages, external messages have no structure imposed on them.
// It is therefore the responsibility of each kernel to define their format and
// a way to identify which messages are addressed to it.
// Here, we will use a `MAGIC_BYTE`, which we will require as a prefix of any
// external message addressed to this rollup.
pub const MAGIC_BYTE: u8 = 0x1a;

fn read_inbox_message<Expr: Michelson>(host: &mut impl Runtime) {
    loop {
        match host.read_input() {
            Ok(Some(message)) => {
                // Parse the payload of the message
                match InboxMessage::<Expr>::parse(message.as_ref()) {
                    Ok(parsed_msg) => match parsed_msg {
                        (remaining, InboxMessage::External([MAGIC_BYTE, data @ ..])) => {
                            // Only process external messages that begin with the magic byte
                            // that we have defined for this rollup.
                            assert!(remaining.is_empty());
                            let message = String::from_utf8_lossy(data);
                            debug_msg!(host, "External message: \"{}\"\n", message);
                        }
                        // Ignore any other message
                        _ => (),
                    },
                    Err(_) =>
                    // Error parsing the message. This could happen when parsing a message
                    // sent to a different rollup, which might have a different Michelson type.
                    {
                        continue
                    }
                }
            }
            Ok(None) =>
            // Exit loop when there are no more messages to read.
            {
                break
            }
            Err(_) =>
            // An error here would most likely indicate a malformed message, ignore.
            {
                continue
            }
        }
    }
}

fn entry(host: &mut impl Runtime) {
    read_inbox_message::<MichelsonUnit>(host);
    host.mark_for_reboot().unwrap();
}

kernel_entry!(entry);
