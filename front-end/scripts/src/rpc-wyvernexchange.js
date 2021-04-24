// A demonstration of interacting with custom RPCs using Polkadot js API

import { createRequire } from 'module';
// import { object } from 'prop-types';
const require = createRequire(import.meta.url);

import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { stringToHex, stringToU8a, u8aToHex } from '@polkadot/util';
import { Bytes, Option, u32, Vec } from '@polkadot/types';
import { readFileSync } from 'fs';
// import { makeOrderArrayEx, makeOrderEx, makeOrder, orderFromJSON } from './order.js'
import { makeOrderArrayEx, makeOrderEx, makeOrder, orderFromJSON } from './orders/order.js'

// Construct parameters for API instance
const wsProvider = new WsProvider('ws://localhost:9944');
// import types from './lib/types.json';
const types = require(`./lib/types.json`);
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
        eveBank: { key: keyring.addFromUri('//Eve//stash', { name: 'Eve-BANK' }), nonce: 0 },
        erowid: { key: keyring.addFromUri('//Erowid', { name: 'Erowid' }), nonce: 0 },
        ferdie: { key: keyring.addFromUri('//Ferdie', { name: 'Ferdie' }), nonce: 0 },
        ferdieBank: { key: keyring.addFromUri('//Ferdie//stash', { name: 'Ferdie-BANK' }), nonce: 0 },
        francis: { key: keyring.addFromUri('//Francis', { name: 'Francis' }), nonce: 0 },
    }

    let accounts = Object.values(users).map((u) => u.key.address);
    let accounts7 = accounts.splice(0, 7);
    let accounts77 = accounts.splice(2, 7);
    // console.log(accounts, "=======account=====", accounts77);
    const buy = makeOrder(users.bob.key.address, true, 0);
    const sell = makeOrder(users.bob.key.address, false, 1);
    [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken] = accounts77;
    [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken] = accounts7;
    buy.taker = users.bob.key.address;
    sell.taker = users.bob.key.address;
    sell.feeRecipient = users.bob.key.address;
    buy.exchange = users.bob.key.address;
    sell.exchange = users.bob.key.address;
    buy.target = sell.target;
    buy.paymentToken = sell.paymentToken;
    buy.maker = users.bob.key.address;
    console.log("hashOrderEx(");
    let sell_hash = await api.rpc.wyvernExchange.hashOrderEx(
        [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
        [sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
        sell.feeMethod,
        sell.side,
        sell.saleKind,
        sell.howToCall,
        sell.calldata,
        sell.replacementPattern,
        sell.staticExtradata);
    console.log(`The value from  hashOrderEx is ${sell_hash} \n`);

    console.log("hashToSignEx(");

    let sell_hashsig = await api.rpc.wyvernExchange.hashToSignEx(
        [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
        [sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
        sell.feeMethod,
        sell.side,
        sell.saleKind,
        sell.howToCall,
        sell.calldata,
        sell.replacementPattern,
        sell.staticExtradata);

    console.log(`The value from  hashToSignEx is ${sell_hashsig}\n`);

    // let sell_hash = await api.sign(users.betty.key,sell_hash);
    let sell_sig = await users.betty.key.sign(sell_hashsig);

    console.log("===========validateOrderEx(");
    let result = await api.rpc.wyvernExchange.validateOrderEx(
        [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
        [sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
        sell.feeMethod,
        sell.side,
        sell.saleKind,
        sell.howToCall,
        sell.calldata,
        sell.replacementPattern,
        sell.staticExtradata,
        sell_sig
    );
    console.log(`The value from  validateOrderEx is `, JSON.parse(`${result}`), `\n`);

    console.log("validateOrderParametersEx(");
    result = await api.rpc.wyvernExchange.validateOrderParametersEx(
        [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
        [sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
        sell.feeMethod,
        sell.side,
        sell.saleKind,
        sell.howToCall,
        sell.calldata,
        sell.replacementPattern,
        sell.staticExtradata
    );
    console.log(`The value from  validateOrderParametersEx is `, JSON.parse(`${result}`), `\n`);

    console.log("ordersCanMatchEx(");
    result = await api.rpc.wyvernExchange.ordersCanMatchEx(
        [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken, sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
        [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt, sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
        [buy.feeMethod, buy.side, buy.saleKind, buy.howToCall, sell.feeMethod, sell.side, sell.saleKind, sell.howToCall],
        buy.calldata,
        sell.calldata,
        buy.replacementPattern,
        sell.replacementPattern,
        buy.staticExtradata,
        sell.staticExtradata
    );
    console.log(`The value from  ordersCanMatchEx is `, JSON.parse(`${result}`), `\n`);

    console.log("calculateMatchPriceEx(");
    // assert.equal(ret, true, 'Orders were not matchable!')
    result = await api.rpc.wyvernExchange.calculateMatchPriceEx(
        [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken, sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
        [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt, sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
        [buy.feeMethod, buy.side, buy.saleKind, buy.howToCall, sell.feeMethod, sell.side, sell.saleKind, sell.howToCall],
        buy.calldata,
        sell.calldata,
        buy.replacementPattern,
        sell.replacementPattern,
        buy.staticExtradata,
        sell.staticExtradata
    );
    console.log(`The value from  calculateMatchPriceEx is `, JSON.parse(`${result}`), `\n`);
}

main().catch(console.error).finally(() => process.exit());
