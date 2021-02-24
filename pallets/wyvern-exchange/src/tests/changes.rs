// Tests to be written here

use super::*;

#[test]
fn change_minimum_maker_protocol_fee() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let new_minimum_maker_protocol_fee = 42;

        let result = WyvernExchange::change_minimum_maker_protocol_fee(
            Origin::signed(sender),
            new_minimum_maker_protocol_fee,
        );

        assert_ok!(result);

        assert_eq!(
            <MinimumMakerProtocolFee<Test>>::get(),
            new_minimum_maker_protocol_fee
        );
    });
}

#[test]
fn change_minimum_taker_protocol_fee() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let min_taker_protocol_fee = 42;

        let result = WyvernExchange::change_minimum_taker_protocol_fee(
            Origin::signed(sender),
            min_taker_protocol_fee,
        );

        assert_ok!(result);

        assert_eq!(
            <MinimumTakerProtocolFee<Test>>::get(),
            min_taker_protocol_fee
        );
    });
}

#[test]
fn change_protocol_fee_recipient() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);

        let result = WyvernExchange::change_protocol_fee_recipient(Origin::signed(sender), sender1);

        assert_ok!(result);

        assert_eq!(<ProtocolFeeRecipient<Test>>::get(), sender1);
    });
}

// [order.exchange, order.maker, order.taker, order.feerecipient, order.target, order.statictarget, order.paymenttoken],
//               [order.makerRelayerFee, order.takerRelayerFee, order.makerProtocolFee, order.takerProtocolFee, order.basePrice, order.extra, order.listingTime, order.expirationTime, order.salt],
//               order.feeMethod,
//               order.side,
//               order.saleKind,
//               order.howToCall,
//               order.calldata,
//               order.replacementPattern,
//               order.staticExtradata,
//               true
//             ).then(res => {
//
//                 return exchangeInstance.cancelOrder_(
//                   [order.exchange, order.maker, order.taker, order.feerecipient, order.target, order.statictarget, order.paymenttoken],
//                   [order.makerRelayerFee, order.takerRelayerFee, order.makerProtocolFee, order.takerProtocolFee, order.basePrice, order.extra, order.listingTime, order.expirationTime, order.salt],
//                   order.feeMethod,
//                   order.side,
//                   order.saleKind,
//                   order.howToCall,
//                   order.calldata,
//                   order.replacementPattern,
//                   order.staticExtradata,
//                   0, '0x', '0x'
