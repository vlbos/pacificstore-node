# Pacific-store Front End

This allows you to create a front-end application that connects to a
[Pacific-store](https://github.com/vlbos/Pacificstore-node/front-end) node back-end with minimal
configuration.

## Our version of Front End

The Front End was also mainly demonstrated. We only need to show some configuration that happens behind the scenes:

### Loads appropriate custom API types

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

## Custom RPCs for JS API

### OrderBook Module

```
"orderbook": {
      "getOrder": {
        "description": "Get an order from the orderbook, throwing if none is found.",
        "params": [
          {
            "name": "order_query",
            "type": "Option<OrderQueryJSON<AccountId>>"
          }
        ],
        "type": "Option<OrderJSONType<AccountId, Moment>>"
      },
      "getOrders": {
        "description": " Get a list of orders from the orderbook, returning the page of orders",
        "params": [
          {
            "name": "order_query",
            "type": "Option<OrderQueryJSON<AccountId>>"
          },
          {
            "name": "page",
            "type": "Option<u64>"
          }
        ],
        "type": "Option<Vec<OrderJSONType<AccountId, Moment>>>"
      },
      "getAsset": {
        "description": "Fetch an asset from the API, throwing if none is found",
        "params": [
          {
            "name": "token_address",
            "type": "String"
          },
          {
            "name": "token_id",
            "type": "String"
          }
        ],
        "type": "Option<JSONType>"
      },
      "getAssets": {
        "description": "Fetch list of assets from the API, returning the page of assets and the count of total assets",
        "params": [
          {
            "name": "asset_query",
            "type": "Option<AssetQueryJSON<AccountId>>"
          },
          {
            "name": "page",
            "type": "Option<u64>"
          }
        ],
        "type": "Option<Vec<JSONType>>"
      }
    },
```

### WyvernExchange Module

[Custom RPCs](https://github.com/vlbos/pacificstore-node/blob/dev/front-end/src/config/development.json)

[Custom Types](https://github.com/vlbos/pacificstore-node/blob/dev/front-end/src/config/types.json)

## Using The Front End

### \* Install dependencies and run the front end app

```bash
# Clone the repository
git clone https://github.com/vlbos/Pacificstore-node.git
cd ./front-end
yarn install
```

- Run a Pacific-store Node

```
RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev --tmp
```

## Usage

You can start the Front End in development mode to connect to a locally running node

```bash
yarn start
```

Run the examples with:

```
cd front-end/scripts
yarn install
yarn examples
```

## Example

### OrderBook extrinsic example

- [order-book](front-end/scripts/src/tx-orderbook.js)

1. changeOwner
2. setOrderLimits
3. setAssetWhiteListLimits
4. postOrder
5. postAssetWhiteList

### OrderBook custom RPC example

- [order-book RPCs](front-end/scripts/src/rpc-orderbook.js)

1. getOrder
2. getOrders
3. getAsset
4. getAssets

### WyvernExchange extrinsic example

- [WyvernExchange](front-end/scripts/src/tx-wyvernexchange.js)

1. approveOrderEx
2. changeMinimumMakerProtocolFee
3. changeMinimumTakerProtocolFee
4. changeProtocolFeeRecipient
5. changeOwner
6. setContractSelf

- [WyvernExchange-cancel-order](front-end/scripts/src/tx-wyvernexchange-cancel-order.js.js)
- [WyvernExchange-match-order](front-end/scripts/src/tx-wyvernexchange-match-order.js)

### WyvernExchange custom RPC example

- [WyvernExchange RPCs](front-end/scripts/src/rpc-wyvernexchange.js)

1. hashOrderEx
2. hashToSignEx
3. validateOrderEx
4. validateOrderParametersEx
5. ordersCanMatchEx
6. calculateMatchPriceEx

## Upstream

This project was forked from the official
[Substrate front-end template](https://github.com/substrate-developer-hub/substrate-front-end-template/tree/v2.0.1).
Please refer to
[its documentation](https://github.com/substrate-developer-hub/substrate-front-end-template/blob/v2.0.1/README.md)
to learn about building and running the front-end.
