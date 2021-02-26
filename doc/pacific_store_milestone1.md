# Pacific Store Orderbook&WyvernExchange Modules 


When it comes to the rise of crypto collectables, it’s no surprise that many secondary markets have arisen for the exchange of these unique assets.

For those unfamiliar with non-fungible tokens (better known as NFTs), they are tokens which are unique from one another due to special characteristics or identifiers which distinguish them from others in a similar set.

Specifically within DeFi, we’ve seen NFTs play a role in the tokenization of assets and contracts alike, all of which play a larger role in the notion of composability – or different products interacting with one another towards a larger ecosystem.

In this overview, we’ll dive into PacificStore – an industry-leading decentralized exchange (DEX) for NFTs built on Polkadot/Kusama.

PacificStore is an open-sourced crypto collectible marketplace which allows users to trade popular tokens built on interoperable standards like .

Prior to PacificStore, there was no easy way to trade crypto collectables other than brokering over-the-counter (OTC) deals – a situation which invokes a large amount of trust and counterparty risk.

With PacificStore, users can bid directly for items owned by other users, with assets being exchanged instantaneously in the event the seller accepts a bid or a buyer accepts an offer.

While there are a number of other crypto collectible marketplaces on the market, PacificStore currently leads as the defacto choice for swapping NFTs.

## Milestone 1

### Orderbook Module

#### Orderbook Pallet

##### Functions

- `post_order(origin, order_id: OrderId, owner: T::AccountId, fields: Option<Vec<OrderField>>) -> DispatchResult` : Send an order to the orderbook.

- `get_orders( order_query: Option<OrderQuery<T::AccountId>>, page: Option<u64>, ) -> Option<Vec<OrderJSONType<T::AccountId, T::Moment>>>`:Get a list of orders from the orderbook, returning the page of orders
  and the count of total orders found.

### WyvernExchange Module

#### WyvernExchange Pallet


##### Functions

- `change_minimum_maker_protocol_fee( origin, new_minimum_maker_protocol_fee: BalanceOf<T>, ) -> DispatchResult`: Change the minimum maker fee paid to the protocol (only:owner)

- `change_minimum_taker_protocol_fee( origin, new_minimum_taker_protocol_fee: BalanceOf<T>, ) -> DispatchResult`: Change the minimum taker fee paid to the protocol (only:owner)

- `change_protocol_fee_recipient( origin, new_protocol_fee_recipient: T::AccountId, ) -> DispatchResult`: Change the protocol fee recipient (only:owner)

- `hash_order( order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, ) -> Result<Vec<u8>, Error<T>> ` : Hash an order, returning the canonical order hash, without the message prefix

- ` hash_to_sign( order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, ) -> Result<Vec<u8>, Error<T>>` : Hash an order, returning the hash that a client must sign.

- `validate_order_parameters( order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, ) -> Result<bool, Error<T>>` : Validate order parameters (does _not_ check validity:signature)

- `validate_order( hash: &[u8], order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, sig: &Signature, ) -> Result<bool, Error<T>> ` : Validate a provided previously approved / signed order, hash, and signature.

- `approve_order( origin: T::Origin, order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, orderbook_inclusion_desired: bool, ) -> DispatchResult ` : Approve an order and optionally mark it for orderbook inclusion. Must be called by the maker of the order

- `cancel_order( origin: T::Origin, order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, sig: &Signature, ) -> DispatchResult ` : Cancel an order, preventing it from being matched. Must be called by the maker of the order

- `calculate_current_price( order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, ) -> Result<BalanceOf<T>, Error<T>>` : Calculate the current price of an order (fn:convenience)

- `calculate_match_price( buy: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, sell: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, ) -> Result<BalanceOf<T>, Error<T>>` : Calculate the price two orders would match at, if in fact they would match (fail:otherwise).

- `orders_can_match( buy: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, sell: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>, ) -> Result<bool, Error<T>>` :Return whether or not two orders can be matched with each other by basic parameters (does not check order signatures / calldata or perform calls:static).

- `atomic_match( msg_sender: T::AccountId, msg_value: BalanceOf<T>, buy: OrderType<T::AccountId, T::Moment, BalanceOf<T>>, buy_sig: Signature, sell: OrderType<T::AccountId, T::Moment, BalanceOf<T>>, sell_sig: Signature, metadata: &[u8], ) -> Result<(), Error<T>>`:Atomically match two orders, ensuring validity of the match, and execute all associated state transitions. Protected against reentrancy by a contract-global lock.

- `calculate_final_price( side: &Side, sale_kind: &SaleKind, base_price: BalanceOf<T>, extra: T::Moment, listing_time: T::Moment, expiration_time: T::Moment, ) -> Result<BalanceOf<T>, Error<T>> `:
  Calculate the settlement price of an order;
  Precondition: parameters have passed validate_parameters.


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



NOTE: This pallet implements the aforementionned process in a simplified way, thus it is intended for demonstration purposes and is not audited or ready for production use.

## Upstream

This project was forked from
[the Substrate DevHub Node Template](https://github.com/substrate-developer-hub/substrate-node-template).
[the Substrate DevHub Front-end Template](https://github.com/substrate-developer-hub/substrate-front-end-template)
[Substrate Enterprise Sample](https://github.com/substrate-developer-hub/substrate-enterprise-sample)