# PacificStore Orderbook & WyvernExchange Modules 
## Introduce
When it comes to the rise of crypto collectables, it’s no surprise that many secondary markets have arisen for the exchange of these unique assets.
For those unfamiliar with non-fungible tokens (better known as NFTs), they are tokens which are unique from one another due to special characteristics or identifiers which distinguish them from others in a similar set.
Specifically within DeFi, we’ve seen NFTs play a role in the tokenization of assets and contracts alike, all of which play a larger role in the notion of composability – or different products interacting with one another towards a larger ecosystem.
In this overview, we’ll dive into PacificStore – an Decentralized NFT Exchange Aggregation Platform built on Polkadot/Kusama.
PacificStore is an open-sourced crypto unique asset marketplace which allows users to trade popular tokenized assets and collectibles built on interoperable standards.
PacificStore is inspired by **OpenSea** but implements **Substrate** runtime module.PacificStore js part uses polkadot.js for interacting With Substrate RPC of Orderbook and WyvernExchange pallet of PacificStore.

## Overview
For this milestone we have provided the following components:
    1. Orderbook pallet
    2. WyvernExchange pallet
    3. PacificStore-node
    4. Test Project
Orderbook pallet implements the backend of Opensea.js based on Substrate.It provides functionalities for posting order,posting asset whitelist.
WyvernExchange pallet implements the Exchange and ExchangeCore part of Wyvern protocol .It provides functionalities for hashing order,validating order,approving order,cancelling order,etc.
PacificStore-node based on Substrate-node-template,integrates Orderbook pallet , WynvernExchange pallet and both custom RPCs.
Test Project based on Substate-front-end-template,In order to demonstrate the usage of the two pallets.It includes custom types and RPCs.
## Milestone 1
### Orderbook Module
#### Orderbook Pallet
##### Dependencies
###### Traits
This pallet depends on on the [FRAME EnsureOrigin System trait]
```
frame_support::traits::EnsureOrigin;
```

###### Pallets
This pallet depends on on the [FRAME Timestamp pallet](https://docs.rs/crate/pallet-timestamp).

##### Testing
Run the tests with:
    ```
    cargo test
    ```
##### How to use in your runtime
###### Runtime `Cargo.toml`
To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.orderbook]
default_features = false
package = 'pallet-orderbook'
version = '2.0.0'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'orderbook/std',
]
```

###### Runtime `lib.rs`
You should implement it's trait like so:

```rust
impl orderbook::Trait for Runtime {
	type Event = Event;
}
```

and include it in your `construct_runtime!` macro:

```rust
Orderbook: orderbook::{Module, Call, Storage, Event<T>},
```

###### Genesis Configuration
This  pallet does not have any genesis configuration.

##### Reference Docs
You can view the reference docs for this pallet by running:

```
cargo doc --open
```


### WyvernExchange Module
#### WyvernExchange Pallet
##### Dependencies
###### Traits
This pallet depends on on the [FRAME EnsureOrigin System trait]
```
frame_support::traits::EnsureOrigin;
```

###### Pallets
This pallet depends on on the [FRAME Timestamp pallet](https://docs.rs/crate/pallet-timestamp),**WyvernExchangeCore**.

##### Testing
Run the tests with:
```
cargo test
```

##### How to use in your runtime
###### Runtime `Cargo.toml`
To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.wyvern-exchange]
default_features = false
package = 'pallet-wyvern-exchange'
version = '2.0.0'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'wyvern-exchange/std',
]
```

###### Runtime `lib.rs`
You should implement it's trait like so:
```rust
impl wyvern_exchange::Trait for Runtime {
}
```

and include it in your `construct_runtime!` macro:

```rust
WyvernExchange: wyvern_exchange::{Module, Call, Storage},
```

#### WyvernExchangeCore Pallet
##### Dependencies
###### Traits
This pallet depends on on the [FRAME EnsureOrigin System trait]
```
frame_support::traits::EnsureOrigin;
```

###### Pallets
This pallet depends on on the [FRAME Timestamp pallet](https://docs.rs/crate/pallet-timestamp).

##### Testing
Run the tests with:
```
cargo test
```

##### How to use in your runtime
###### Runtime `Cargo.toml`

To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.wyvern-exchange-core]
default_features = false
package = 'pallet-wyvern-exchange-core '
version = '2.0.0'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'wyvern-exchange-core/std',
]
```

###### Runtime `lib.rs`
You should implement it's trait like so:
```rust

impl wyvern_exchange_core::Trait for Runtime {
}

impl wyvern_exchange_core::exchange_common::Trait for Runtime {
	type Currency = Balances;
}

impl wyvern_exchange_core::sale_kind_interface::Trait for Runtime {
}
impl wyvern_exchange_core::Trait for Runtime {
    type Event = Event;
    type Public = MultiSigner;
    type Signature = Signature;
}
```

and include it in your `construct_runtime!` macro:

```rust
WyvernExchangeCore: wyvern_exchange_core::{Module, Call, Storage, Event<T>},
```


## Testing Guide

```shell
git clone https://github.com/vlbos/pacificstore-node.git
cd pacificstore-node

cargo run -- --dev --tmp
```

[Custom RPCs](https://github.com/vlbos/pacificstore-node/blob/dev/front-end/src/config/development.json)
[Custom Type](https://github.com/vlbos/pacificstore-node/blob/dev/front-end/src/config/types.json)

start front-end
```bash
cd front-end
yarn install
yarn start
```

Test example
```bash
cd front-end/scripts
yarn install
yarn example
```

### Runtime Module Unit Test
##### Testing
Run the tests with:
```
git clone https://github.com/vlbos/pacificstore-node.git
cd pacificstore-node
cargo test
```

### RPC API Example
##### Example
Run the Node
```shell
git clone https://github.com/vlbos/pacificstore-node.git
cd pacificstore-node

cargo run -- --dev --tmp
```

Run the examples with:
```
cd front-end/scripts
yarn install
yarn examples 
```

###### Genesis Configuration
This pallet does not have any genesis configuration.

##### Reference Docs
You can view the reference docs for this pallet by running:

```
cargo doc --open
```

### Test Project
In order to help develop this pallet, it is being consumed by
[a test project](https://github.com/vlbos/pacificstore-node/tree/dev/front-end) .
[a test example document](https://github.com/vlbos/pacificstore-node/tree/dev/doc/pacific_store_milestone1_js_example) .
### Build & Run

First, build & run the node:

```shell
cargo run -- --dev --tmp
```

## Acknowledgements

It is inspired by existing projects & standards:

- [Opensea js](https://github.com/ProjectOpenSea/opensea-js)
- [Wyvern Ethereum](https://github.com/ProjectOpenSea/wyvern-js/blob/master/src/wyvern-ethereum/contracts/exchange/ExchangeCore.sol)



NOTE: This pallet implements the aforementioned process in a simplified way, thus it is intended for demonstration purposes and is not audited or ready for production use.

## Upstream

This project was forked from
- [the Substrate DevHub Node Template](https://github.com/substrate-developer-hub/substrate-node-template).
- [the Substrate DevHub Front-end Template](https://github.com/substrate-developer-hub/substrate-front-end-template)
- [Substrate Enterprise Sample](https://github.com/substrate-developer-hub/substrate-enterprise-sample)