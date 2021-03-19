# Pacific Store Wyvern Exchange  pallet

The Product Registry pallet provides functionality for registering and managing master data (aka class-level) about products / trade items exchanged in a supply chain between various stakeholders. This data is typically created_date once by the product's manufacturer / supplier to be shared with other network participants.

When this pallet is added to a Subtrate runtime, other custom Substrate pallets can then implement additional business logic leveraging this Product Registry pallet as a reference for known products and their owning organizations.

This pallet is part of the [Pacific-store-node](https://github.com/vlbos/pacific-store-node).

It is inspired by existing projects & standards:
- [Opensea js](https://github.com/ProjectOpenSea/opensea-js)
- [Wyvern Ethereum](https://github.com/ProjectOpenSea/wyvern-js/blob/master/src/wyvern-ethereum/contracts/exchange/ExchangeCore.sol)

NOTE: This pallet implements the aforementioned process in a simplified way, thus it is intended for demonstration purposes and is not audited or ready for production use.

## Usage
### approveOrder
To approve a order, one must send a transaction with a `wyvernExchange.approveOrder` extrinsic with the following arguments:
- `order` as the orderType to approve.
- `orderbook_inclusion_desired` Whether orderbook providers should include the order in their orderbooks.

### cancelOrder
To cancel a order, one must send a transaction with a `wyvernExchange.cancelOrder` extrinsic with the following arguments:
- `order` as the orderType to cancel.
- `sig` signature.

### atomicMatch
To atomically match two orders, ensuring validity of the match, and execute all associated state transitions, one must send a transaction with a `wyvernExchange.atomicMatch` extrinsic with the following arguments:
- `msg_sender` as the orderType to cancel.
- `msg_value` as the balance.
- `buy` as the buy-side order  orderType .
- `buy_sig` as the buy-side order signature.
- `sell` as the sell-side order  orderType .
- `sell_sig` as the sell-side order signature.

### changeMinimumMakerProtocolFee
To change the minimum maker fee paid to the protocol, one must send a transaction with a `wyvernExchange.changeMinimumMakerProtocolFee` extrinsic with the following arguments:
- `new_minimum_maker_protocol_fee` as the new fee to set in basis points.

### changeMinimumTakerProtocolFee
To change the minimum taker fee paid to the protocol, one must send a transaction with a `wyvernExchange.changeMinimumTakerProtocolFee` extrinsic with the following arguments:
- `new_minimum_taker_protocol_fee` as the new fee to set in basis points.

### changeProtocolFeeRecipient
To change the protocol fee recipient, one must send a transaction with a `wyvernExchange.changeProtocolFeeRecipient` extrinsic with the following arguments:
- `new_protocol_fee_recipient` as the new protocol fee recipient AccountId.

## Dependencies

### Traits

This pallet depends on on the [FRAME EnsureOrigin System trait]
```
frame_support::traits::EnsureOrigin;
```

### Pallets

This pallet depends on on the [FRAME Timestamp pallet](https://docs.rs/crate/pallet-timestamp).

## Testing

Run the tests with:

    ```
    cargo test
    ```

## How to use in your runtime

### Runtime `Cargo.toml`

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

### Runtime `lib.rs`

You should implement it's trait like so:

```rust

impl wyvern_exchange::Trait for Runtime {
}

impl wyvern_exchange::exchange_common::Trait for Runtime {
	type Currency = Balances;
}

impl wyvern_exchange::sale_kind_interface::Trait for Runtime {
}
impl wyvern_exchange::exchange_core::Trait for Runtime {
    type Event = Event;
    type Public = MultiSigner;
    type Signature = Signature;
}
```

and include it in your `construct_runtime!` macro:

```rust
WyvernExchange: wyvern_exchange::{Module, Call, Storage, Event<T>},
```

### Genesis Configuration

This template pallet does not have any genesis configuration.

## Reference Docs

You can view the reference docs for this pallet by running:

```
cargo doc --open
```
