# SORU Kernel Gallery

[[_TOC_]]


The kernel gallery is a direcory of examples to help you get started writing
your own WASM kernels for [Tezos SORU](http://tezos.gitlab.io/alpha/smart_rollups.html).

This repository is intended as companion to the docs on [developing your wasm kernel](http://tezos.gitlab.io/alpha/smart_rollups.html#developing-wasm-kernels). Additionally, it showcases
simple end-to-end rollup applications, demonstrating how you can use rollups in your DApps.

We recommend going through examples in order:
- **00_debug_kernel**: shows how to debug messages and read from the shared inbox.
- **01_counter_kernel**: shows a simple rollup that tracks how many times users have called in in its persistent storage. Additionally introductes the `mock_host` testing fixtures.
- **02_tzwitter**: a twitter clone demonstrating a full rollup DApp.

Each kernel directory includes a README.md that demonstrates how to test the kernel
with the `octez-smart-rollup-wasm-debugger` against a set of inputs and commands. The
expected outputs are included in the README and checked in CI with [MDX](https://github.com/realworldocaml/mdx).

## Setup

To build the kernels, you will need the Rust toolchain with WASM support installed, detailed below.

To run the `octez-smart-rollup-wasm-debugger`, you will need to install it [from OPAM](https://opam.ocaml.org/packages/octez-smart-rollup-wasm-debugger/).

Alternatively, Nix users can activate a shell with the required dependencies with `nix develop`.

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


<!-- TODO: I haven't finished editing past this point:-->
## Tests

We provide pre-defined tasks for building kernels, that requires [`cargo-make`](https://github.com/sagiegurari/cargo-make):

```shell
cargo install cargo-make
```

After install `cargo-make` we can now build our kernel! Remember to replace `<name>` by one of `debug`, `output`, `hello` or `counter`.

```shell
cargo make wasm-<name>-kernel
```

This will export the wasm file at the directory `target/wasm32-unknown-unknown/release/<name>_kernel.wasm`.



## Unit Test

We use [`wasm-bindgen-test`](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/usage.html) to unit test our kernels. To use `wasm-bindgen-test` with Cargo however, you need to install `wasm-bindgen-cli` that will provide you the required test runner.

```shell
cargo install wasm-bindgen-cli
```

then we can test all kernel together by running

```shell
cargo test
```

## Debug tool for kernel

As REPL (read-eval-print-loop) is an interactive environment, the `octez-wasm-repl` is the tool to evaluate the WASM PVM without running any Tezos node in the background. It has been designed for interact and test the kernel in a local environment.

In the Pistachio-gitbook you can find the tutorial at [How to debug wasm kernels](doc/how-to-debug-kernels.md)

## Interact kernel with SORU

Currently, the MondayNet test is one of the periodic Tezos testnets. More information can be found in <https://teztnets.xyz/mondaynet-about>

You can find more details at:

- [How to interact with Mondaynet](doc/how-to-mondaynet.md)
- [How to interact with SORU](doc/how-to-soru.md)
