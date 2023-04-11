use super::tezos_header::TezosHeader;
use tezos_smart_rollup_host::input::Message;

pub trait Sequencer {
    fn on_operation(&mut self, operation: Vec<u8>) -> Message;
    fn on_tezos_header(&mut self, tezos_header: &TezosHeader) -> Vec<Vec<u8>>;
}
