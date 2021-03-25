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
## Custom Types for JS API
### OrderBook Module

```
  "OrderId": "Vec<u8>",
  "TokenId": "Vec<u8>",
  "FieldName": "Vec<u8>",
  "FieldValue": "Vec<u8>",
  "OrderField": {
    "name": "FieldName",
    "value": "FieldValue"
  },
  "JSONField": {
    "name": "FieldName",
    "json": "Option<Vec<OrderField>>"
  },
  "OrderJSONType": {
    "index": "u64",
    "id": "OrderId",
    "owner": "AccountId",
    "fields": "Option<Vec<OrderField>>",
    "created_date": "Moment"
  },
  "OrderQuery": {
    "limit": "Option<u64>",
    "offset": "Option<u64>",
    "owner": "Option<AccountId>",
    "token_ids": "Option<Vec<TokenId>>",
    "params": "Option<Vec<OrderField>>"
  },
  "AssetQuery": {
    "owner": "Option<AccountId>",
    "asset_contract_address": "Option<Vec<u8>>",
    "token_ids": "Option<Vec<TokenId>>",
    "search": "Option<Vec<u8>>",
    "order_by": "Option<Vec<u8>>",
    "order_direction": "Option<Vec<u8>>",
    "limit": "Option<u64>",
    "offset": "Option<u64>"
  },
  "QueryParameter": {
    "name": "String",
    "value": "String"
  },
  "OrderQueryJSON": {
    "limit": "Option<u64>",
    "offset": "Option<u64>",
    "owner": "Option<AccountId>",
    "token_ids": "Option<Vec<String>>",
    "params": "Option<Vec<QueryParameter>>"
  },
  "AssetQueryJSON": {
    "owner": "Option<AccountId>",
    "asset_contract_address": "Option<String>",
    "token_ids": "Option<Vec<String>>",
    "search": "Option<String>",
    "order_by": "Option<String>",
    "order_direction": "Option<String>",
    "limit": "Option<u64>",
    "offset": "Option<u64>"
  },
  "JSONType": {
    "fields": "Option<Vec<OrderField>>",
    "jsons": "Option<Vec<JSONField>>"
  },
```

### WyvernExchange Module

```

  "Side": {
    "_enum": ["Buy", "Sell"]
  },
  "SaleKind": {
    "_enum": ["FixedPrice", "DutchAuction"]
  },
  "FeeMethod": {
    "_enum": ["ProtocolFee", "SplitFee"]
  },
  "HowToCall": {
    "_enum": ["Call", "DelegateCall"]
  },
  
```

[Custom Type](https://github.com/vlbos/pacific-store-node/blob/dev/front-end/src/config/types.json)

[Custom RPCs](https://github.com/vlbos/pacific-store-node/blob/dev/front-end/src/config/development.json)



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


## Orderbook Module
### Orderbook Pallet Methods
#### PostOrder
##### Description
To post a order, one must send a transaction with a `orderbook.postOrder` extrinsic with the following arguments:

##### Permissions
Anyone

##### Parameters
- `order_id` as the Order ID .
- `owner` as the Substrate Account representing the account owning this order.
- `fields` which is a series of fields (name & value) describing the order. Typically, there would at least be a textual description. It could also contain instance / lot master data e.g. expiration, price.
  
##### Events
OrderPosted
- `Account`  Submit order account.
- `OrderID`  Globally unique identifier of newly created collection.
- `Owner`  Order owner.

#### PostAssetWhiteList
##### Description
Create a whitelist entry for an asset to prevent others from buying.Buyers will have to have verified at least one of the emails on an asset in order to buy. This will return error code if the given API key isn't allowed to create whitelist entries for this contract or asset.one must send a transaction with a `orderbook.postAssetWhiteList` extrinsic .


##### Permissions
Asset Owner

##### Parameters
- `token_address` as the Address of the asset's contract .
- `token_id` as the The asset's token ID.
- `email`  as the email allowed to buy.
##### Events
AssetWhiteListPosted
- `token_address` as the Address of the asset's contract .
- `token_id` as the The asset's token ID.
- `email`  as the email allowed to buy.


##### RPC Functions
- `get_order( order_query: Option<OrderQuery<T::AccountId>> ) -> Option<OrderJSONType<T::AccountId, T::Moment>>`:Get an order from the orderbook, throwing if none is found.
- `get_orders( order_query: Option<OrderQuery<T::AccountId>>, page: Option<u64>, ) -> Option<Vec<OrderJSONType<T::AccountId, T::Moment>>>`:Get a list of orders from the orderbook, returning the page of orders and the count of total orders found.
- `pub fn get_asset(token_address: Option<Vec<u8>>,token_id: Option<Vec<u8>>,) -> Option<JSONType>`:Fetch an asset from the API, throwing if none is found.
- `pub fn get_assets(asset_query: Option<AssetQuery<T::AccountId>>,page: Option<u64>,) -> Option<Vec<JSONType>> `:Fetch list of assets from the API, returning the page of assets.



### WyvernExchange Module
#### WyvernExchange Pallet Functions

#### approveOrderEx
##### Description
To approve a order, one must send a transaction with a `wyvernExchange.approveOrder` extrinsic.

##### Permissions
Order owner

##### Parameters
- `addrs` as the array of order's fields in **AccoundId** type .
- `uints` as the array of order's fields in integer/Balance type
- `fee_method` as Fee method: protocol fee or split fee.
- `side` as Side: buy or sell.
- `sale_kind` as Currently supported kinds of sale: fixed price, Dutch auction. 
- `how_to_call` as  Call or DelegateCall.
- `calldata` as  order calldata
- `replacement_pattern` as replacement mask
- `static_extradata`as order  extradata
- `orderbook_inclusion_desired` Whether orderbook providers should include the order in their orderbooks.

##### Events
- OrderApprovedPartOne
order JSON:Order JSON.
- `hash` as the order hash
- `exchange` as Exchange AccountId, intended as a versioning mechanism.
- `maker` as maker AccountId.
- `taker` as taker AccountId, if specified.
- `maker_relayer_fee` as Maker relayer fee of the order, unused for taker order.
- `taker_relayer_fee` as Taker relayer fee of the order, or maximum taker fee for a taker order.
- `maker_protocol_fee` as  Maker protocol fee of the order, unused for taker order.
- `taker_protocol_fee` as Taker protocol fee of the order, or maximum taker fee for a taker order.
- `fee_recipient`  as  OrderType fee recipient or zero AccountId for taker order.
- `fee_method` as  Fee method (protocol token or split fee).
- `side` as  Side (buy/sell).
- `sale_kind` as  Kind of sale.
- `target` as  order's target
- OrderApprovedPartTwo
order JSON:Order JSON.
- `hash`  as the order hash
- `how_to_call` as how to call such as Call or DelegateCall
- `calldata` as call data
- `replacement_pattern` as  Calldata replacement pattern, or an empty byte array for no replacement.
- `static_target` as  Static call target, zero for no static call.
- `static_extradata` as Static call extra data.
- `payment_token` as Token used to pay for the order, or the zero-AccountId as a sentinel value for Ether.
- `base_price` as Base price of the order (in paymentTokens).
- `extra` as Auction extra parameter - minimum bid increment for English auctions, starting/ending price difference.
- `listing_time` as Listing timestamp.
- `expiration_time` asExpiration timestamp - 0 for no expiry.
- `salt`  as OrderType salt, used to prevent duplicate hashes.
- `orderbook_inclusion_desired`

###### cancelOrder
To cancel a order, one must send a transaction with a `wyvernExchange.cancelOrder` extrinsic.

##### Permissions
Order owner

##### Parameters
- `addrs` as the array of order's fields in **AccoundId** type .
- `uints` as the array of order's fields in integer/Balance type
- `fee_method` as Fee method: protocol fee or split fee.
- `side` as Side: buy or sell.
- `sale_kind` as Currently supported kinds of sale: fixed price, Dutch auction. 
- `how_to_call` as  Call or DelegateCall.
- `calldata` as  order calldata
- `replacement_pattern` as replacement mask
- `static_extradata`as order  extradata
- `sig` signature.
  
##### Events
OrderCancelled
order_hash: The  order hash.


###### atomicMatch
To atomically match two orders, ensuring validity of the match, and execute all associated state transitions, one must send a transaction with a `wyvernExchange.atomicMatch` extrinsic.

##### Permissions
Anyone

##### Parameters
- `addrs` as the array of Buy-side and Sell-side order's fields in **AccoundId** type .
- `uints` as the array of Buy-side and Sell-side order's fields in integer/Balance type
- `fee_methods_sides_kinds_how_to_calls` as the array of the Buy-side and Sell-side order's fields in Enum type such as Fee method: protocol fee or split fee, Side: buy or sell,Currently supported kinds of sale: fixed price, Dutch auction, Call or DelegateCall.
- `calldata_buy` as Buy-side order calldata
- `calldata_sell` as Sell-side order calldata
- `replacement_pattern_buy` Buy-side order calldata replacement mask
- `replacement_pattern_sell` Sell-side order calldata replacement mask
- `static_extradata_buy` as Buy-side order extradata
- `static_extradata_sell` as Sell-side order extradata
- `sig_buy` as the buy-side order signature.
- `sig_sell` as the sell-side order signature.
- `rss_metadata` as the metadata of the signature of order's hash 
##### Events
OrdersMatched
- `buy_hash` as the hash of Buy-side order
- `sell_hash` as the hash of Sell-side order
- `buy_maker` as the maker accountId of Buy-side order
- `sell_maker` as the maker accountId of Sell-side order
- `price` as the price  of  order
- `metadata` as the metadata of  order


#### WyvernExchangeCore Pallet Functions
###### changeMinimumMakerProtocolFee
To change the minimum maker fee paid to the protocol, one must send a transaction with a `wyvernExchange.changeMinimumMakerProtocolFee` extrinsic .

##### Permissions
Owner

##### Parameters
- `new_minimum_maker_protocol_fee` as the new fee to set in basis points.
  
##### Events
MinimumMakerProtocolFeeChanged
- `new_minimum_maker_protocol_fee` as the new fee to set in basis points.

###### changeMinimumTakerProtocolFee
To change the minimum taker fee paid to the protocol, one must send a transaction with a `wyvernExchange.changeMinimumTakerProtocolFee` extrinsic.

##### Permissions
Owner

##### Parameters
- `new_minimum_taker_protocol_fee` as the new fee to set in basis points.
  
##### Events
MinimumTakerProtocolFeeChanged
- `new_minimum_taker_protocol_fee` as the new fee to set in basis points.

###### changeProtocolFeeRecipient
To change the protocol fee recipient, one must send a transaction with a `wyvernExchange.changeProtocolFeeRecipient` extrinsic .

##### Permissions
Owner

##### Parameters
- `new_protocol_fee_recipient` as the new protocol fee recipient AccountId.
##### Events
ProtocolFeeRecipientChanged
- `new_protocol_fee_recipient` as the new protocol fee recipient AccountId.


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