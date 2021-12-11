// Tests to be written here

use super::*;

pub fn store_test_order_index<T: Trait>(index: u64, order_id: OrderId) {
    OrderIndices::insert(order_id, index);
}

pub fn get_test_order() -> Vec<OrderField> {
    vec![
        OrderField::new(b"created_date", b"2019-01-29T04:04:03.258323"),
        OrderField::new(
            b"order_hash", 
            b"0x3f8d16507c4d9905815e860324d64b9c9f5933a70e59c2a07a63320459f67826",
        ),
        OrderField::new(b"metadata.asset.id", b"505"),
        OrderField::new(b"metadata.asset.address", b"0x16baf0de678e52367adc69fd067e5edd1d33e3bf"),
        OrderField::new(b"metadata.schema", b"ERC721"),
        OrderField::new(b"exchange", b"0x5206e78b21ce315ce284fb24cf05e0585a93b1d9"),
        OrderField::new(b"maker.user.username", b"alex2"),
        OrderField::new(
            b"maker.profile_img_url", 
            b"https://storage.googleapis.com/opensea-static/opensea-profile/11.png",
        ),
        OrderField::new(b"maker.address", b"0xe96a1b303a1eb8d04fb973eb2b291b8d591c8f72"),
        OrderField::new(b"maker.config", b"affiliate"),
        OrderField::new(b"taker.user", b"null"),
        OrderField::new(
            b"taker.profile_img_url", 
            b"https://storage.googleapis.com/opensea-static/opensea-profile/1.png",
        ),
        OrderField::new(b"taker.address", b"0x0000000000000000000000000000000000000000"),
        OrderField::new(b"taker.config", b""),
        OrderField::new(b"current_price", b"10000000000000000"),
        OrderField::new(b"current_bounty", b"100000000000000.0"),
        OrderField::new(b"maker_relayer_fee", b"100"),
        OrderField::new(b"taker_relayer_fee", b"250"),
        OrderField::new(b"maker_protocol_fee", b"0"),
        OrderField::new(b"taker_protocol_fee", b"0"),
        OrderField::new(b"maker_referrer_fee", b"0"),
        OrderField::new(b"fee_recipient.user", b"null"),
        OrderField::new(
            b"fee_recipient.profile_img_url", 
            b"https://storage.googleapis.com/opensea-static/opensea-profile/1.png",
        ),
        OrderField::new(b"fee_recipient.address", b"0x0000000000000000000000000000000000000000"),
        OrderField::new(b"fee_recipient.config", b""),
        OrderField::new(b"fee_method", b"1"),
        OrderField::new(b"side", b"1"),
        OrderField::new(b"sale_kind", b"0"),
        OrderField::new(b"target", b"0x16baf0de678e52367adc69fd067e5edd1d33e3bf"),
        OrderField::new(b"how_to_call", b"0"),
        OrderField::new(
            b"calldata", 
            b"0x23b872dd000000000000000000000000e96a1b303a1eb8d04fb973eb2b291b8d591c\
            8f7200000000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000001f9",
        ),
        OrderField::new(
            b"replacement_pattern", 
            b"0x000000000000000000000000000000000000000000000000000000000000000000000000\
            ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
            0000000000000000000000000000000000000000000000000000000000000000",
        ),
        OrderField::new(b"static_target", b"0x0000000000000000000000000000000000000000"),
        OrderField::new(b"static_extradata", b"0x"),
        OrderField::new(b"payment_token", b"0xc778417e063141139fce010982780140aa0cd5ab"),
        OrderField::new(
            b"payment_token_contract.address", 
            b"0xc778417e063141139fce010982780140aa0cd5ab",
        ),
        OrderField::new(b"payment_token_contract.image_url", b"null"),
        OrderField::new(b"payment_token_contract.name", b"Wrapped Ether"),
        OrderField::new(b"payment_token_contract.symbol", b"WETH"),
        OrderField::new(b"payment_token_contract.decimals", b"18"),
        OrderField::new(b"payment_token_contract.eth_price", b"1.000000000000000"),
        OrderField::new(b"base_price", b"10000000000000000"),
        OrderField::new(b"extra", b"0"),
        OrderField::new(b"listing_time", b"1548734810"),
        OrderField::new(b"expiration_time", b"0"),
        OrderField::new(
            b"salt", 
            b"83006245783548033686093530747847303952463217644495033304999143031082661844460",
        ),
        OrderField::new(b"v", b"28"),
        OrderField::new(
            b"r", 
            b"0x2a0b0f3b8e6705cdf7894d9f1fb547646c5502a9d1d993c308ed0310620cf660",
        ),
        OrderField::new(
            b"s", 
            b"0x19211a9a0c3ab3bb94b840774a2f9badf637b95d90b68965a4cf3734d5eaba98",
        ),
        OrderField::new(b"cancelled", b"false"),
        OrderField::new(b"finalized", b"false"),
        OrderField::new(b"marked_invalid", b"false"),
        OrderField::new(
            b"prefixed_hash", 
            b"0x98a07dfb9e4da7ffc0ad0fb230afc8684dc4a0ac44623eded6a4c42e1df99954",
        ),
]
}




pub fn get_test_orders() -> Vec<Vec<OrderField>> {
let order_array = [
  [
    [ b"created_date".to_vec(), b"2019-01-29T04:04:03.258323".to_vec() ],
    [
        b"order_hash".to_vec(),
        b"0x3f8d16507c4d9905815e860324d64b9c9f5933a70e59c2a07a63320459f67826".to_vec()
    ],
    [ b"metadata.asset.id".to_vec(), b"505".to_vec() ],
    [
        b"metadata.asset.address".to_vec(),
        b"0x16baf0de678e52367adc69fd067e5edd1d33e3bf".to_vec()
    ],
    [ b"metadata.schema".to_vec(), b"ERC721".to_vec() ],
    [ b"exchange".to_vec(), b"0x5206e78b21ce315ce284fb24cf05e0585a93b1d9".to_vec() ],
    [ b"maker.user.username".to_vec(), b"alex2".to_vec() ],
    [
        b"maker.profile_img_url".to_vec(),
        b"https://storage.googleapis.com/opensea-static/opensea-profile/11.png".to_vec(),
    ],
    [
        b"maker.address".to_vec(),
        b"0xe96a1b303a1eb8d04fb973eb2b291b8d591c8f72".to_vec()
    ],
    [ b"maker.config".to_vec(), b"affiliate".to_vec() ],
    [ b"taker.user".to_vec(), b"null".to_vec() ],
    [
        b"taker.profile_img_url".to_vec(),
        b"https://storage.googleapis.com/opensea-static/opensea-profile/1.png".to_vec(),
    ],
    [
        b"taker.address".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec(),
    ],
    [ b"taker.config".to_vec(), b"".to_vec() ],
    [ b"current_price".to_vec(), b"10000000000000000".to_vec() ],
    [ b"current_bounty".to_vec(), b"100000000000000.0".to_vec() ],
    [ b"maker_relayer_fee".to_vec(), b"100".to_vec() ],
    [ b"taker_relayer_fee".to_vec(), b"250".to_vec() ],
    [ b"maker_protocol_fee".to_vec(), b"0".to_vec() ],
    [ b"taker_protocol_fee".to_vec(), b"0".to_vec() ],
    [ b"maker_referrer_fee".to_vec(), b"0".to_vec() ],
    [ b"fee_recipient.user".to_vec(), b"null".to_vec() ],
    [
        b"fee_recipient.profile_img_url".to_vec(),
        b"https://storage.googleapis.com/opensea-static/opensea-profile/1.png".to_vec(),
    ],
    [
        b"fee_recipient.address".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec(),
    ],
    [ b"fee_recipient.config".to_vec(), b"".to_vec() ],
    [ b"fee_method".to_vec(), b"1".to_vec() ],
    [ b"side".to_vec(), b"1".to_vec() ],
    [ b"sale_kind".to_vec(), b"0".to_vec() ],
    [ b"target".to_vec(), b"0x16baf0de678e52367adc69fd067e5edd1d33e3bf".to_vec() ],
    [ b"how_to_call".to_vec(), b"0".to_vec() ],
    [
        b"calldata".to_vec(),
        b"0x23b872dd000000000000000000000000e96a1b303a1eb8d04fb973eb2b291b8d591c8f72\
        00000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000001f9".to_vec(),
    ],
    [
        b"replacement_pattern".to_vec(),
        b"0x000000000000000000000000000000000000000000000000000000000000000000000000\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        0000000000000000000000000000000000000000000000000000000000000000".to_vec(),
    ],
    [
        b"static_target".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec(),
    ],
    [ b"static_extradata".to_vec(), b"".to_vec() ],
    [
        b"payment_token".to_vec(),
        b"0xc778417e063141139fce010982780140aa0cd5ab".to_vec(),
    ],
    [
        b"payment_token_contract.address".to_vec(),
        b"0xc778417e063141139fce010982780140aa0cd5ab".to_vec(),
    ],
    [ b"payment_token_contract.image_url".to_vec(), b"null".to_vec() ],
    [ b"payment_token_contract.name".to_vec(), b"Wrapped Ether".to_vec() ],
    [ b"payment_token_contract.symbol".to_vec(), b"WETH".to_vec() ],
    [ b"payment_token_contract.decimals".to_vec(), b"18".to_vec() ],
    [ b"payment_token_contract.eth_price".to_vec(), b"1.000000000000000".to_vec() ],
    [ b"base_price".to_vec(), b"10000000000000000".to_vec() ],
    [ b"extra".to_vec(), b"0".to_vec() ],
    [ b"listing_time".to_vec(), b"1548734810".to_vec() ],
    [ b"expiration_time".to_vec(), b"0".to_vec() ],
    [
        b"salt".to_vec(),
        b"83006245783548033686093530747847303952463217644495033304999143031082661844460".to_vec(),
    ],
    [ b"v".to_vec(), b"28".to_vec() ],
    [
        b"r".to_vec(),
        b"0x2a0b0f3b8e6705cdf7894d9f1fb547646c5502a9d1d993c308ed0310620cf660".to_vec(),
    ],
    [
        b"s".to_vec(),
        b"0x19211a9a0c3ab3bb94b840774a2f9badf637b95d90b68965a4cf3734d5eaba98".to_vec(),
    ],
    [ b"cancelled".to_vec(), b"false".to_vec() ],
    [ b"finalized".to_vec(), b"false".to_vec() ],
    [ b"marked_invalid".to_vec(), b"false".to_vec() ],
    [
        b"prefixed_hash".to_vec(),
        b"0x98a07dfb9e4da7ffc0ad0fb230afc8684dc4a0ac44623eded6a4c42e1df99954".to_vec(),
    ]
  ].to_vec(),
  [
    [ b"exchange".to_vec(), b"0x0000000000000000000000000000000000000000".to_vec() ],
    [
        b"maker.address".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec(),
    ],
    [
        b"taker.address".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec(),
    ],
    [ b"maker_relayer_fee".to_vec(), b"0".to_vec() ],
    [ b"taker_relayer_fee".to_vec(), b"0".to_vec() ],
    [ b"maker_protocol_fee".to_vec(), b"0".to_vec() ],
    [ b"taker_protocol_fee".to_vec(), b"0".to_vec() ],
    [
        b"fee_recipient.address".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec(),
    ],
    [ b"fee_method".to_vec(), b"0".to_vec() ],
    [ b"side".to_vec(), b"0".to_vec() ],
    [ b"sale_kind".to_vec(), b"0".to_vec() ],
    [ b"target".to_vec(), b"0x0000000000000000000000000000000000000000".to_vec() ],
    [ b"how_to_call".to_vec(), b"0".to_vec() ],
    [ b"calldata".to_vec(), b"".to_vec() ],
    [ b"replacement_pattern".to_vec(), b"".to_vec() ],
    [
        b"static_target".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec()
    ],
    [ b"static_extradata".to_vec(), b"".to_vec() ],
    [
        b"payment_token".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec()
    ],
    [ b"base_price".to_vec(), b"0".to_vec() ],
    [ b"extra".to_vec(), b"0".to_vec() ],
    [ b"listing_time".to_vec(), b"0".to_vec() ],
    [ b"expiration_time".to_vec(), b"0".to_vec() ],
    [ b"salt".to_vec(), b"0".to_vec() ],
    [
        b"order_hash".to_vec(),
        b"0x611bdaa1abb525bcc8a261575b72b1dd796cd309cfbf25783df67d88385e458a".to_vec(),
    ]
  ].to_vec(),
  [
    [ b"exchange".to_vec(), b"0x3177ea64b90543b5706f6661549fd4bd8baebb1e".to_vec() ],
    [
        b"maker.address".to_vec(),
        b"0x065abe5f01cf94d37762780695cf19b151ed5809".to_vec()
    ],
    [
        b"taker.address".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec()
    ],
    [ b"maker_relayer_fee".to_vec(), b"0".to_vec() ],
    [ b"taker_relayer_fee".to_vec(), b"0".to_vec() ],
    [ b"maker_protocol_fee".to_vec(), b"0".to_vec() ],
    [ b"taker_protocol_fee".to_vec(), b"0".to_vec() ],
    [ b"fee_method".to_vec(), b"0".to_vec() ],
    [
        b"fee_recipient.address".to_vec(),
        b"0x11db40014e2985c360b3f2a4ba350fbf104dc326".to_vec()
    ],
    [ b"side".to_vec(), b"0".to_vec() ],
    [ b"sale_kind".to_vec(), b"0".to_vec() ],
    [ b"target".to_vec(), b"0x16baf0de678e52367adc69fd067e5edd1d33e3bf".to_vec() ],
    [ b"how_to_call".to_vec(), b"0".to_vec() ],
    [
        b"calldata".to_vec(),
        b"0x23b872dd0000000000000000000000001111111111111111111111111111111111111111\
        000000000000000000000000065abe5f01cf94d37762780695cf19b151ed5809\
        000000000000000000000000000000000000000000000000000000000000006f".to_vec(),
    ],
    [
        b"replacement_pattern".to_vec(),
        b"0x00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        0000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000".to_vec(),
    ],
    [
        b"static_target".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec(),
    ],
    [ b"static_extradata".to_vec(), b"".to_vec() ],
    [
        b"payment_token".to_vec(),
        b"0xc778417e063141139fce010982780140aa0cd5ab".to_vec(),
    ],
    [ b"base_price".to_vec(), b"20000000000000000".to_vec() ],
    [ b"extra".to_vec(), b"0".to_vec() ],
    [ b"listing_time".to_vec(), b"1528769775".to_vec() ],
    [ b"expiration_time".to_vec(), b"0".to_vec() ],
    [
        b"salt".to_vec(),
        b"53846990274470578006430928947393045524278076945060732094139894196992264138607".to_vec(),
    ],
    [
        b"order_hash".to_vec(),
        b"0x9ff08c3956db7cc4fcdad563e65f280ec561dd68e84d6b212ccde09c712a9aba".to_vec(),
    ],
    [ b"metadata.asset.id".to_vec(), b"111".to_vec() ],
    [
        b"metadata.asset.address".to_vec(),
        b"0x16baf0de678e52367adc69fd067e5edd1d33e3bf".to_vec(),
    ],
    [ b"metadata.schema".to_vec(), b"ERC721".to_vec() ]
  ].to_vec(),
  [
    [
        b"order_hash".to_vec(),
        b"0x5045ff865c3b1a0ca4c99e760127b6b969979863f2d3417f4ce06b3e448d7b5a".to_vec(),
    ],
    [ b"metadata.asset.id".to_vec(), b"8576".to_vec() ],
    [
        b"metadata.asset.address".to_vec(),
        b"0xcfbc9103362aec4ce3089f155c2da2eea1cb7602".to_vec(),
    ],
    [ b"metadata.schema".to_vec(), b"ERC721".to_vec() ],
    [ b"exchange".to_vec(), b"0x7be8076f4ea4a4ad08075c2508e481d6c946d12b".to_vec() ],
    [ b"maker.user".to_vec(), b"462".to_vec() ],
    [
        b"maker.profile_img_url".to_vec(),
        b"https://storage.googleapis.com/opensea-static/opensea-profile/30.png".to_vec(),
    ],
    [
        b"maker.address".to_vec(),
        b"0x223edbc8166ba1b514729261ff53fb8c73ab4d79".to_vec(),
    ],
    [ b"maker.config".to_vec(), b"".to_vec() ],
    [ b"taker.user".to_vec(), b"1766".to_vec() ],
    [
        b"taker.profile_img_url".to_vec(),
        b"https://storage.googleapis.com/opensea-static/opensea-profile/1.png".to_vec(),
    ],
    [
        b"taker.address".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec(),
    ],
    [ b"taker.config".to_vec(), b"".to_vec() ],
    [ b"current_price".to_vec(), b"7174257768513290".to_vec() ],
    [ b"maker_relayer_fee".to_vec(), b"250".to_vec() ],
    [ b"taker_relayer_fee".to_vec(), b"0".to_vec() ],
    [ b"maker_protocol_fee".to_vec(), b"0".to_vec() ],
    [ b"taker_protocol_fee".to_vec(), b"0".to_vec() ],
    [ b"fee_recipient.user".to_vec(), b"null".to_vec() ],
    [
        b"fee_recipient.profile_img_url".to_vec(),
        b"https://storage.googleapis.com/opensea-static/opensea-profile/28.png".to_vec(),
    ],
    [
        b"fee_recipient.address".to_vec(),
        b"0x5b3256965e7c3cf26e11fcaf296dfc8807c01073".to_vec()
    ],
    [ b"fee_recipient.config".to_vec(), b"".to_vec() ],
    [ b"fee_method".to_vec(), b"1".to_vec() ],
    [ b"side".to_vec(), b"1".to_vec() ],
    [ b"sale_kind".to_vec(), b"1".to_vec() ],
    [ b"target".to_vec(), b"0xcfbc9103362aec4ce3089f155c2da2eea1cb7602".to_vec() ],
    [ b"how_to_call".to_vec(), b"0".to_vec() ],
    [
        b"calldata".to_vec(),
        b"0x23b872dd000000000000000000000000223edbc8166ba1b514729261ff53fb8c73ab4d79\
0000000000000000000000000000000000000000000000000000000000000000000000000000000000\
0000000000000000000000000000000000000000002180".to_vec(),
    ],
    [
        b"replacement_pattern".to_vec(),
        b"0x000000000000000000000000000000000000000000000000000000000000000000000000\
ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
0000000000000000000000000000000000000000000000000000000000000000".to_vec(),
    ],
    [
        b"static_target".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec()
    ],
    [ b"static_extradata".to_vec(), b"".to_vec() ],
    [
        b"payment_token".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec()
    ],
    [ b"base_price".to_vec(), b"8500000000000000".to_vec() ],
    [ b"extra".to_vec(), b"3000000000000001".to_vec() ],
    [ b"listing_time".to_vec(), b"1534620227".to_vec() ],
    [ b"expiration_time".to_vec(), b"1535829926".to_vec() ],
    [
        b"salt".to_vec(),
        b"79790726582490632926414916888561832909222048588204370740711320601991661022189".to_vec(),
    ],
    [ b"v".to_vec(), b"27".to_vec() ],
    [
        b"r".to_vec(),
        b"0x6670e13713f0c748b7b679d1263ee444d9ae888ff8a51e86eee308b38284b56b".to_vec(),
    ],
    [
        b"s".to_vec(),
        b"0x722915ae24400e802ee8d71683629f3ef271517cabcf6b25240719680b841afe".to_vec(),
    ],
    [ b"cancelled".to_vec(), b"false".to_vec() ],
    [ b"finalized".to_vec(), b"false".to_vec() ],
    [ b"marked_invalid".to_vec(), b"false".to_vec() ],
    [
        b"prefixed_hash".to_vec(),
        b"0x044665f3aca521068b7e2b4b7497c541fcbedf73f64f7fd78d45e74be3b3e6a6".to_vec(),
    ]
  ].to_vec(),
  [
    [
        b"order_hash".to_vec(),
        b"0xa1d001b2bf6db176d53cf58433eccebf35c964ab5c52d1a01870c1346a105fcd".to_vec(),
    ],
    [ b"metadata.asset.id".to_vec(), b"764".to_vec() ],
    [
        b"metadata.asset.address".to_vec(),
        b"0xcfbc9103362aec4ce3089f155c2da2eea1cb7602".to_vec(),
    ],
    [ b"metadata.schema".to_vec(), b"ERC721".to_vec() ],
    [ b"exchange".to_vec(), b"0x7be8076f4ea4a4ad08075c2508e481d6c946d12b".to_vec() ],
    [ b"maker.user.username".to_vec(), b"wanderer".to_vec() ],
    [
        b"maker.profile_img_url".to_vec(),
        b"https://storage.googleapis.com/opensea-static/opensea-profile/4.png".to_vec(),
    ],
    [
        b"maker.address".to_vec(),
        b"0x6be4a7bbb812bfa6a63126ee7b76c8a13529bdb8".to_vec(),
    ],
    [ b"maker.config".to_vec(), b"".to_vec() ],
    [ b"taker.user.username".to_vec(), b"NullAddress".to_vec() ],
    [
        b"taker.profile_img_url".to_vec(),
        b"https://storage.googleapis.com/opensea-static/opensea-profile/1.png".to_vec(),
    ],
    [
        b"taker.address".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec(),
    ],
    [ b"taker.config".to_vec(), b"".to_vec() ],
    [ b"current_price".to_vec(), b"0.01".to_vec() ],
    [ b"maker_relayer_fee".to_vec(), b"0".to_vec() ],
    [ b"taker_relayer_fee".to_vec(), b"0".to_vec() ],
    [ b"maker_protocol_fee".to_vec(), b"0".to_vec() ],
    [ b"taker_protocol_fee".to_vec(), b"0".to_vec() ],
    [ b"fee_recipient.user".to_vec(), b"null".to_vec() ],
    [
        b"fee_recipient.profile_img_url".to_vec(),
        b"https://storage.googleapis.com/opensea-static/opensea-profile/20.png".to_vec(),
    ],
    [
        b"fee_recipient.address".to_vec(),
        b"0x11db40014e2985c360b3f2a4ba350fbf104dc326".to_vec()
    ],
    [ b"fee_recipient.config".to_vec(), b"".to_vec() ],
    [ b"fee_method".to_vec(), b"0".to_vec() ],
    [ b"side".to_vec(), b"0".to_vec() ],
    [ b"sale_kind".to_vec(), b"0".to_vec() ],
    [ b"target".to_vec(), b"0xcfbc9103362aec4ce3089f155c2da2eea1cb7602".to_vec() ],
    [ b"how_to_call".to_vec(), b"0".to_vec() ],
    [
        b"calldata".to_vec(),
        b"0x23b872dd0000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000006be4a7bbb812bfa6a63126ee7b76c8a13529bdb8\
        00000000000000000000000000000000000000000000000000000000000002fc".to_vec(),
    ],
    [
        b"replacement_pattern".to_vec(),
        b"0x00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        0000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000".to_vec(),
    ],
    [
        b"static_target".to_vec(),
        b"0x0000000000000000000000000000000000000000".to_vec(),
    ],
    [ b"static_extradata".to_vec(), b"".to_vec() ],
    [
        b"payment_token".to_vec(),
        b"0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_vec(),
    ],
    [ b"base_price".to_vec(), b"10000000000000000".to_vec() ],
    [ b"extra".to_vec(), b"0".to_vec() ],
    [ b"listing_time".to_vec(), b"1529186191".to_vec() ],
    [ b"expiration_time".to_vec(), b"0".to_vec() ],
    [
        b"salt".to_vec(),
        b"40568100110604210101393766805325371426576667098111185846452878965970704361888".to_vec(),
    ],
    [ b"v".to_vec(), b"28".to_vec() ],
    [
        b"r".to_vec(),
        b"0x96ccf4bb243e5c95d9f72364c0fd8daeb036e791c9244bb8c7c9ce1b41f78692".to_vec(),
    ],
    [
        b"s".to_vec(),
        b"0x56c515bd9c3292864c1dd0b4baebd0aae8610b3fb38597a77a54228297fb9f58".to_vec(),
    ],
    [ b"cancelled".to_vec(), b"false".to_vec() ],
    [ b"finalized".to_vec(), b"false".to_vec() ],
    [ b"marked_invalid".to_vec(), b"false".to_vec() ],
    [ b"prefixed_hash".to_vec(), b"null".to_vec() ]
  ].to_vec()
];

order_array.to_vec().into_iter()
.map(|order| order.into_iter()
.map(|field|OrderField::new(&field.get(0).unwrap(),&field.get(1).unwrap())).collect())
.collect()

}