use std::collections::VecDeque;

use tezos_smart_rollup_host::{
    input::Message,
    path::{OwnedPath, Path},
    runtime::{Runtime, RuntimeError, ValueType},
};

#[derive(Default)]
pub struct NativeHost {
    inputs: VecDeque<Message>,
    level: u32,
    id: u32,
}

pub trait AddInput {
    fn add_input(&mut self, input: Vec<u8>);
}

impl AddInput for NativeHost {
    fn add_input(&mut self, input: Vec<u8>) {
        let msg = Message::new(self.level, self.id, input);
        self.id += 1;
        self.inputs.push_back(msg);
    }
}

impl Runtime for NativeHost {
    fn write_output(&mut self, _from: &[u8]) -> Result<(), RuntimeError> {
        todo!()
    }

    fn write_debug(&self, msg: &str) {
        println!("Write_debug: {}", &msg);
    }

    fn read_input(&mut self) -> Result<Option<Message>, RuntimeError> {
        Ok(self.inputs.pop_front())
    }

    fn store_has<T: Path>(&self, _path: &T) -> Result<Option<ValueType>, RuntimeError> {
        todo!()
    }

    fn store_read<T: Path>(
        &self,
        _path: &T,
        _from_offset: usize,
        _max_bytes: usize,
    ) -> Result<Vec<u8>, RuntimeError> {
        todo!()
    }

    fn store_read_slice<T: Path>(
        &self,
        _path: &T,
        _from_offset: usize,
        _buffer: &mut [u8],
    ) -> Result<usize, RuntimeError> {
        todo!()
    }

    fn store_write<T: Path>(
        &mut self,
        _path: &T,
        _src: &[u8],
        _at_offset: usize,
    ) -> Result<(), RuntimeError> {
        todo!()
    }

    fn store_delete<T: Path>(&mut self, _path: &T) -> Result<(), RuntimeError> {
        todo!()
    }

    fn store_count_subkeys<T: Path>(&self, _prefix: &T) -> Result<i64, RuntimeError> {
        todo!()
    }

    fn store_get_subkey<T: Path>(
        &self,
        _prefix: &T,
        _index: i64,
    ) -> Result<OwnedPath, RuntimeError> {
        todo!()
    }

    fn store_move(
        &mut self,
        _from_path: &impl Path,
        _to_path: &impl Path,
    ) -> Result<(), RuntimeError> {
        todo!()
    }

    fn store_copy(
        &mut self,
        _from_path: &impl Path,
        _to_path: &impl Path,
    ) -> Result<(), RuntimeError> {
        todo!()
    }

    fn reveal_preimage(
        &self,
        _hash: &[u8; tezos_smart_rollup_core::PREIMAGE_HASH_SIZE],
        _destination: &mut [u8],
    ) -> Result<usize, RuntimeError> {
        todo!()
    }

    fn store_value_size(&self, _path: &impl Path) -> Result<usize, RuntimeError> {
        todo!()
    }

    fn mark_for_reboot(&mut self) -> Result<(), RuntimeError> {
        todo!()
    }

    fn reveal_metadata(
        &self,
    ) -> Result<tezos_smart_rollup_host::metadata::RollupMetadata, RuntimeError> {
        todo!()
    }

    fn last_run_aborted(&self) -> Result<bool, RuntimeError> {
        todo!()
    }

    fn upgrade_failed(&self) -> Result<bool, RuntimeError> {
        todo!()
    }

    fn restart_forced(&self) -> Result<bool, RuntimeError> {
        todo!()
    }

    fn reboot_left(&self) -> Result<u32, RuntimeError> {
        todo!()
    }

    fn runtime_version(&self) -> Result<String, RuntimeError> {
        todo!()
    }
}
