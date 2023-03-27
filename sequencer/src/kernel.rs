use tezos_smart_rollup_host::runtime::Runtime;

pub trait Kernel {
    fn entry<Host: Runtime>(host: &mut Host);
}

/// Dummy kernel for test purpose

pub struct DummyKernel;

impl Kernel for DummyKernel {
    fn entry<Host: Runtime>(host: &mut Host) {
        let msg = "Hello kernel!";
        host.write_debug(msg);
    }
}
