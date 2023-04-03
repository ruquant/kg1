use std::marker::PhantomData;

use tezos_smart_rollup_host::input::Message;

use crate::{database::Database, host::Host, kernel::Kernel, node::TezosHeader};

pub struct LowLatency<K, H, D>
where
    K: Kernel,
    H: Host<D>,
    D: Database,
{
    host: H,
    _marker1: PhantomData<D>,
    _marker2: PhantomData<K>,
}

impl<K, H, D> LowLatency<K, H, D>
where
    D: Database,
    H: Host<D>,
    K: Kernel,
{
    /// Instanciate a new low latency component
    pub fn new(host: H) -> Self {
        Self {
            host,
            _marker1: PhantomData::default(),
            _marker2: PhantomData::default(),
        }
    }

    /// Simulates the message and update the durable state
    pub fn on_operation(&mut self, op: Message) {
        // Add the message to the runtime
        self.host.add_message(op);

        // Execute the kernel with the host
        K::entry(&mut self.host);
    }

    pub(crate) fn on_tezos_header(&mut self, _header: &TezosHeader)
    where
        D: Database + Send + 'static,
        K: Kernel,
        H: Host<D>,
    {
        println!("TODO: simulate the end of level and the two first message of the inbox")
    }
}
