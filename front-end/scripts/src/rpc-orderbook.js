// A demonstration of interacting with custom RPCs using Polkadot js API

import { createRequire } from 'module';
// import { object } from 'prop-types';
const require = createRequire(import.meta.url);
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { stringToHex, stringToU8a, u8aToHex } from '@polkadot/util';
import { Bytes, Option, u32, Vec } from '@polkadot/types';
import { v4 as uuidv4 } from 'uuid';
import { readFileSync } from 'fs';
// import { makeOrderArrayEx, makeOrderEx, makeOrder, orderFromJSON } from './order.js'
import { makeOrderArrayEx, makeOrderArrayHexEx, makeOrderFromJSONHex, makeOrderEx, makeOrder, orderFromJSON } from './orders/order.js'

// Construct parameters for API instance
const wsProvider = new WsProvider('ws://localhost:9944');
const types = require('./lib/types.json');
const rpcs = require(`./lib/rpcs.json`);
const rpc = { ...rpcs };
import { TypeRegistry } from '@polkadot/types/create';
const registry = new TypeRegistry();


async function main() {
    // Construct the actual api
    const api = await ApiPromise.create({
        provider: wsProvider,
        types,
        rpc,
        registry
    });

    const keyring = new Keyring({ type: 'sr25519' });

    const users = {
        admin: { key: keyring.addFromUri('//Alice', { name: 'ADMIN' }), nonce: 0 },
        bob: { key: keyring.addFromUri('//Bob', { name: 'Bob' }), nonce: 0 },
        bobBank: { key: keyring.addFromUri('//Bob//stash', { name: 'Bob-BANK' }), nonce: 0 },
        betty: { key: keyring.addFromUri('//Bert', { name: 'Bert' }), nonce: 0 },
        charlie: { key: keyring.addFromUri('//Charlie', { name: 'Charlie' }), nonce: 0 },
        charlieBank: { key: keyring.addFromUri('//Charlie//stash', { name: 'Charlie-BANK' }), nonce: 0 },
        clarice: { key: keyring.addFromUri('//Clarice', { name: 'Clarice' }), nonce: 0 },
        dave: { key: keyring.addFromUri('//Dave', { name: 'Dave' }), nonce: 0 },
        daveBank: { key: keyring.addFromUri('//Dave//stash', { name: 'Dave-BANK' }), nonce: 0 },
        daisy: { key: keyring.addFromUri('//Daisy', { name: 'Daisy' }), nonce: 0 },
        eve: { key: keyring.addFromUri('//Eve', { name: 'Eve' }), nonce: 0 },
        eveBank: { key: keyring.addFromUri('//Eve//stash', { name: 'Eve-BANL' }), nonce: 0 },
        erowid: { key: keyring.addFromUri('//Erowid', { name: 'Erowid' }), nonce: 0 },
        ferdie: { key: keyring.addFromUri('//Ferdie', { name: 'Ferdie' }), nonce: 0 },
        ferdieBank: { key: keyring.addFromUri('//Ferdie//stash', { name: 'Ferdie-BANK' }), nonce: 0 },
        francis: { key: keyring.addFromUri('//Francis', { name: 'Francis' }), nonce: 0 },
    }

    const orderArray = makeOrderArrayHexEx();
    const orders = makeOrderEx();

    // console.log("getOrder(", {
    //     params: orderArray[0]
    // });
    // let order = await api.rpc.orderbook.getOrder({
    //     params: orderArray[0]
    // });
    // console.log(`The value from the getOrder is ${order}\n`);

    // let orderjsons = await api.rpc.orderbook.getOrders({
    //     params: orderArray[1]
    // }, 1);
    // console.log(`The value from the getOrders is ${orderjsons}\n`);

    console.log("getAsset(", orders[2]["metadata.asset.address"], stringToHex(orders[2]["metadata.asset.id"] + ""));
    let asset1 = await api.rpc.orderbook.getAsset(stringToHex(orders[2]["metadata.asset.address"]), stringToHex(orders[2]["metadata.asset.id"] + ""));
    console.log(`The value from the getAsset is ${asset1}\n`);
    console.log("===================");
    console.log(makeOrderFromJSONHex([JSON.parse(`${asset1}`)]));
    console.log("===================");

    console.log("=======getAssets(", {
        token_ids: [stringToHex(orders[3]["metadata.asset.id"])],
        asset_contract_address: stringToHex(orders[3]["metadata.asset.address"])
    }, 1);
    let assets = await api.rpc.orderbook.getAssets({
        token_ids: [stringToHex(orders[2]["metadata.asset.id"])],
        asset_contract_address: stringToHex(orders[2]["metadata.asset.address"])
    }, 1);
    console.log(`The value from the getAssets is ${assets}\n`);

}

main().catch(console.error).finally(() => process.exit());
