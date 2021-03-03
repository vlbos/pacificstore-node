```
cd substrate-enterprise-sample/chain/pallets/wyvern-exchange

SKIP_WASM_BUILD=1 cargo check
```

```
SKIP_WASM_BUILD=1 cargo test -- --nocapture
```

```
rustfmt src/lib.rs 
```

```
catgo fmt

cargo clippy
```


1. Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
sudo apt-get install libssl-dev pkg-config libclang-dev clang
```

2. Remove all installed toolchains with `rustup toolchain list` and `rustup toolchain uninstall <toolchain>`.

3. Install Rust Toolchain 1.44.0:

```bash
rustup install 1.44.0
```

4. Make it default (actual toolchain version may be different, so do a `rustup toolchain list` first)
```bash
rustup toolchain list
rustup default 1.44.0-x86_64-unknown-linux-gnu
```

5. Install nightly toolchain and add wasm target for it:

```bash
rustup toolchain install nightly-2020-05-01
rustup target add wasm32-unknown-unknown --toolchain nightly-2020-05-01-x86_64-unknown-linux-gnu
```

6. Build:
```bash
cargo build
```
```bash
 export CARGO_HTTP_MULTIPLEXING=false
```
## Run

You can start a development chain with:

```bash

cargo run -- --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev --tmp`.

```bash
purge-chain

./target/release/xxx purge-chain --dev

export CARGO_HTTP_MULTIPLEXING=false

cargo run --release -- --dev --tmp
cargo run -- --dev --tmp


[substrate/frame/babe/src/equivocation.rs]
(https://polkadot.js.org/apps/#/extrinsics?rpc=ws://127.0.0.1:9944)

```


### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/pacific-store --dev
```

Purge the development chain's state:

```bash
./target/release/pacific-store purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/pacific-store -lruntime=debug --dev
```