import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { v4 as uuidv4 } from 'uuid';

import submit from './lib/submit-signed-xt.js';
// import types from './lib/types.json';
// import { readFile } from 'fs/promises'
// const types = JSON.parse(await readFile(new URL('./lib/types.json', import.meta.url)))
// // const { name, version } = pkg;           

import { createRequire } from 'module';
// import { object } from 'prop-types';
const require = createRequire(import.meta.url);

const types = require('./types.json');
const rpcs = require(`./rpcs.json`);
const rpc = { ...rpcs };
import { makeOrderArrayEx, makeOrderEx, makeOrder, orderFromJSON } from './order.js'


async function main() {
    const provider = new WsProvider("ws://127.0.0.1:9944");
    const api = await ApiPromise.create({ provider, types,rpc });
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

    try {

        const second = 1000;
        const block = 6.5 * second;
        const minute = 60 * second;
        const hour = 60 * minute;
        const day = 24 * hour;
        await new Promise(r => setTimeout(r, block));


        console.log("======1====67=8888====");

        await new Promise(r => setTimeout(r, block));

        const salary = 100_000_000_000_000;


        submit(api, api.tx.balances.transfer(users.betty.key.address, salary), users.bobBank);


        await new Promise(r => setTimeout(r, block));
        console.log("======1=========");
        // submit(api, api.tx.sumStorage.setThing1(1), users.betty);
        console.log("========2=======");
        const cmds = ["postOrder", "postAssetWhiteList", "approveOrderEx", "cancelOrderEx", "changeMinimumMakerProtocolFee", "changeMinimumTakerProtocolFee", "changeProtocolFeeRecipient"];//
        const cmd = cmds[0];
  const buy = makeOrder(users.bob.key.address, true);
                const sell = makeOrder(users.bob.key.address, false);
        switch (cmd) {
            case "postOrder":
                const order_id = uuidv4();
                const orderArray = makeOrderArrayEx();
                for (let o of orderArray) {
                    submit(api, api.tx.orderbook.postOrder(order_id, users.bob.key.address, o), users.betty);
                }
                break;
            case "postAssetWhiteList":
                submit(api, api.tx.orderbook.postAssetWhiteList('users.bob.key.address', 'token id', "test@test.com"), users.betty);


                break;
            case "approveOrderEx":


                submit(api, api.tx.wyvernExchange.approveOrderEx(users.bob.key.address,
                    [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
                    [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
                    buy.feeMethod,
                    buy.side,
                    buy.saleKind,
                    buy.howToCall,
                    buy.calldata,
                    buy.replacementPattern,
                    buy.staticExtradata,
                    true
                ), users.betty);


                break;
            case "cancelOrderEx":


                submit(api, api.tx.wyvernExchange.approveOrderEx(users.bob.key.address,
                    [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
                    [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
                    buy.feeMethod,
                    buy.side,
                    buy.saleKind,
                    buy.howToCall,
                    buy.calldata,
                    buy.replacementPattern,
                    buy.staticExtradata,
                    true
                ), users.betty);
                submit(api, api.tx.wyvernExchange.cancelOrderEx(users.bob.key.address,
                    [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
                    [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
                    buy.feeMethod,
                    buy.side,
                    buy.saleKind,
                    buy.howToCall,
                    buy.calldata,
                    buy.replacementPattern,
                    buy.staticExtradata,
                    true), users.betty);

                break;
            case "atomicMatchEx":

              
                submit(api, api.tx.wyvernExchange.approveOrderEx(users.bob.key.address,
                    [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
                    [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
                    buy.feeMethod,
                    buy.side,
                    buy.saleKind,
                    buy.howToCall,
                    buy.calldata,
                    buy.replacementPattern,
                    buy.staticExtradata,
                    true
                ), users.betty);
                submit(api, api.tx.wyvernExchange.approveOrderEx(users.bob.key.address,
                    [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
                    [sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
                    sell.feeMethod,
                    sell.side,
                    sell.saleKind,
                    sell.howToCall,
                    sell.calldata,
                    sell.replacementPattern,
                    sell.staticExtradata,
                    true
                ), users.betty);
const buysig = "";
const sellsig="";

                submit(api, api.tx.wyvernExchange.atomicMatchEx(
                    [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken, sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
                    [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt, sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
                    [buy.feeMethod, buy.side, buy.saleKind, buy.howToCall, sell.feeMethod, sell.side, sell.saleKind, sell.howToCall],
                    buy.calldata,
                    sell.calldata,
                    buy.replacementPattern,
                    sell.replacementPattern,
                    buy.staticExtradata,
                    sell.staticExtradata,
                    [buysig, sellsig],
                    '0x0000000000000000000000000000000000000000000000000000000000000000'), users.betty);


                break;
            case "changeMinimumMakerProtocolFee":

                submit(api, api.tx.wyvernExchangeCore.changeMinimumMakerProtocolFee(users.bob.key.address,
                    1), users.betty);
                break;
            case "changeMinimumTakerProtocolFee":
                submit(api, api.tx.wyvernExchangeCore.changeMinimumTakerProtocolFee(users.bob.key.address,
                    1), users.betty);
                break;
            case "changeProtocolFeeRecipient":
                submit(api, api.tx.wyvernExchangeCore.changeProtocolFeeRecipient(users.bob.key.address,
                    users.bob.key.address), users.betty);
                break;
        }
        await new Promise(r => setTimeout(r, block));
    } catch (e) {
        console.log("============", e);
        throw e;
    }
}

main().catch(console.error).finally(() => process.exit());
