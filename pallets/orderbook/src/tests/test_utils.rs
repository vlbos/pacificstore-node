// Tests to be written here

use super::*;
pub fn store_test_order<T: Trait>(
    index: u64,
    order_id: OrderId,
    owner: T::AccountId,
    created_date: T::Moment,
) {
    let order_fields = get_test_order();
    Orders::<T>::insert(
        index,
        OrderJSONType {
            index,
            order_id: order_id.clone(),
            owner,
            created_date,
            fields: Some(order_fields),
        },
    );

    store_test_order_index::<Test>(index, order_id);
}

pub fn store_test_order_index<T: Trait>(index: u64, order_id: OrderId) {
    OrderIndices::insert(order_id, index);
}

pub fn get_test_order() -> Vec<OrderField> {
    vec![
OrderField::new(b"created_date", b"2019-01-29T04:04:03.258323"),
OrderField::new(b"order_hash", b"0x3f8d16507c4d9905815e860324d64b9c9f5933a70e59c2a07a63320459f67826"),
OrderField::new(b"metadata.asset.id", b"505"),
OrderField::new(b"metadata.asset.address", b"0x16baf0de678e52367adc69fd067e5edd1d33e3bf"),
OrderField::new(b"metadata.schema", b"ERC721"),
OrderField::new(b"exchange", b"0x5206e78b21ce315ce284fb24cf05e0585a93b1d9"),
OrderField::new(b"maker.user.username", b"alex2"),
OrderField::new(b"maker.profile_img_url", b"https://storage.googleapis.com/opensea-static/opensea-profile/11.png"),
OrderField::new(b"maker.address", b"0xe96a1b303a1eb8d04fb973eb2b291b8d591c8f72"),
OrderField::new(b"maker.config", b"affiliate"),
OrderField::new(b"taker.user", b"null"),
OrderField::new(b"taker.profile_img_url", b"https://storage.googleapis.com/opensea-static/opensea-profile/1.png"),
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
OrderField::new(b"fee_recipient.profile_img_url", b"https://storage.googleapis.com/opensea-static/opensea-profile/1.png"),
OrderField::new(b"fee_recipient.address", b"0x0000000000000000000000000000000000000000"),
OrderField::new(b"fee_recipient.config", b""),
OrderField::new(b"fee_method", b"1"),
OrderField::new(b"side", b"1"),
OrderField::new(b"sale_kind", b"0"),
OrderField::new(b"target", b"0x16baf0de678e52367adc69fd067e5edd1d33e3bf"),
OrderField::new(b"how_to_call", b"0"),
OrderField::new(b"calldata", b"0x23b872dd000000000000000000000000e96a1b303a1eb8d04fb973eb2b291b8d591c8f72000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001f9"),
OrderField::new(b"replacement_pattern", b"0x000000000000000000000000000000000000000000000000000000000000000000000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000000000000000000000000000000000000000000000000000000000"),
OrderField::new(b"static_target", b"0x0000000000000000000000000000000000000000"),
OrderField::new(b"static_extradata", b"0x"),
OrderField::new(b"payment_token", b"0xc778417e063141139fce010982780140aa0cd5ab"),
OrderField::new(b"payment_token_contract.address", b"0xc778417e063141139fce010982780140aa0cd5ab"),
OrderField::new(b"payment_token_contract.image_url", b"null"),
OrderField::new(b"payment_token_contract.name", b"Wrapped Ether"),
OrderField::new(b"payment_token_contract.symbol", b"WETH"),
OrderField::new(b"payment_token_contract.decimals", b"18"),
OrderField::new(b"payment_token_contract.eth_price", b"1.000000000000000"),
OrderField::new(b"base_price", b"10000000000000000"),
OrderField::new(b"extra", b"0"),
OrderField::new(b"listing_time", b"1548734810"),
OrderField::new(b"expiration_time", b"0"),
OrderField::new(b"salt", b"83006245783548033686093530747847303952463217644495033304999143031082661844460"),
OrderField::new(b"v", b"28"),
OrderField::new(b"r", b"0x2a0b0f3b8e6705cdf7894d9f1fb547646c5502a9d1d993c308ed0310620cf660"),
OrderField::new(b"s", b"0x19211a9a0c3ab3bb94b840774a2f9badf637b95d90b68965a4cf3734d5eaba98"),
OrderField::new(b"cancelled", b"false"),
OrderField::new(b"finalized", b"false"),
OrderField::new(b"marked_invalid", b"false"),
OrderField::new(b"prefixed_hash", b"0x98a07dfb9e4da7ffc0ad0fb230afc8684dc4a0ac44623eded6a4c42e1df99954"),
            ]
}
