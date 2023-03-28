use tezos_smart_rollup_host::{path::RefPath, runtime::Runtime};

pub trait Kernel {
    fn entry<Host: Runtime>(host: &mut Host);
}

/// Dummy kernel for test purpose

pub struct DummyKernel;

impl Kernel for DummyKernel {
    fn entry<Host: Runtime>(host: &mut Host) {
        let msg = "Hello kernel!";
        host.write_debug(msg);

        loop {
            let input = host.read_input();
            match input {
                Ok(Some(_)) => {
                    host.write_debug("A message has been received");
                    let path = RefPath::try_from("/counter").unwrap();
                    let counter: u64 = 1;
                    let () = host.store_write(&path, &counter.to_be_bytes(), 0).unwrap();
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
