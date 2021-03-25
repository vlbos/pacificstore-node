// A demonstration of interacting with custom RPCs using Polkadot js API

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { stringToHex, stringToU8a, u8aToHex } = require('@polkadot/util');
const { Bytes, Option, u32, Vec } = require('@polkadot/types');
const { v4: uuidv4 } = require('uuid');
const { readFileSync } = require('fs');
// import { makeOrderArrayEx, makeOrderEx, makeOrder, orderFromJSON } from './order.js'
const { makeOrderArrayEx, makeOrderEx, makeOrder, orderFromJSON } = require('./order.js')

// Construct parameters for API instance
const wsProvider = new WsProvider('ws://localhost:9944');
const types = require('./lib/types.json');
const rpcs = require(`./lib/rpcs.json`);
const rpc = { ...rpcs };
const { TypeRegistry } = require('@polkadot/types/create');
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


    let order = await api.rpc.orderbook.getOrder({
        limit: 1,
        offset: 1,
        owner: stringToU8a(users.francis.key.publicKey),
        token_ids: [stringToU8a('dddddddddddddddddddddddddd')],
        params: [[[0x1], [0x1]]]
    });
    console.log(`The value from the getOrder is ${order}\n`);

    let orders = await api.rpc.orderbook.getOrders({
        limit: 1,
        offset: 1,
        owner: stringToU8a(users.francis.key.publicKey),
        token_ids: [stringToU8a('dddddddddddddddddddddddddd')],
        params: [["[0x1]", "[0x1]"]]
    }, 1);
    console.log(`The value from the getOrders is ${orders}\n`);
    let asset = await api.rpc.orderbook.getAsset("", stringToU8a('dddddddddddddddddddddddddd'));
    console.log(`The value from the getAsset is ${asset}\n`);
    let assets = await api.rpc.orderbook.getAssets({
        limit: 1,
        offset: 1,
        owner: stringToU8a(users.francis.key.publicKey),
        token_ids: [stringToU8a('dddddddddddddddddddddddddd')],
        params: [[[0x1], [0x1]]]
    }, 1);
    console.log(`The value from the getAssets is ${assets}\n`);
    let accounts = Object.values(users).map((u) => u.key.address);
    let accounts7 = accounts.splice(0, 7);
    let accounts77 = accounts.splice(2, 7);
    let buy = makeOrder(users.betty.key.address, true);
    let sell = makeOrder(users.bob.key.address, false);
    [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken] = accounts77;
    [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken] = accounts7;
    console.log("hashOrderEx(",
        [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
        [sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
        sell.feeMethod,
        sell.side,
        sell.saleKind,
        sell.howToCall,
        sell.calldata,
        sell.replacementPattern,
        sell.staticExtradata);
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
    console.log(`The value from  hashOrderEx is ${sell_hash}\n`);

    console.log("hashToSignEx(",
        [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
        [sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
        sell.feeMethod,
        sell.side,
        sell.saleKind,
        sell.howToCall,
        sell.calldata,
        sell.replacementPattern,
        sell.staticExtradata);

    let order_hash = await api.rpc.wyvernExchange.hashToSignEx(
        [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
        [sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
        sell.feeMethod,
        sell.side,
        sell.saleKind,
        sell.howToCall,
        sell.calldata,
        sell.replacementPattern,
        sell.staticExtradata);

    console.log(`The value from  hashToSignEx is ${order_hash}\n`);

    // let order_sig = await api.sign(users.betty.key,order_hash);
    let s = await users.betty.key.sign(order_hash);
    // let order_sig =api.createType("Signature","Sr25519(\""+s+"\")");//registry.createType('Signature',users.betty.key.sign(order_hash,{withType: true}));
    //   let order_sig = "Sr25519(\""+s+"\")";
    let order_sig = s;//{"Sr25519":s};//[1].concat(s);

    // import { stringToU8a, u8aToHex } from '@polkadot/util';

    // // create Alice based on the development seed
    // const alice = keyring.addFromUri('//Alice');

    // // create the message, actual signature and verify
    // const message = stringToU8a('this is our message');
    // const signature = alice.sign(message);
    // const isValid = alice.verify(message, signature);

    // // output the result
    // console.log(`${u8aToHex(signature)} is ${isValid ? 'valid' : 'invalid'}`);

    // import { stringToU8a, u8aToHex } from '@polkadot/util';
    // import { signatureVerify } from '@polkadot/util-crypto';

    // // create Alice based on the development seed
    // const alice = keyring.addFromUri('//Alice');

    // // create the message and sign it
    // const message = stringToU8a('this is our message');
    // const signature = alice.sign(message);

    // // verify the message using Alice's address
    // const { isValid } = signatureVerify(message, signature, alice.address);

    // // output the result
    // console.log(`${u8aToHex(signature)} is ${isValid ? 'valid' : 'invalid'}`);

    const SIG = '0x659effefbbe5ab4d7136ebb5084b959eb424e32b862307371be4721ac2c46334245af4f1476c36c5e5aff04396c2fdd2ce561ec90382821d4aa071b559b1db0f';
    // order_sig=SIG;
    order = buy;
    console.log("===========validateOrderEx(",
        [order.exchange, order.maker, order.taker, order.feeRecipient, order.target, order.staticTarget, order.paymentToken],
        [order.makerRelayerFee, order.takerRelayerFee, order.makerProtocolFee, order.takerProtocolFee, order.basePrice, order.extra, order.listingTime, order.expirationTime, order.salt],
        order.feeMethod,
        order.side,
        order.saleKind,
        order.howToCall,
        order.calldata,
        order.replacementPattern,
        order.staticExtradata,
        order_sig
    );
    let result = await api.rpc.wyvernExchange.validateOrderEx(
        [order.exchange, order.maker, order.taker, order.feeRecipient, order.target, order.staticTarget, order.paymentToken],
        [order.makerRelayerFee, order.takerRelayerFee, order.makerProtocolFee, order.takerProtocolFee, order.basePrice, order.extra, order.listingTime, order.expirationTime, order.salt],
        order.feeMethod,
        order.side,
        order.saleKind,
        order.howToCall,
        order.calldata,
        order.replacementPattern,
        order.staticExtradata,
        order_sig
    );
    console.log(`The value from  validateOrderEx is ${result}\n`);

    console.log("validateOrderParametersEx(",
        [order.exchange, order.maker, order.taker, order.feeRecipient, order.target, order.staticTarget, order.paymentToken],
        [order.makerRelayerFee, order.takerRelayerFee, order.makerProtocolFee, order.takerProtocolFee, order.basePrice, order.extra, order.listingTime, order.expirationTime, order.salt],
        order.feeMethod,
        order.side,
        order.saleKind,
        order.howToCall,
        order.calldata,
        order.replacementPattern,
        order.staticExtradata
    );
    result = await api.rpc.wyvernExchange.validateOrderParametersEx(
        [order.exchange, order.maker, order.taker, order.feeRecipient, order.target, order.staticTarget, order.paymentToken],
        [order.makerRelayerFee, order.takerRelayerFee, order.makerProtocolFee, order.takerProtocolFee, order.basePrice, order.extra, order.listingTime, order.expirationTime, order.salt],
        order.feeMethod,
        order.side,
        order.saleKind,
        order.howToCall,
        order.calldata,
        order.replacementPattern,
        order.staticExtradata
    );
    console.log(`The value from  validateOrderParametersEx is ${result}\n`);

    console.log("ordersCanMatchEx(",
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
    console.log(`The value from  ordersCanMatchEx is ${result}\n`);

    console.log("calculateMatchPriceEx(",
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
    console.log(`The value from  calculateMatchPriceEx is ${result}\n`);
}

main().catch(console.error).finally(() => process.exit());
