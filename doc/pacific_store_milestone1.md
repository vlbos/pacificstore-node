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
## Quickstart

```shell
git clone https://github.com/vlbos/pacific-store-node.git
cd pacific-store-node

cargo run -- --dev --tmp
```

[Custom RPCs](https://github.com/vlbos/pacific-store-node/blob/dev/front-end/src/config/development.json)
[Custom Type](https://github.com/vlbos/pacific-store-node/blob/dev/front-end/src/config/types.json)

start front-end
```bash
cd front-end
yarn start
```

## Milestone 1
### Orderbook Module
#### Orderbook Pallet
##### Functions

- `post_order(origin, order_id: OrderId, owner: T::AccountId, fields: Option<Vec<OrderField>>) -> DispatchResult` : Send an order to the orderbook.
- `post_asset_white_list(origin,token_address: Vec<u8>,token_id: Vec<u8>,email: Vec<u8>,) -> DispatchResult`  -  Create a whitelist entry for an asset to prevent others from buying.Buyers will have to have verified at least one of the emails on an asset in order to buy.

##### RPC Functions
- `get_order( order_query: Option<OrderQuery<T::AccountId>> ) -> Option<OrderJSONType<T::AccountId, T::Moment>>`:Get an order from the orderbook, throwing if none is found.
- `get_orders( order_query: Option<OrderQuery<T::AccountId>>, page: Option<u64>, ) -> Option<Vec<OrderJSONType<T::AccountId, T::Moment>>>`:Get a list of orders from the orderbook, returning the page of orders and the count of total orders found.
- `pub fn get_asset(token_address: Option<Vec<u8>>,token_id: Option<Vec<u8>>,) -> Option<JSONType>`:Fetch an asset from the API, throwing if none is found.
- `pub fn get_assets(asset_query: Option<AssetQuery<T::AccountId>>,page: Option<u64>,) -> Option<Vec<JSONType>> `:Fetch list of assets from the API, returning the page of assets.

##### Storage Define 
```rust
         NextOrderIndex: u64;
        pub Orders get(fn order_by_index): map hasher(blake2_128_concat) u64 => Option<OrderJSONType<T::AccountId, T::Moment>>;
        pub OrderIndices get(fn order_index_by_id): map hasher(blake2_128_concat) OrderId => u64;
        pub OrdersByField get(fn order_index_by_field): double_map hasher(blake2_128_concat) Vec<u8>, hasher(blake2_128_concat) Vec<u8>  => Vec<u64>;
        pub OwnerOf get(fn owner_of): map hasher(blake2_128_concat) OrderId => Option<T::AccountId>;
        pub AssetWhitelist get(fn asset_white_list): double_map hasher(blake2_128_concat) Vec<u8>, hasher(blake2_128_concat) Vec<u8>  => Vec<u8>;

```
##### Event Define 
```rust
        OrderPosted(AccountId, OrderId, AccountId),
        AssetWhiteListPosted(Vec<u8>, Vec<u8>, Vec<u8>),
```
##### Error Define 
```rust
        OrderIdMissing,
        OrderIdTooLong,
        OrderIdExists,
        OrderTooManyFields,
        OrderInvalidFieldName,
        OrderInvalidFieldValue
```

##### Usage

###### postOrder
To post a order, one must send a transaction with a `orderbook.postOrder` extrinsic with the following arguments:
- `order_id` as the Order ID .
- `owner` as the Substrate Account representing the account owning this order.
- `fields` which is a series of fields (name & value) describing the order. Typically, there would at least be a textual description. It could also contain instance / lot master data e.g. expiration, price.

###### postAssetWhiteList
Create a whitelist entry for an asset to prevent others from buying.Buyers will have to have verified at least one of the emails on an asset in order to buy. This will return error code if the given API key isn't allowed to create whitelist entries for this contract or asset.one must send a transaction with a `orderbook.postAssetWhiteList` extrinsic with the following arguments:
- `token_address` as the Address of the asset's contract .
- `token_id` as the The asset's token ID.
- `email`  as the email allowed to buy. 


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
##### Functions

- `change_minimum_maker_protocol_fee( origin, new_minimum_maker_protocol_fee: BalanceOf<T>, ) -> DispatchResult`: Change the minimum maker fee paid to the protocol (only:owner).
- `change_minimum_taker_protocol_fee( origin, new_minimum_taker_protocol_fee: BalanceOf<T>, ) -> DispatchResult`: Change the minimum taker fee paid to the protocol (only:owner).
- `change_protocol_fee_recipient( origin, new_protocol_fee_recipient: T::AccountId, ) -> DispatchResult`: Change the protocol fee recipient (only:owner).
- `approve_order( origin: T::Origin, order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, orderbook_inclusion_desired: bool, ) -> DispatchResult ` : Approve an order and optionally mark it for orderbook inclusion. Must be called by the maker of the order.
- `cancel_order( origin: T::Origin, order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, sig:&[u8], ) -> DispatchResult ` : Cancel an order, preventing it from being matched. Must be called by the maker of the order.
- `atomic_match( msg_sender: T::AccountId, msg_value: BalanceOf<T>, buy: OrderType<T::AccountId, T::Moment, BalanceOf<T>>, buy_sig: Vec<u8>, sell: OrderType<T::AccountId, T::Moment, BalanceOf<T>>, sell_sig: Vec<u8>, metadata: &[u8], ) -> Result<(), Error<T>>`:Atomically match two orders, ensuring validity of the match, and execute all associated state transitions. 
##### RPC Functions
- `hash_order( order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, ) -> Result<Vec<u8>, Error<T>> ` : Hash an order, returning the canonical order hash, without the message prefix
- ` hash_to_sign( order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, ) -> Result<Vec<u8>, Error<T>>` : Hash an order, returning the hash that a client must sign.
- `pub fn require_valid_order(order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,sig: &[u8],) -> Result<Vec<u8>, Error<T>>`: Assert an order is valid and return its hash.
- `validate_order_parameters( order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, ) -> Result<bool, Error<T>>` : Validate order parameters (does _not_ check validity:signature)
- `validate_order( hash: &[u8], order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, sig:&[u8], ) -> Result<bool, Error<T>> ` : Validate a provided previously approved / signed order, hash, and signature.
- `calculate_current_price( order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, ) -> Result<BalanceOf<T>, Error<T>>` : Calculate the current price of an order (fn:convenience).
- `calculate_match_price( buy: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, sell: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, ) -> Result<BalanceOf<T>, Error<T>>` : Calculate the price two orders would match at, if in fact they would match (fail:otherwise).
- `orders_can_match( buy: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, sell: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, ) -> Result<bool, Error<T>>` :Return whether or not two orders can be matched with each other by basic parameters (does not check order signatures / calldata or perform calls:static).
- `calculate_final_price( side: &Side, sale_kind: &SaleKind, base_price: BalanceOf<T>, extra: T::Moment, listing_time: T::Moment, expiration_time: T::Moment, ) -> Result<BalanceOf<T>, Error<T>> `:
  Calculate the settlement price of an order;Precondition: parameters have passed validate_parameters.


##### Storage Define
```rust
        NextOrderIndex: BalanceOf<T>;
        pub ContractSelf:T::AccountId;
        //The token used to pay exchange fees.
        pub ExchangeToken:T::AccountId;
        //Cancelled / finalized orders, by hash.
        pub CancelledOrFinalized get(fn cancelled_or_finalized): map hasher(blake2_128_concat) Vec<u8> => bool;
        //Orders verified by on-chain approval (alternative to ECDSA signatures so that smart contracts can place orders directly).
        pub ApprovedOrders get(fn approved_orders): map hasher(blake2_128_concat) Vec<u8> => bool;
        //For split fee orders, minimum required protocol maker fee, in basis points. Paid to owner (who can change it).
        pub MinimumMakerProtocolFee:BalanceOf<T>;
        //For split fee orders, minimum required protocol taker fee, in basis points. Paid to owner (who can change it).
        pub MinimumTakerProtocolFee:BalanceOf<T>;
        //Recipient of protocol fees.
        pub ProtocolFeeRecipient:T::AccountId;
```
##### Event Define
```rust
  OrderApprovedPartOne(
            Vec<u8>,
            AccountId,
            AccountId,
            AccountId,
            Balance,
            Balance,
            Balance,
            Balance,
            AccountId,
            FeeMethod,
            Side,
            SaleKind,
            AccountId,
        ),
        OrderApprovedPartTwo(
            Vec<u8>,
            HowToCall,
            Vec<u8>,
            Vec<u8>,
            AccountId,
            Vec<u8>,
            AccountId,
            Balance,
            Moment,
            Moment,
            Moment,
            u64,
            bool,
        ),
        OrderCancelled(Vec<u8>),
        OrdersMatched(Vec<u8>, Vec<u8>, AccountId, AccountId, Balance, Vec<u8>),
        MinimumMakerProtocolFeeChanged(Balance),
        MinimumTakerProtocolFeeChanged(Balance),
        ProtocolFeeRecipientChanged(AccountId, AccountId),
```

##### Error Define
```rust
    pub enum Error for Module<T: Trait> {
        MsgVerifyFailed,
        InvalidBuyOrderParameters,
        InvalidSellOrderParameters,
        OrdersCannotMatch,
        ListingTimeExpired,
        ArrayNotEqual,
        ArraySizeNotAsSameAsDesired,
        ArraySizeNotAsSameAsMask,
        BuyTakerProtocolFeeGreaterThanSellTakerProtocolFee,
        BuyTakerRelayerFeeGreaterThanSellTakerRelayerFee,
        SellPaymentTokenEqualPaymentToken,
        SellTakerProtocolFeeGreaterThanBuyTakerProtocolFee,
        SellTakerRelayerFeeGreaterThanBuyTakerRelayerFee,
        ValueLessThanRequiredAmount,
        ValueNotZero,
        BuyPriceLessThanSellPrice,
        OrderHashMissing,
        OnlyMaker,
        InvalidOrderHash,
    }
```

##### Usage
###### approveOrder
To approve a order, one must send a transaction with a `wyvernExchange.approveOrder` extrinsic with the following arguments:
- `order` as the orderType to approve.
- `orderbook_inclusion_desired` Whether orderbook providers should include the order in their orderbooks.

###### cancelOrder
To cancel a order, one must send a transaction with a `wyvernExchange.cancelOrder` extrinsic with the following arguments:
- `order` as the orderType to cancel.
- `sig` signature.

###### atomicMatch
To atomically match two orders, ensuring validity of the match, and execute all associated state transitions, one must send a transaction with a `wyvernExchange.atomicMatch` extrinsic with the following arguments:
- `msg_sender` as the orderType to cancel.
- `msg_value` as the balance.
- `buy` as the buy-side order  orderType .
- `buy_sig` as the buy-side order signature.
- `sell` as the sell-side order  orderType .
- `sell_sig` as the sell-side order signature.

###### changeMinimumMakerProtocolFee
To change the minimum maker fee paid to the protocol, one must send a transaction with a `wyvernExchange.changeMinimumMakerProtocolFee` extrinsic with the following arguments:
- `new_minimum_maker_protocol_fee` as the new fee to set in basis points.

###### changeMinimumTakerProtocolFee
To change the minimum taker fee paid to the protocol, one must send a transaction with a `wyvernExchange.changeMinimumTakerProtocolFee` extrinsic with the following arguments:
- `new_minimum_taker_protocol_fee` as the new fee to set in basis points.

###### changeProtocolFeeRecipient
To change the protocol fee recipient, one must send a transaction with a `wyvernExchange.changeProtocolFeeRecipient` extrinsic with the following arguments:
- `new_protocol_fee_recipient` as the new protocol fee recipient AccountId.

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

###### Genesis Configuration

This pallet does not have any genesis configuration.

##### Reference Docs

You can view the reference docs for this pallet by running:

```
cargo doc --open
```

### Test Project

In order to help develop this pallet, it is being consumed by
[a test project](https://github.com/vlbos/pacific-store-node/tree/dev/front-end) .

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