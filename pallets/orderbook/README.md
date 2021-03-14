# Pacific Store Orderbook pallet

The Orderbook pallet provides functionality for posting and managing master data  about NFT orders exchanged in a marketplace between various users. This data is typically posted once by the NFT's buyer / seller to be shared with other network participants.

When this pallet is added to a Subtrate runtime, other custom Substrate pallets can then implement additional business logic leveraging this Orderbook pallet as a reference for known NFTs .

This pallet is part of the [Pacific-store-node](https://github.com/vlbos/pacific-store-node) .

It is inspired by existing projects & standards:
- [Opensea js](https://github.com/ProjectOpenSea/opensea-js)
- [Wyvern Ethereum](https://github.com/ProjectOpenSea/wyvern-js/blob/master/src/wyvern-ethereum/contracts/exchange/ExchangeCore.sol)

NOTE: This pallet implements the aforementioned process in a simplified way, thus it is intended for demonstration purposes and is not audited or ready for production use.

## Usage

### postOrder
To post a order, one must send a transaction with a `orderbook.postOrder` extrinsic with the following arguments:
- `order_id` as the Order ID .
- `owner` as the Substrate Account representing the account owning this order.
- `fields` which is a series of fields (name & value) describing the order. Typically, there would at least be a textual description. It could also contain instance / lot master data e.g. expiration, price.

### postAssetWhiteList
Create a whitelist entry for an asset to prevent others from buying.Buyers will have to have verified at least one of the emails on an asset in order to buy. This will return error code if the given API key isn't allowed to create whitelist entries for this contract or asset.one must send a transaction with a `orderbook.postAssetWhiteList` extrinsic with the following arguments:
- `token_address` as the Address of the asset's contract .
- `token_id` as the The asset's token ID.
- `email`  as the email allowed to buy. 


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

### Runtime `lib.rs`

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

### Genesis Configuration

This template pallet does not have any genesis configuration.

## Reference Docs

You can view the reference docs for this pallet by running:

```
cargo doc --open
```
