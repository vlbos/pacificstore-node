// import { readFile } from 'fs/promises'
// const pkg = JSON.parse(await readFile(new URL('./orders.json', import.meta.url)))
// const { name, version } = pkg;
import { stringToHex, hexToString, stringToU8a, u8aToHex } from '@polkadot/util';

import { createRequire } from 'module';
// import { object } from 'prop-types';
const require = createRequire(import.meta.url);

const ordersJSONFixture = require('./orders.json');

// import * as ordersJSONFixture from './orders.json'
const ordersJSON = ordersJSONFixture;//{"sdfsa":"ssd"};//ordersJSONFixture;

/*
 * Flatten Object @gdibble: Inspired by https://gist.github.com/penguinboy/762197
 *   input:  { 'a':{ 'b':{ 'b2':2 }, 'c':{ 'c2':2, 'c3':3 } } }
 *   output: { 'a.b.b2':2, 'a.c.c2':2, 'a.c.c3':3 }
 */
var flattenObjectx = function (ob) {
    var toReturn = {};
    var flatObject;
    for (var i in ob) {
        if (!ob.hasOwnProperty(i)) {
            continue;
        }
        if ((typeof ob[i]) === 'object') {
            flatObject = flattenObject(ob[i]);
            for (var x in flatObject) {
                if (!flatObject.hasOwnProperty(x)) {
                    continue;
                }
                toReturn[i + (!!isNaN(x) ? '.' + x : '')] = flatObject[x];
            }
        } else {
            toReturn[i] = ob[i];
        }
    }
    return toReturn;
};

function flattenObject(ob) {
    var toReturn = {};

    for (var i in ob) {
        if (!ob.hasOwnProperty(i)) continue;

        if ((typeof ob[i]) == 'object' && ob[i] !== null) {
            var flatObject = flattenObject(ob[i]);
            for (var x in flatObject) {
                if (!flatObject.hasOwnProperty(x)) continue;

                toReturn[i + '.' + x] = flatObject[x];
            }
        } else {
            toReturn[i] = ob[i];
        }
    }
    return toReturn;
}


const makeOrderEx = () => {
    let objs = [];
    ordersJSON.map((orderJSON, index) => {
        objs.push(flattenObject(orderJSON));
    })
    return objs;
}

const makeOrderArrayEx = () => {
    let objs = [];
    ordersJSON.map((orderJSON, index) => {
        let obj = flattenObject(orderJSON);
        let arr = [];
        Object.keys(obj).reduce((a, o) => { a.push([o, obj[o] + ""]); return a; }, arr);
        objs.push(arr);

    })
    // console.log(objs)
    return objs;
}
// makeOrderArrayEx();
const makeOrderArrayExForRust = () => {
    let objs = [];
    ordersJSON.map((orderJSON, index) => {
        let obj = flattenObject(orderJSON);
        let arr = [];
        Object.keys(obj).reduce((a, o) => { a.push(["b\""+o+"\"", "b\""+obj[o] + "\""]); return a; }, arr);
        objs.push(arr);

    })
    console.log(objs)
    return objs;
}
// makeOrderArrayExForRust();
const makeOrderArrayHexEx = () => {
    let objs = [];
    ordersJSON.map((orderJSON, index) => {
        let obj = flattenObject(orderJSON);
        let arr = [];

        Object.keys(obj).reduce((a, o) => {
            let objstr = obj[o] + "";
            // if (!objstr.startsWith("0x")) {
            objstr = stringToHex(objstr);
            // }
            a.push([stringToHex(o), objstr]); return a;
        }, arr);
        objs.push(arr);

    })
    // console.log(objs)
    return objs;
}

const hash_address = [
    'exchange',
    'maker.address',
    'taker.address',
    'fee_recipient.address',
    'target',
    'calldata',
    'replacement_pattern',
    'static_target',
    'static_extradata',
    'payment_token',
    'order_hash',
    'metadata.asset.address',
];
const makeOrderFromJSONHex = (orderjson) => {
    // let objs = [];

    let objs = orderjson.map((order, index) => {
        let arr = [];

        order.fields.reduce((a, o) => {
            let value = o.value;
            if (-1 == hash_address.indexOf(hexToString(o.name))) { value = hexToString(value); }
            a.push(hexToString(o.name), value); return a;
        }, arr);
        // objs.push(arr);
        return arr;
    })
    // console.log(objs)
    return objs;
}

const accounts = ["Alice"];
const proxy = "proxy";
const makeOrder = (exchange, isMaker, side) => ({
    exchange: exchange,
    maker: accounts[0],
    taker: accounts[0],
    makerRelayerFee: 0,
    takerRelayerFee: 0,
    makerProtocolFee: 0,
    takerProtocolFee: 0,
    feeRecipient: isMaker ? accounts[0] : '0x0000000000000000000000000000000000000000',
    feeMethod: 0,
    side: side,
    saleKind: 0,
    target: proxy,
    howToCall: 0,
    calldata: '0x',
    replacementPattern: '0x',
    staticTarget: '0x0000000000000000000000000000000000000000',
    staticExtradata: '0x',
    paymentToken: accounts[0],
    basePrice: 0,
    extra: 0,
    listingTime: 0,
    expirationTime: 0,
    salt: 0
})

function changes() {
    exchangeInstance.changeMinimumMakerProtocolFee(1);

    exchangeInstance.minimumMakerProtocolFee.call()
    // assert.equal(res.toNumber(), 1, 'Protocol fee was not changed')



    exchangeInstance.changeMinimumTakerProtocolFee(1);
    exchangeInstance.minimumTakerProtocolFee;
    // assert.equal(res.toNumber(), 1, 'Protocol fee was not changed')

    exchangeInstance.changeProtocolFeeRecipient(accounts[1]);
    exchangeInstance.protocolFeeRecipient;
    // assert.equal(res, accounts[1], 'Protocol fee recipient was not changed')

}
function hashtosign() {
    const order = makeOrder(exchangeInstance.address)
    const hash = "";// hashToSign(order)
    exchangeInstance.hashToSign_(
        [order.exchange, order.maker, order.taker, order.feeRecipient, order.target, order.staticTarget, order.paymentToken],
        [order.makerRelayerFee, order.takerRelayerFee, order.makerProtocolFee, order.takerProtocolFee, order.basePrice, order.extra, order.listingTime, order.expirationTime, order.salt],
        order.feeMethod,
        order.side,
        order.saleKind,
        order.howToCall,
        order.calldata,
        order.replacementPattern,
        order.staticExtradata);
    // assert.equal(solHash, hash, 'Hashes were not equal')
}

async function calculateFinalPrice() {
    let time = await promisify(getTime)
    exchangeInstance.calculateFinalPrice.call(1, 1, 100, 100, time, time + 100);
    //   assert.equal(price.toNumber(), 100, 'Incorrect price')
    time = await promisify(getTime)
    exchangeInstance.calculateFinalPrice.call(1, 1, 100, 100, time - 100, time);
    // assert.equal(price.toNumber(), 0, 'Incorrect price')
    time = await promisify(getTime)
    exchangeInstance.calculateFinalPrice.call(0, 1, 100, 100, time - 50, time + 50);
    //   assert.equal(price.toNumber(), 150, 'Incorrect price')
    time = await promisify(getTime)
    exchangeInstance.calculateFinalPrice.call(0, 1, 100, 200, time - 50, time + 50);
    // assert.equal(price.toNumber(), 200, 'Incorrect price')
}
function calculateCurrentPrice_() {
    const order = makeOrder(exchangeInstance.address, true)
    order.saleKind = 0
    order.listingTime = 1
    order.expirationTime = 1000
    const hash = hashOrder(order);
    exchangeInstance.validateOrderParameters_(
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
    // assert.equal(ret, true, 'Order did not validate')
    exchangeInstance.calculateCurrentPrice_.call(
        [order.exchange, order.maker, order.taker, order.feeRecipient, order.target, order.staticTarget, order.paymentToken],
        [order.makerRelayerFee, order.takerRelayerFee, order.makerProtocolFee, order.takerProtocolFee, order.basePrice, order.extra, order.listingTime, order.expirationTime, order.salt],
        order.feeMethod,
        order.side,
        order.saleKind,
        order.howToCall,
        order.calldata,
        order.replacementPattern,
        order.staticExtradata);
    // assert.equal(price.toNumber(), 0, 'Incorrect price')
}
function cancelOrder() {
    // const order = makeOrder(exchangeInstance.address);
    //  exchangeInstance.hashOrder_(
    //           [order.exchange, order.maker, order.taker, order.feeRecipient, order.target, order.staticTarget, order.paymentToken],
    //           [order.makerRelayerFee, order.takerRelayerFee, order.makerProtocolFee, order.takerProtocolFee, order.basePrice, order.extra, order.listingTime, order.expirationTime, order.salt],
    //           order.feeMethod,
    //           order.side,
    //           order.saleKind,
    //           order.howToCall,
    //           order.calldata,
    //           order.replacementPattern,
    //           order.staticExtradata);
    //  exchangeInstance.approveOrder_(
    //               [order.exchange, order.maker, order.taker, order.feeRecipient, order.target, order.staticTarget, order.paymentToken],
    //               [order.makerRelayerFee, order.takerRelayerFee, order.makerProtocolFee, order.takerProtocolFee, order.basePrice, order.extra, order.listingTime, order.expirationTime, order.salt],
    //               order.feeMethod,
    //               order.side,
    //               order.saleKind,
    //               order.howToCall,
    //               order.calldata,
    //               order.replacementPattern,
    //               order.staticExtradata,
    //               true
    //             );
    //  exchangeInstance.validateOrder_(
    //                 [order.exchange, order.maker, order.taker, order.feeRecipient, order.target, order.staticTarget, order.paymentToken],
    //                 [order.makerRelayerFee, order.takerRelayerFee, order.makerProtocolFee, order.takerProtocolFee, order.basePrice, order.extra, order.listingTime, order.expirationTime, order.salt],
    //                 order.feeMethod,
    //                 order.side,
    //                 order.saleKind,
    //                 order.howToCall,
    //                 order.calldata,
    //                 order.replacementPattern,
    //                 order.staticExtradata,
    //                 0, '', '',
    //                 {from: accounts[1]}
    //               );
    //                 // assert.equal(ret, true, 'Order did not validate')
    //                cancelOrder_(
    //                   [order.exchange, order.maker, order.taker, order.feeRecipient, order.target, order.staticTarget, order.paymentToken],
    //                   [order.makerRelayerFee, order.takerRelayerFee, order.makerProtocolFee, order.takerProtocolFee, order.basePrice, order.extra, order.listingTime, order.expirationTime, order.salt],
    //                   order.feeMethod,
    //                   order.side,
    //                   order.saleKind,
    //                   order.howToCall,
    //                   order.calldata,
    //                   order.replacementPattern,
    //                   order.staticExtradata,
    //                   0, '', ''
    //                 );
    // validateOrder_(
    //                     [order.exchange, order.maker, order.taker, order.feeRecipient, order.target, order.staticTarget, order.paymentToken],
    //                     [order.makerRelayerFee, order.takerRelayerFee, order.makerProtocolFee, order.takerProtocolFee, order.basePrice, order.extra, order.listingTime, order.expirationTime, order.salt],
    //                     order.feeMethod,
    //                     order.side,
    //                     order.saleKind,
    //                     order.howToCall,
    //                     order.calldata,
    //                     order.replacementPattern,
    //                     order.staticExtradata,
    //                     0, '', ''
    //                   )
}

// const matchOrder = (buy, sell,  value) => {

//     const buyHash = hashOrder(buy)
//     const sellHash = hashOrder(sell)
//     // return web3.eth.sign(buyHash, accounts[0]).then(signature => {
//     //     signature = signature.substr(2)


// // async signature => {
// //             signature = signature.substr(2)

//             await exchangeInstance.hashOrder_(
//                 [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
//                 [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
//                 buy.feeMethod,
//                 buy.side,
//                 buy.saleKind,
//                 buy.howToCall,
//                 buy.calldata,
//                 buy.replacementPattern,
//                 buy.staticExtradata);
//             await exchangeInstance.hashOrder_(
//                 [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
//                 [sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
//                 sell.feeMethod,
//                 sell.side,
//                 sell.saleKind,
//                 sell.howToCall,
//                 sell.calldata,
//                 sell.replacementPattern,
//                 sell.staticExtradata);
//             exchangeInstance.ordersCanMatch_(
//                 [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken, sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
//                 [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt, sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
//                 [buy.feeMethod, buy.side, buy.saleKind, buy.howToCall, sell.feeMethod, sell.side, sell.saleKind, sell.howToCall],
//                 buy.calldata,
//                 sell.calldata,
//                 buy.replacementPattern,
//                 sell.replacementPattern,
//                 buy.staticExtradata,
//                 sell.staticExtradata
//             );

//             // assert.equal(ret, true, 'Orders were not matchable!')
//             exchangeInstance.calculateMatchPrice_(
//                 [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken, sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
//                 [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt, sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
//                 [buy.feeMethod, buy.side, buy.saleKind, buy.howToCall, sell.feeMethod, sell.side, sell.saleKind, sell.howToCall],
//                 buy.calldata,
//                 sell.calldata,
//                 buy.replacementPattern,
//                 sell.replacementPattern,
//                 buy.staticExtradata,
//                 sell.staticExtradata
//             );
//             // assert.equal(matchPrice.toNumber(), buy.basePrice.toNumber(), 'Incorrect match price!')

//             exchangeInstance.approveOrder_(
//                 [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken],
//                 [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt],
//                 buy.feeMethod,
//                 buy.side,
//                 buy.saleKind,
//                 buy.howToCall,
//                 buy.calldata,
//                 buy.replacementPattern,
//                 buy.staticExtradata,
//                 true
//             );
//             exchangeInstance.approveOrder_(
//                 [sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
//                 [sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
//                 sell.feeMethod,
//                 sell.side,
//                 sell.saleKind,
//                 sell.howToCall,
//                 sell.calldata,
//                 sell.replacementPattern,
//                 sell.staticExtradata,
//                 true
//             );
//             exchangeInstance.atomicMatch_(
//                 [buy.exchange, buy.maker, buy.taker, buy.feeRecipient, buy.target, buy.staticTarget, buy.paymentToken, sell.exchange, sell.maker, sell.taker, sell.feeRecipient, sell.target, sell.staticTarget, sell.paymentToken],
//                 [buy.makerRelayerFee, buy.takerRelayerFee, buy.makerProtocolFee, buy.takerProtocolFee, buy.basePrice, buy.extra, buy.listingTime, buy.expirationTime, buy.salt, sell.makerRelayerFee, sell.takerRelayerFee, sell.makerProtocolFee, sell.takerProtocolFee, sell.basePrice, sell.extra, sell.listingTime, sell.expirationTime, sell.salt],
//                 [buy.feeMethod, buy.side, buy.saleKind, buy.howToCall, sell.feeMethod, sell.side, sell.saleKind, sell.howToCall],
//                 buy.calldata,
//                 sell.calldata,
//                 buy.replacementPattern,
//                 sell.replacementPattern,
//                 buy.staticExtradata,
//                 sell.staticExtradata,
//                 [bv, sv],
//                 [br, bs, sr, ss, '0x0000000000000000000000000000000000000000000000000000000000000000'], { from: value ? accounts[0] : accounts[1], value: value || 0 })

//         }



const orderFromJSON = (order) => {

    const created_date = new Date(`${order.created_date}Z`)

    const fromJSON = {
        hash: order.order_hash || order.hash,
        cancelledOrFinalized: order.cancelled || order.finalized,
        markedInvalid: order.marked_invalid,
        metadata: order.metadata,
        quantity: (order.quantity || 1),
        exchange: order.exchange,
        makerAccount: order.maker,
        takerAccount: order.taker,
        // Use string address to conform to Wyvern Order schema
        maker: order.maker.address,
        taker: order.taker.address,
        makerRelayerFee: (order.maker_relayer_fee),
        takerRelayerFee: (order.taker_relayer_fee),
        makerProtocolFee: (order.maker_protocol_fee),
        takerProtocolFee: (order.taker_protocol_fee),
        makerReferrerFee: (order.maker_referrer_fee || 0),
        waitingForBestCounterOrder: order.fee_recipient.address == 0,
        feeMethod: order.fee_method,
        feeRecipientAccount: order.fee_recipient,
        feeRecipient: order.fee_recipient.address,
        side: order.side,
        saleKind: order.sale_kind,
        target: order.target,
        howToCall: order.how_to_call,
        calldata: order.calldata,
        replacementPattern: order.replacement_pattern,
        staticTarget: order.static_target,
        staticExtradata: order.static_extradata,
        paymentToken: order.payment_token,
        basePrice: (order.base_price),
        extra: (order.extra),
        currentBounty: (order.current_bounty || 0),
        currentPrice: (order.current_price || 0),

        createdTime: (Math.round(created_date.getTime() / 1000)),
        listingTime: (order.listing_time),
        expirationTime: (order.expiration_time),

        salt: (order.salt)
        // v: parseInt(order.v),
        // r: order.r,
        // s: order.s,

        // paymentTokenContract: order.payment_token_contract ? tokenFromJSON(order.payment_token_contract) : undefined,
        // asset: order.asset ? assetFromJSON(order.asset) : undefined,
        // assetBundle: order.asset_bundle ? assetBundleFromJSON(order.asset_bundle) : undefined,
    }
}

export { makeOrderArrayEx, makeOrderArrayHexEx, makeOrderFromJSONHex, makeOrderEx, makeOrder, orderFromJSON };