// Tests to be written here

use super::*;

#[test]
fn approve_order_ex() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);

        create_account_test(sender);
        create_account_test(sender1);
        let order = make_order(sender, sender, sender, 0);
        let addrs = vec![
            order.exchange,
            order.maker,
            order.taker,
            order.fee_recipient,
            order.target,
            order.static_target,
            order.payment_token,
        ]
        .to_vec();
        let uints = vec![
            order.maker_relayer_fee,
            order.taker_relayer_fee,
            order.maker_protocol_fee,
            order.taker_protocol_fee,
            order.base_price,
            order.extra,
            order.listing_time,
            order.expiration_time,
            order.salt,
        ]
        .to_vec();
        // let addrs =  Vec::<<Test as system::Trait>::AccountId>::new();
        //    let     uints =  Vec::<u32>::new();
        let fee_method = FeeMethod::from(0);
        let side = Side::from(0);
        let sale_kind = SaleKind::from(0);
        let how_to_call = HowToCall::from(0);
        let calldata = Vec::<u8>::new();
        let replacement_pattern = Vec::<u8>::new();
        let static_extradata = Vec::<u8>::new();
        let orderbook_inclusion_desired: bool = false;

        let result = WyvernExchange::approve_order_ex(
            Origin::signed(sender),
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            calldata,
            replacement_pattern,
            static_extradata,
            orderbook_inclusion_desired,
        );

        assert_ok!(result);
    });
}

#[test]
fn cancel_order_ex() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);

        create_account_test(sender);
        create_account_test(sender1);
        <ContractSelf<Test>>::put(sender);
        let order = make_order(sender, sender, sender, 0);
        let addrs = vec![
            order.exchange,
            order.maker,
            order.taker,
            order.fee_recipient,
            order.target,
            order.static_target,
            order.payment_token,
        ]
        .to_vec();
        let uints = vec![
            order.maker_relayer_fee,
            order.taker_relayer_fee,
            order.maker_protocol_fee,
            order.taker_protocol_fee,
            order.base_price,
            order.extra,
            order.listing_time,
            order.expiration_time,
            order.salt,
        ]
        .to_vec();

        let fee_method = FeeMethod::from(0);
        let side = Side::from(0);
        let sale_kind = SaleKind::from(0);
        let how_to_call = HowToCall::from(0);
        let calldata = Vec::<u8>::new();
        let replacement_pattern = Vec::<u8>::new();
        let static_extradata = Vec::<u8>::new();
        let sig = Signature::default();

        let result = WyvernExchange::cancel_order_ex(
            Origin::signed(sender),
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            calldata,
            replacement_pattern,
            static_extradata,
            sig,
        );

        assert_ok!(result);
    });
}

#[test]
fn atomic_match_ex() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);
        System::set_block_number(1);
        Timestamp::set_timestamp(100);
        run_to_block(100);
        create_account_test(sender);
        create_account_test(sender1);
        <ContractSelf<Test>>::put(sender);
        let buy = make_order(sender, sender, sender, 0);
        let sell = make_order(sender1, sender, sender1, 1);
        let addrs = vec![
            buy.exchange,
            buy.maker,
            buy.taker,
            buy.fee_recipient,
            buy.target,
            buy.static_target,
            buy.payment_token,
            sell.exchange,
            sell.maker,
            sell.taker,
            sell.fee_recipient,
            sell.target,
            sell.static_target,
            sell.payment_token,
        ]
        .to_vec();
        let uints = vec![
            buy.maker_relayer_fee,
            buy.taker_relayer_fee,
            buy.maker_protocol_fee,
            buy.taker_protocol_fee,
            buy.base_price,
            buy.extra,
            buy.listing_time,
            buy.expiration_time,
            buy.salt,
            sell.maker_relayer_fee,
            sell.taker_relayer_fee,
            sell.maker_protocol_fee,
            sell.taker_protocol_fee,
            sell.base_price,
            sell.extra,
            sell.listing_time,
            sell.expiration_time,
            sell.salt,
        ]
        .to_vec();

        let fee_methods_sides_kinds_how_to_calls: Vec<u8> = vec![
            buy.fee_method.value(),
            buy.side.value(),
            buy.sale_kind.value(),
            buy.how_to_call.value(),
            sell.fee_method.value(),
            sell.side.value(),
            sell.sale_kind.value(),
            sell.how_to_call.value(),
        ]
        .to_vec();

        let calldata_buy = Vec::<u8>::new();
        let calldata_sell = Vec::<u8>::new();
        let replacement_pattern_buy = Vec::<u8>::new();
        let replacement_pattern_sell = Vec::<u8>::new();
        let static_extradata_buy = Vec::<u8>::new();
        let static_extradata_sell = Vec::<u8>::new();
        let sig = vec![Signature::default(), Signature::default()];
        let rss_metadata = Vec::<u8>::new();

        let result = WyvernExchange::atomic_match_ex(
            Origin::signed(sender),
            addrs,
            uints,
            fee_methods_sides_kinds_how_to_calls,
            calldata_buy,
            calldata_sell,
            replacement_pattern_buy,
            replacement_pattern_sell,
            static_extradata_buy,
            static_extradata_sell,
            sig,
            rss_metadata,
        );

        assert_ok!(result);
    });
}
