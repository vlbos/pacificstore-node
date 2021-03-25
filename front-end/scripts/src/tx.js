import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { v4 as uuidv4 } from 'uuid';
import { u8aToString, u8aToHex } from '@polkadot/util';

import submit from './lib/submit-signed-tx.js';
// import types from './lib/types.json';
// import { readFile } from 'fs/promises'
// const types = JSON.parse(await readFile(new URL('./lib/types.json', import.meta.url)))
// // const { name, version } = pkg;           

import { createRequire } from 'module';
// import { object } from 'prop-types';
const require = createRequire(import.meta.url);

const types = require('./lib/types.json');
const rpcs = require(`./lib/rpcs.json`);
const rpc = { ...rpcs };
import { makeOrderArrayEx, makeOrderEx, makeOrder, orderFromJSON } from './orders/order.js'


async function main() {
    const provider = new WsProvider("ws://127.0.0.1:9944");
    const api = await ApiPromise.create({ provider, types, rpc });
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

        const salary = 100_000_000_000_000;
        let senders = [users.bobBank, users.bob, users.betty];
        for (let sender of senders) {
            if (0 == sender.nonce) {
                console.log("sender.nonce==7==", sender.nonce);
                let nonce = await api.rpc.system.accountNextIndex(sender.key.address);
                if (0 != nonce.words[0]) {
                    sender.nonce = nonce.words[0];
                    console.log("sender.nonce==77==", sender.nonce);
                }
            }
        }

        const arg_offset = 2;
        const user_arg_offset = 0;

        var argumentss = process.argv.splice(arg_offset);
        console.log('arguments：', argumentss);

        // // //////////////////////////
        // // // print process.argv
        process.argv.forEach(function (val, index, array) {
            console.log(index + ': ' + val);
        });


        // await new Promise(r => setTimeout(r, block));

        let index = 0;
        if (undefined != argumentss[0]) {
            index = argumentss[0];
        }
        let accounts = Object.values(users).map((u) => u.key.address);
        let accounts7 = accounts.splice(0, 7);
        let accounts77 = accounts.splice(2, 7);
        console.log(accounts, "=======account=====", accounts77);
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
        let buy_hash = "";
        let sell_hash = "";
        let buy_sig = "";
        let sell_sig = "";
        // const cmds = ["transfer", "changeOwner", "atomicMatchEx"];
        // const cmds = ["transfer", "changeOwner", "cancelOrderEx"];
        // const cmds = [ "transfer","changeOwner","approveOrderEx", "cancelOrderEx", "atomicMatchEx"];
        const cmds = ["transfer", "postOrder", "postAssetWhiteList", "changeMinimumMakerProtocolFee", "changeMinimumTakerProtocolFee", "changeProtocolFeeRecipient"];//
        // const cmd = cmds[index];
        for (let cmd of cmds) {
            switch (cmd) {
                case "transfer":
                    console.log("======transfer====");

                    submit(api, api.tx.balances.transfer(users.betty.key.address, salary), users.bobBank);
                    submit(api, api.tx.balances.transfer(users.bob.key.address, salary), users.bobBank);

                    await new Promise(r => setTimeout(r, block));
                    break;
                case "postOrder":
                    const order_id = uuidv4();
                    const orderArray = makeOrderArrayEx();
                    let o = orderArray[0];
                    // console.log(o);
                    // for (let o of orderArray) {
                    console.log("======postOrder=========");

                    submit(api, api.tx.orderbook.postOrder(order_id, users.bob.key.address, o), users.betty);

                    // }
                    break;
                case "postAssetWhiteList":
                    console.log("========postAssetWhiteList=======");
                    submit(api, api.tx.orderbook.postAssetWhiteList('users.bob.key.address', 'token id', "test@test.com"), users.betty);
                    break;
                case "approveOrderEx":
                    console.log("========approveOrderEx=======", [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
                        [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
                        buy.feeMethod,
                        buy.side,
                        buy.saleKind,
                        buy.howToCall,
                        buy.calldata,
                        buy.replacementPattern,
                        buy.staticExtradata,
                        true);
                    buy.maker = users.betty.key.address;
                    submit(api, api.tx.wyvernExchange.approveOrderEx(
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
                    buy.maker = users.bob.key.address;

                    console.log("========approveOrderEx=======", [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
                        [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
                        buy.feeMethod,
                        buy.side,
                        buy.saleKind,
                        buy.howToCall,
                        buy.calldata,
                        buy.replacementPattern,
                        buy.staticExtradata,
                        true);
                    submit(api, api.tx.wyvernExchange.approveOrderEx(
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
                    console.log("========hashToSignEx=======", [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
                        [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
                        buy.feeMethod,
                        buy.side,
                        buy.saleKind,
                        buy.howToCall,
                        buy.calldata,
                        buy.replacementPattern,
                        buy.staticExtradata);
                    buy_hash = await api.rpc.wyvernExchange.hashToSignEx(
                        [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
                        [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
                        buy.feeMethod,
                        buy.side,
                        buy.saleKind,
                        buy.howToCall,
                        buy.calldata,
                        buy.replacementPattern,
                        buy.staticExtradata);
                    buy_sig = users.bob.key.sign(buy_hash);
                    console.log("========cancelOrderEx=======", [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
                        [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
                        buy.feeMethod,
                        buy.side,
                        buy.saleKind,
                        buy.howToCall,
                        buy.calldata,
                        buy.replacementPattern,
                        buy.staticExtradata,
                        u8aToHex(buy_sig));

                    submit(api, api.tx.wyvernExchange.cancelOrderEx(
                        [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
                        [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
                        buy.feeMethod,
                        buy.side,
                        buy.saleKind,
                        buy.howToCall,
                        buy.calldata,
                        buy.replacementPattern,
                        buy.staticExtradata,
                        u8aToHex(buy_sig)), users.bob);

                    break;
                case "atomicMatchEx":
                    console.log("========approveOrderEx=====buy==", [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
                        [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
                        buy.feeMethod,
                        buy.side,
                        buy.saleKind,
                        buy.howToCall,
                        buy.calldata,
                        buy.replacementPattern,
                        buy.staticExtradata,
                        true);
                    buy.maker = users.bob.key.address;
                    submit(api, api.tx.wyvernExchange.approveOrderEx(
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
                    ), users.bob);
                    console.log("========approveOrderEx=====sell==", [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
                        [sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
                        sell.feeMethod,
                        sell.side,
                        sell.saleKind,
                        sell.howToCall,
                        sell.calldata,
                        sell.replacementPattern,
                        sell.staticExtradata,
                        true);
                    sell.maker = users.betty.key.address;
                    submit(api, api.tx.wyvernExchange.approveOrderEx(
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

                    console.log("========hashToSignEx====sell===", [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
                        [sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
                        sell.feeMethod,
                        sell.side,
                        sell.saleKind,
                        sell.howToCall,
                        sell.calldata,
                        sell.replacementPattern,
                        sell.staticExtradata);

                    sell_hash = await api.rpc.wyvernExchange.hashToSignEx(
                        [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
                        [sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
                        sell.feeMethod,
                        sell.side,
                        sell.saleKind,
                        sell.howToCall,
                        sell.calldata,
                        sell.replacementPattern,
                        sell.staticExtradata);
                    console.log(`The value from  hashToSignEx is ${sell_hash}\n`);
                    console.log("========hashToSignEx===buy====", [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
                        [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
                        buy.feeMethod,
                        buy.side,
                        buy.saleKind,
                        buy.howToCall,
                        buy.calldata,
                        buy.replacementPattern,
                        buy.staticExtradata);

                    buy_hash = await api.rpc.wyvernExchange.hashToSignEx(
                        [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
                        [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
                        buy.feeMethod,
                        buy.side,
                        buy.saleKind,
                        buy.howToCall,
                        buy.calldata,
                        buy.replacementPattern,
                        buy.staticExtradata);

                    console.log(`The value from  hashToSignEx is ${buy_hash}\n`);
                    await new Promise(r => setTimeout(r, block));

                    // let buy_sig = users.betty.key.sign(buy_hash, { withType: true });
                    // let sell_sig = users.betty.key.sign(sell_hash, { withType: true });
                    buy_sig = users.bob.key.sign(buy_hash);
                    sell_sig = users.betty.key.sign(sell_hash);
                    console.log([buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken, sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
                        [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt, sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
                        [buy.feeMethod, buy.side, buy.saleKind, buy.howToCall, sell.feeMethod, sell.side, sell.saleKind, sell.howToCall],
                        buy.calldata,
                        sell.calldata,
                        buy.replacementPattern,
                        sell.replacementPattern,
                        buy.staticExtradata,
                        sell.staticExtradata,
                        u8aToHex(buy_sig), u8aToHex(sell_sig),
                        '0x0000000000000000000000000000000000000000000000000000000000000000', "========atomicMatchEx=======", buy_sig, sell_sig, u8aToHex(buy_sig), u8aToHex(sell_sig));
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
                        u8aToHex(buy_sig), u8aToHex(sell_sig),
                        '0x0000000000000000000000000000000000000000000000000000000000000000'), users.bob);
                    break;
                case "changeMinimumMakerProtocolFee":
                    console.log("========changeMinimumMakerProtocolFee=======");
                    submit(api, api.tx.wyvernExchangeCore.changeMinimumMakerProtocolFee(
                        1), users.betty);
                    break;
                case "changeMinimumTakerProtocolFee":
                    console.log("========changeMinimumTakerProtocolFee=======");
                    submit(api, api.tx.wyvernExchangeCore.changeMinimumTakerProtocolFee(
                        1), users.betty);
                    break;
                case "changeProtocolFeeRecipient":
                    console.log("========changeProtocolFeeRecipient=======");
                    submit(api, api.tx.wyvernExchangeCore.changeProtocolFeeRecipient(
                        users.bob.key.address), users.betty);
                    break;
                case "changeOwner":
                    console.log("========changeOwner=======");
                    submit(api, api.tx.wyvernExchangeCore.changeOwner(
                        users.bob.key.address), users.betty);
                    break;
            }
        }
        await new Promise(r => setTimeout(r, block));
    } catch (e) {
        console.log("============", e);
        throw e;
    }
}

main().catch(console.error).finally(() => process.exit());
