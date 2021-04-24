# PacificStore Orderbook & WyvernExchange Modules 
## Introduce
PacificStore is an open-sourced crypto unique asset marketplace which allows users to trade popular tokenized assets and collectibles built on interoperable standards.

In December 2020, we received a grant from the web3 foundation, for which we are very grateful.Fast forward to now, almost 5 months later, with the support of the Open Grants Program by the Web3 Foundation, we are pleased to share the work of our  milestone-1 — providing Orderbook & WyvernExchange components for the community.

When it comes to the rise of crypto collectables, it’s no surprise that many secondary markets have arisen for the exchange of these unique assets.
For those unfamiliar with non-fungible tokens (better known as NFTs), they are tokens which are unique from one another due to special characteristics or identifiers which distinguish them from others in a similar set.
Specifically within DeFi, we’ve seen NFTs play a role in the tokenization of assets and contracts alike, all of which play a larger role in the notion of composability – or different products interacting with one another towards a larger ecosystem.
In this overview, we’ll dive into PacificStore – an Decentralized NFT Exchange Aggregation Platform built on Polkadot/Kusama.

PacificStore is inspired by **OpenSea** but implements **Substrate** runtime module.PacificStore js part uses polkadot.js for interacting With Substrate RPC of Orderbook and WyvernExchange pallet of PacificStore.

## Overview
For this milestone we have provided the following components:
    1. Orderbook pallet
    2. WyvernExchange pallet
    3. PacificStore-node
    4. Test Project
Orderbook pallet implements the backend of Opensea.js based on Substrate.It provides functionalities for posting order,posting asset whitelist.
WyvernExchange pallet implements the Exchange and ExchangeCore part of Wyvern protocol .It provides functionalities for hashing order,validating order,approving order,cancelling order,etc.
PacificStore-node based on Substrate-node-template,integrates Orderbook pallet , WyvernExchange pallet and both custom RPCs.
Test Project based on Substate-front-end-template,In order to demonstrate the usage of the two pallets.It includes custom types and RPCs.



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
yarn examples
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