# Pistachio

[[_TOC_]]

Pistachio is an example project to show how to develop and use [Tezos SORU WASM kernel](http://tezos.gitlab.io/alpha/smart_rollups.html#developing-wasm-kernels). Currently we have 4 example kernels:

- **debug kernel**: shows how to use `WasmHost::write_debug` to write message to the debug log.
- **output kernel**: shows how to use input/output message.
- **hello kernel**: shows how to use [Capn' Proto](https://capnproto.org) for safe and efficient message decoding on DAC/DAL input.
- **counter kernel**: shows how to storing (read/write) an `Int`.

## Build

### Setup Rust

The suggested [Rust](https://www.rust-lang.org/) version is `1.66.0`.

You can install from scratch

```shell
# [install rust]
wget https://sh.rustup.rs/rustup-init.sh
chmod +x rustup-init.sh
./rustup-init.sh --profile minimal --default-toolchain 1.66.0 -y
# [source cargo]
. $HOME/.cargo/env
```

or, you can use `rustup` instead,

```shell
rustup update 1.66.0
rustup override set 1.66.0-<channel_full_name>
rustup toolchain install 1.66.0
```

More details of install Rust can be found at: https://www.rust-lang.org/tools/install.

### Setup WASM

We need to add `wasm32-unknown-unknown` to be a possible target of Rust:

```shell
rustup target add wasm32-unknown-unknown
```

### Build kernel to WASM with Cargo

We provide pre-defined tasks for building kernels, that requires [`cargo-make`](https://github.com/sagiegurari/cargo-make):

```shell
cargo install cargo-make
```

After install `cargo-make` we can now build our kernel! Remember to replace `<name>` by one of `debug`, `output`, `hello` or `counter`.

```shell
cargo make wasm-<name>-kernel
```

This will export the wasm file at the directory `target/wasm32-unknown-unknown/release/<name>_kernel.wasm`.

### Strip the generated WASM

The size of generated wasm file might be large, but [WebAssembly Binary Toolkit (wabt)](https://github.com/WebAssembly/wabt) provides a tool, `wasm-strip`, to strip down the size of our wasm kernel.

Notice that, you need to make sure you have installed `wabt` with your system package manager; and, `wasm-strip` will directly edit the wasm file, so you might want to backup your wasm file.

```shell
wasm-strip target/wasm32-unknown-unknown/release/<name>_kernel.wasm
```

## Unit Test

We use [`wasm-bindgen-test`](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html) to unit test our kernels. To use `wasm-bindgen-test` with Cargo however, you need to install `wasm-bindgen-cli` that will provide you the required test runner.

```shell
cargo install wasm-bindgen-cli
```

then we can test all kernel together by running

```shell
cargo test
```

## `octez-wasm-repl` debug tool for kernel

As REPL (read-eval-print-loop) is an interactive environment, the `octez-wasm-repl` is the tool to evaluate the WASM PVM without running any Tezos node in the background. It has been designed for interact and test the kernel in a local environment.

In the Pistachio-gitbook you can find the tutorial of:
- [How to debug wasm kernels](https://marigold-proto.gitbook.io/proto-gitbook/smart-optimistic-rollup/how-to-mondaynet)

## Interact kernel with SORU

Currently, the MondayNet test is one of the periodic Tezos testnets. More information can be found in <https://teztnets.xyz/mondaynet-about>

In the Pistachio-gitbook, you can find the tutorials of:
- [How to interact with Mondaynet](https://marigold-proto.gitbook.io/proto-gitbook/smart-optimistic-rollup/how-to-mondaynet)
- [How to interact with SORU](https://marigold-proto.gitbook.io/proto-gitbook/smart-optimistic-rollup/how-to-mondaynet) 

## Pistachio-gitbook

[Pistachio-gitbook](https://marigold-proto.gitbook.io/proto-gitbook/).

## Footnotes

The logo of this project is the [Pistachio icons created by Freepik - Flaticon](https://www.flaticon.com/free-icons/pistachio).
