# Contract

## Pre-requisites

- [Rust](https://www.rust-lang.org/tools/install)

```bash
rustup component add rust-src
cargo install --force --locked cargo-contract --version 3.2.0
```

## Test Node

Start a testnode with the following command:

```bash
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --tag v0.30.0 --force
substrate-contracts-node --base-path chain
```

## Build the Contract

```bash
cd contract
cargo contract build --release
```

## Test the Contract

```bash
cargo test --release --features e2e-tests
```