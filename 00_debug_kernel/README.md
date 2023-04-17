# Example 0: Debug Kernel

In our first kernel, we will demonstrate how to write debug messages
and read from the shared inbox.

## Running the example

First, compile the kernel to WASM with Cargo:
<!-- $MDX skip -->
```sh
$ cargo build --release --target wasm32-unknown-unknown
```

Then you can execute the kernel locally against the provided inputs (empty in this example) and commands:
```sh
$ octez-smart-rollup-wasm-debugger \
> ../target/wasm32-unknown-unknown/release/debug_kernel.wasm \
> --inputs ./inputs.json \
> --commands ./commands.json
Loaded 0 inputs at level 0
Hello from kernel!
Evaluation took 55264 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
```

Additionally, you can omit the `--commands` flag to enter a REPL mode and
explore the execution of the kernel interactively. Try it out!
