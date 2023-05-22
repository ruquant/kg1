# Smart Rollup Kernel Gallery

[[_TOC_]]

The kernel gallery is a directory of examples to help you get started writing your own WASM kernels for [Tezos Smart Rollups](http://tezos.gitlab.io/alpha/smart_rollups.html).

The kernel gallery is a direcory of examples to help you get started writing
your own WASM kernels for [Tezos Smart Rollups](http://tezos.gitlab.io/alpha/smart_rollups.html).

This repository is intended as companion to the docs on [developing your wasm kernel](http://tezos.gitlab.io/alpha/smart_rollups.html#developing-wasm-kernels). Additionally, it showcases
simple end-to-end rollup applications, demonstrating how you can use rollups in your DApps.

We recommend going through examples in order:
- **00_debug_kernel**: shows how to debug messages and read from the shared inbox.
- **01_storage_kernel**: shows how to read and write to the kernel's persistent storage.
- **02_reboot_kernel**: shows how to mark a kernel reboot and discusses kernel control flow.
- **03_inbox_kernel**: shows how to read from the shared inbox.
- **04_filtering_kernel**: shows how to filter messages coming from the shared inbox.
- **05_outbox_kernel**: shows how to write messages to the outbox to communicate back to the L1.
- **06_counter_kernel**: a larger example combining the above elements into a simple counter application.
- **09_tzwitter_app**: A full fledged rollup DApp for social media, combining an L1 smart contract, rollup kernel, React+Typescript frontend, and deployment script.

Each kernel directory includes a README.md that demonstrates how to test the kernel with the `octez-smart-rollup-wasm-debugger` against a set of inputs and commands. The expected outputs are included in the README.

## Setup

To build the kernels, you will need the Rust toolchain with WASM support installed, detailed below.

To run the `octez-smart-rollup-wasm-debugger`, you will need to install it [from OPAM](https://opam.ocaml.org/packages/octez-smart-rollup-wasm-debugger/).

Alternatively, Nix users can activate a shell with the required dependencies with `nix develop`.

### Setup Rust

(Mac users, don't miss the section for you further down!)

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
rustup override set 1.66.0-<channel_full_name>op
rustup toolchain install 1.66.0
```

More details of install Rust can be found at: https://www.rust-lang.org/tools/install.

### Setup WASM

We need to add `wasm32-unknown-unknown` to be a possible target of Rust:

```shell
rustup target add wasm32-unknown-unknown
```

#### Additional setup for Mac

The Apple `clang` compiler installed by default does not support the `wasm32` target (check with `clang --print-targets`).
You need to install a version which does, such as the one available through Homebrew:

```shell
brew install llvm
```

You also need to add this version at the beginning of your `PATH`. For instance, on an Apple Silicon Mac, this can be done using:

```shell
export PATH="/opt/homebrew/opt/llvm/bin/:$PATH"
```

If you installed `clang` via Homebrew, you can obtain the correct path using `brew --prefix llvm`.

In addition, you might also need to manually set the following environment variables, where `LLVM_PATH` is the same path as above:

```shell
export AR="${LLVM_PATH}/bin/llvm-ar"
export CC="${LLVM_PATH}/bin/clang"
```

## Build the WASM kernels

You can build all the kernels with Cargo. In order to build the `09_tzwitter_app` kernel you also need to set this environment variable (the value itself is not important).

```shell
export TZWITTER_L1_CONTRACT="KT1..."
cargo build --release --target wasm32-unknown-unknown
```

Alternatively, you can build using `cargo-make`:

```shell
cargo make wasm
```

### Strip the generated WASM

The size of generated wasm file might be large, but [WebAssembly Binary Toolkit (wabt)](https://github.com/WebAssembly/wabt) provides a tool, `wasm-strip`, to strip down the size of our wasm kernel.

Notice that, you need to make sure you have installed `wabt` with your system package manager; and, `wasm-strip` will directly edit the wasm file, so you might want to backup your wasm file.

```shell
wasm-strip target/wasm32-unknown-unknown/release/<name>_kernel.wasm
```


## Tests

Each kernel comes with tests and tests of the README, defined as [`cargo-make`](https://github.com/sagiegurari/cargo-make) tasks.
You can install `cargo-make` like so:

```shell
cargo install cargo-make
```

## Octez Smart Rollup WASM Debugger

The Octez software system includes an interactive debugger for Smart Rollup kernels, documented [here](https://tezos.gitlab.io/alpha/smart_rollups.html#testing-your-kernel). 
Each kernel's README includes examples how to use it.

## Deployment

Refer to [the docs](https://tezos.gitlab.io/alpha/smart_rollups.html#deploying-a-rollup-node). Additionally, 
you can look at an example deployment script in `./09_tzwitter_app/deploy.sh`.

## Sequencer

A first version of the sequencer has been developed in this repository. If you want to learn more, please take a look to the [readme](./09_rust_sequencer/readme.md).
