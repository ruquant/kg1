use tezos_smart_rollup_host::runtime::Runtime;

pub trait Kernel {
    fn entry<Host: Runtime>(host: &mut Host);
}

/// Dummy kernel for test purpose

pub struct DummyKernel;

/// Kernel implementation for test
///
/// Any kernel are supported
mod external_kernel {
    use tezos_smart_rollup_host::{
        path::RefPath,
        runtime::{Runtime, RuntimeError},
        Error,
    };

    const COUNTER_PATH: RefPath = RefPath::assert_from(b"/counter");

    #[derive(Default)]
    pub struct Counter {
        inner: u64,
    }

    impl Counter {
        fn increment(self) -> Self {
            Self {
                inner: self.inner + 1,
            }
        }
    }

    pub fn write_counter<Host: Runtime>(
        host: &mut Host,
        counter: &Counter,
    ) -> Result<(), RuntimeError> {
        let src = counter.inner.to_be_bytes();
        host.store_write(&COUNTER_PATH, &src, 0)
    }

    pub fn read_counter<Host: Runtime>(host: &mut Host) -> Result<Option<Counter>, RuntimeError> {
        match host.store_read(&COUNTER_PATH, 0, 8) {
            Err(RuntimeError::HostErr(Error::StoreNotANode)) => Ok(None),
            Err(RuntimeError::PathNotFound) => Ok(None),
            Err(err) => Err(err),
            Ok(bytes) => {
                let bytes = bytes.try_into().unwrap();
                let inner = u64::from_be_bytes(bytes);
                Ok(Some(Counter { inner }))
            }
        }
    }

    pub fn entry<Host: Runtime>(host: &mut Host) {
        let msg = "Hello kernel!";
        host.write_debug(msg);

        loop {
            let input = host.read_input();
            match input {
                Ok(Some(_)) => {
                    host.write_debug("A message has been received");
                    let counter = read_counter(host).unwrap().unwrap_or_default();
                    let counter = counter.increment();
                    let () = write_counter(host, &counter).unwrap();
                }
                Ok(None) => {
                    host.write_debug("End of the inbox");
                    break;
                }
                Err(_) => {
                    host.write_debug("Error ");
                    break;
                }
            }
        }
    }
}

impl Kernel for DummyKernel {
    fn entry<Host: Runtime>(host: &mut Host) {
        external_kernel::entry(host);
    }
}
