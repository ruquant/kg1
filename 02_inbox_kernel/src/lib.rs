use tezos_smart_rollup::{
    inbox::{InboxMessage, InternalInboxMessage},
    kernel_entry,
    michelson::{Michelson, MichelsonUnit},
    prelude::*,
};

fn read_inbox_message<Expr: Michelson>(host: &mut impl Runtime) {
    loop {
        match host.read_input() {
            Ok(None) => break,
            Err(_) => continue,
            Ok(Some(message)) => {
                // Show the inbox level of the message
                debug_msg!(host, "Inbox level: {} ", message.level.to_string());
                // Parse the payload of the message
                match InboxMessage::<Expr>::parse(message.as_ref()) {
                    Ok(parsed_msg) => match parsed_msg {
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
