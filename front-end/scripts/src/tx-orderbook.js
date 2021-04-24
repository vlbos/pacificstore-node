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
        eveBank: { key: keyring.addFromUri('//Eve//stash', { name: 'Eve-BANK' }), nonce: 0 },
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
                let nonce = await api.rpc.system.accountNextIndex(sender.key.address);
                if (0 != nonce.words[0]) {
                    sender.nonce = nonce.words[0];
                }
            }
        }

        let accounts = Object.values(users).map((u) => u.key.address);
        let accounts7 = accounts.splice(0, 7);
        let accounts77 = accounts.splice(2, 7);
        // console.log(accounts, "=======account=====", accounts77);

        console.log("======transfer====");

        submit(api, api.tx.balances.transfer(users.betty.key.address, salary), users.bobBank);
        submit(api, api.tx.balances.transfer(users.bob.key.address, salary), users.bobBank);

        await new Promise(r => setTimeout(r, block));

        submit(api, api.tx.orderbook.changeOwner(
            users.betty.key.address), users.betty);
        submit(api, api.tx.orderbook.setOrderLimits(
            1000), users.betty);
        submit(api, api.tx.orderbook.setAssetWhiteListLimits(
            1000), users.betty);

        const orderArray = makeOrderArrayEx();
        // let o = orderArray[0];
        console.log("======postOrder=========");
        let order_id = "";
        for (let o of orderArray) {
            // console.log(o);
            order_id = uuidv4();
            submit(api, api.tx.orderbook.postOrder(order_id, users.bob.key.address, o), users.betty);
        }

        console.log("========postAssetWhiteList=======");
        submit(api, api.tx.orderbook.postAssetWhiteList('users.bob.key.address', 'token id', "test@test.com"), users.betty);

        await new Promise(r => setTimeout(r, block));
    } catch (e) {
        console.log("============", e);
        throw e;
    }
}

main().catch(console.error).finally(() => process.exit());
