// A demonstration of interacting with custom RPCs using Polkadot js API

const { ApiPromise, WsProvider , Keyring} = require('@polkadot/api');
const { stringToHex,stringToU8a } =  require ('@polkadot/util');
const { Bytes, Option, u32, Vec } =  require ( '@polkadot/types');
const { v4 : uuidv4 } =  require (  'uuid');
const { readFileSync } = require('fs');

// Construct parameters for API instance
const wsProvider = new WsProvider('ws://localhost:9944');
const  types  =  require('./types');
const rpcs = require(`./rpcs.json`);
const rpc = {  ...rpcs  };
// let  types = {};
try{
//   types = JSON.parse(readFileSync('/Users/lisheng/mygit/vlbos/pacificstore/v/substrate-enterprise-sample/scripts/js/types.json', 'utf8'));
// Object.assign(types,stypes);
}
catch(error)
{
console.log(error);
}
// const rpc = {
//   silly: {
//     seven: {
//       description: "Always returns 7",
//       params: [],
//       type: "u32",
//     },
//     double: {
//       description: "Doubles the parameter",
//       params: [
//         {
//           name: "val",
//           type: "u32",
//         }
//       ],
//       type: "u32",
//     }
//   },
//   sumStorage: {
//     getSum: {
//       description: "Gets the sum of the two storage values in sum-storage pallet via a runtime api.",
//       params: [],
//       type: "u32",
//     }
//   }
// }

async function main() {
  // Construct the actual api
  const api = await ApiPromise.create({
    provider: wsProvider,
    types,
    rpc,
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


//   let silly7 = await api.rpc.orderbook.getOrders({
//       limit:1,
//       offset: 1,
//       owner: stringToU8a(users.francis.key.publicKey),
//       token_ids: [stringToU8a('dddddddddddddddddddddddddd')],
//       params: [[[0x1],[0x1]]]
//     });
//   console.log(`The value from the silly_seven is ${silly7}\n`);


  let silly7 = await api.rpc.wyvernExchange.validateOrderParametersEx([users.francis.key.address],
      [1],
      'ProtocolFee',
      'Buy',
     'FixedPrice',
'Call',
       uuidv4(),
 uuidv4(),
 uuidv4(),
    );
  console.log(`The value from the silly_seven is ${silly7}\n`);


//   // Query the custom SillyRpc
//   let silly7 = await api.rpc.silly.seven();
//   let silly14 = await api.rpc.silly.double(7);
//   console.log(`The value from the silly_seven is ${silly7}\n`);
//   console.log(`The double of 7 according to silly_double is ${silly14}\n`);

//   // Query raw storage values, the oldschool way
//   const v1 = ( await api.query.sumStorage.thing1() ).toNumber();
//   const v2 = ( await api.query.sumStorage.thing2() ).toNumber();
//   console.log(`The individual storage values are ${v1}, and ${v2}.`);
//   console.log(`The sum calculated in javascript is ${v1 + v2}\n`);

//   // Query the custom RPC that uses the runtimeAPI
//   let directSum = ( await api.rpc.sumStorage.getSum() ).toNumber();
//   console.log(`The sum queried directly from the RPC is ${directSum}`);


}

main().catch(console.error).finally(() => process.exit());
