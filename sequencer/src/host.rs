use std::collections::VecDeque;

use tezos_smart_rollup_host::{
    input::Message,
    path::{OwnedPath, Path},
    runtime::{Runtime, RuntimeError, ValueType},
    Error,
};

pub struct NativeHost {
    inputs: VecDeque<Message>,
    level: u32,
    id: u32,
    db: sled::Db,
}

impl NativeHost {
    pub fn new(db: sled::Db) -> Self {
        NativeHost {
            inputs: VecDeque::default(),
            level: 0,
            id: 0,
            db,
        }
    }
}

/// Check the size of the data
///
/// The data should have a size greater than 2^31
pub fn check_data_size(data: &[u8]) -> Result<&[u8], RuntimeError> {
    i32::try_from(data.len())
        .map_err(|_| RuntimeError::HostErr(Error::StoreValueSizeExceeded))
        .map(|_| data)
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
        path: &T,
        from_offset: usize,
        max_bytes: usize,
    ) -> Result<Vec<u8>, RuntimeError> {
        let NativeHost { db, .. } = self;
        let key = path.as_bytes();
        let res = db.get(key);
        match res {
            Ok(Some(vec)) => {
                let mut data = vec
                    .to_vec()
                    .iter()
                    .skip(from_offset)
                    .copied()
                    .collect::<Vec<u8>>();
                if data.len() > max_bytes {
                    data.resize(max_bytes, 0);
                }
                Ok(data)
            }
            Err(_) => Err(RuntimeError::HostErr(Error::GenericInvalidAccess)),
            Ok(None) => Err(RuntimeError::HostErr(Error::StoreNotANode)),
        }
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
        path: &T,
        src: &[u8],
        at_offset: usize,
    ) -> Result<(), RuntimeError> {
        let NativeHost { db, .. } = self;
        let key = path.as_bytes();
        let src = check_data_size(src)?;
        let data = src.iter().skip(at_offset).copied().collect::<Vec<u8>>();
        let res = db.insert(key, data);
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(RuntimeError::HostErr(Error::GenericInvalidAccess)),
        }
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
