// Tests to be written here

use super::*;
use crate::{mock::*, Error};
use codec::Encode;
use frame_support::{assert_noop, assert_ok, dispatch};
use sp_core::Pair;
const TEST_SENDER: &str = "Alice";
const TEST_SENDER_1: &str = "Bob";

type AccountId = <Test as system::Trait>::AccountId;
type Moment = <Test as timestamp::Trait>::Moment;
type Balance = <Test as balances::Trait>::Balance;
fn make_order(
    maker: AccountId,
    taker: AccountId,
    fee_recipient: AccountId,
    side: u8,
) -> OrderType<AccountId, Moment, Balance> {
    let sender = account_key(TEST_SENDER);
    let fee: u64 = 0;
    let bytes = vec![0x0];
    let time = Moment::default();
    OrderType::<AccountId, Moment, Balance> {
        index: 0,
        exchange: sender,
        maker: maker,
        taker: taker,
        maker_relayer_fee: fee,
        taker_relayer_fee: fee,
        maker_protocol_fee: fee,
        taker_protocol_fee: fee,
        fee_recipient: fee_recipient,
        fee_method: FeeMethod::from(0),
        side: Side::from(side),
        sale_kind: SaleKind::from(0),
        target: sender,
        how_to_call: HowToCall::from(0),
        calldata: bytes.clone(),
        replacement_pattern: bytes.clone(),
        static_target: sender,
        static_extradata: bytes.clone(),
        payment_token: sender,
        base_price: fee,
        extra: time,
        listing_time: Zero::zero(),
        expiration_time: Zero::zero(),
        salt: 0,
        created_date: time,
    }
}

fn make_order_ex(
    maker: AccountId,
    taker: AccountId,
    fee_recipient: AccountId,
    side: u8,
) -> (
    Vec<AccountId>,
    Vec<u64>,
    FeeMethod,
    Side,
    SaleKind,
    HowToCall,
    Vec<u8>,
    Vec<u8>,
    Vec<u8>,
) {
    let order = make_order(maker, taker, fee_recipient, side);
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
    let side = order.side;
    let sale_kind = SaleKind::from(0);
    let how_to_call = HowToCall::from(0);
    let calldata = Vec::<u8>::new();
    let replacement_pattern = Vec::<u8>::new();
    let static_extradata = Vec::<u8>::new();
    (
        addrs,
        uints,
        fee_method,
        side,
        sale_kind,
        how_to_call,
        calldata,
        replacement_pattern,
        static_extradata,
    )
}

#[test]
fn change_minimum_maker_protocol_fee() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let new_minimum_maker_protocol_fee = 42;
        let result = ExchangeCore::change_minimum_maker_protocol_fee(
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
        let result = ExchangeCore::change_minimum_taker_protocol_fee(
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
        let result = ExchangeCore::change_protocol_fee_recipient(Origin::signed(sender), sender1);
        assert_ok!(result);
        assert_eq!(<ProtocolFeeRecipient<Test>>::get(), sender1);
    });
}

#[test]
fn hash_order_ex() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);
        create_account_test(sender);
        create_account_test(sender1);
        <ContractSelf<Test>>::put(sender);
        let (
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            calldata,
            replacement_pattern,
            static_extradata,
        ) = make_order_ex(sender, sender, sender, 0);
        let hash = WyvernExchange::hash_order_ex(
            addrs.clone(),
            uints.clone(),
            fee_method.clone(),
            side.clone(),
            sale_kind.clone(),
            how_to_call.clone(),
            calldata.clone(),
            replacement_pattern.clone(),
            static_extradata.clone(),
        );

        assert_eq!(
            hash,
            vec![
                123, 133, 145, 87, 234, 200, 253, 138, 44, 140, 16, 13, 202, 91, 13, 171, 241, 253,
                240, 155, 153, 69, 181, 204, 128, 12, 220, 94, 16, 237, 78, 190
            ]
        );
    });
}

#[test]
fn require_valid_order() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);
        create_account_test(sender);
        create_account_test(sender1);
        <ContractSelf<Test>>::put(sender);
        let order = make_order(sender, sender, sender, 0);
        let hash = ExchangeCore::hash_to_sign(&order).unwrap();
        let alice_pair = account_pair("Alice");
        let alice_sig = <[u8; 64]>::from(alice_pair.sign(&hash));
        let sig = alice_sig;
        let result = ExchangeCore::require_valid_order(&order, &sig);

        assert_ok!(result);
    });
}

#[test]
fn require_valid_order_ex() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);
        create_account_test(sender);
        create_account_test(sender1);
        <ContractSelf<Test>>::put(sender);
        let (
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            calldata,
            replacement_pattern,
            static_extradata,
        ) = make_order_ex(sender, sender, sender, 0);
        let hash = WyvernExchange::hash_to_sign_ex(
            addrs.clone(),
            uints.clone(),
            fee_method.clone(),
            side.clone(),
            sale_kind.clone(),
            how_to_call.clone(),
            calldata.clone(),
            replacement_pattern.clone(),
            static_extradata.clone(),
        );
        let alice_pair = account_pair("Alice");
        let alice_sig = (<[u8; 64]>::from(alice_pair.sign(&hash))).to_vec();
        let sig = alice_sig;
        let hash = WyvernExchange::require_valid_order_ex(
            addrs.clone(),
            uints.clone(),
            fee_method.clone(),
            side.clone(),
            sale_kind.clone(),
            how_to_call.clone(),
            calldata.clone(),
            replacement_pattern.clone(),
            static_extradata.clone(),
            sig,
        );

        assert_eq!(
            hash,
            vec![
                37, 49, 117, 31, 84, 85, 213, 82, 131, 89, 165, 235, 73, 255, 49, 61, 233, 44, 133,
                116, 14, 159, 125, 27, 157, 50, 252, 154, 134, 82, 90, 216
            ]
        );
    });
}

#[test]
fn validate_order_parameters_ex() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);
        create_account_test(sender);
        create_account_test(sender1);
        <ContractSelf<Test>>::put(sender);
        let (
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            calldata,
            replacement_pattern,
            static_extradata,
        ) = make_order_ex(sender, sender, sender, 0);
        let result = WyvernExchange::validate_order_parameters_ex(
            addrs.clone(),
            uints.clone(),
            fee_method.clone(),
            side.clone(),
            sale_kind.clone(),
            how_to_call.clone(),
            calldata.clone(),
            replacement_pattern.clone(),
            static_extradata.clone(),
        );

        assert_eq!(result, true);
    });
}

#[test]
fn validate_order_ex() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);
        create_account_test(sender);
        create_account_test(sender1);
        <ContractSelf<Test>>::put(sender);
        let (
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            calldata,
            replacement_pattern,
            static_extradata,
        ) = make_order_ex(sender, sender, sender, 0);
        let hash = WyvernExchange::hash_to_sign_ex(
            addrs.clone(),
            uints.clone(),
            fee_method.clone(),
            side.clone(),
            sale_kind.clone(),
            how_to_call.clone(),
            calldata.clone(),
            replacement_pattern.clone(),
            static_extradata.clone(),
        );
        let alice_pair = account_pair("Alice");
        let alice_sig = (<[u8; 64]>::from(alice_pair.sign(&hash))).to_vec();
        let sig = alice_sig;
        let result = WyvernExchange::validate_order_ex(
            addrs.clone(),
            uints.clone(),
            fee_method.clone(),
            side.clone(),
            sale_kind.clone(),
            how_to_call.clone(),
            calldata.clone(),
            replacement_pattern.clone(),
            static_extradata.clone(),
            sig,
        );

        assert_eq!(result, true);
    });
}

#[test]
fn approve_order_ex() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);
        create_account_test(sender);
        create_account_test(sender1);
        let (
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            calldata,
            replacement_pattern,
            static_extradata,
        ) = make_order_ex(sender, sender, sender, 0);
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
fn cancel_order_ex_with_approved_order() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);
        create_account_test(sender);
        create_account_test(sender1);
        <ContractSelf<Test>>::put(sender);
        let alice_pair = account_pair("Alice");
        let calldatas = "calldata.to_vec()".encode();
        let alice_sig = (<[u8; 64]>::from(alice_pair.sign(&calldatas))).to_vec();
        let sig = alice_sig;
        let (
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            calldata,
            replacement_pattern,
            static_extradata,
        ) = make_order_ex(sender, sender, sender, 0);
        let orderbook_inclusion_desired: bool = false;
        let result = WyvernExchange::approve_order_ex(
            Origin::signed(sender),
            addrs.clone(),
            uints.clone(),
            fee_method.clone(),
            side.clone(),
            sale_kind.clone(),
            how_to_call.clone(),
            calldata.clone(),
            replacement_pattern.clone(),
            static_extradata.clone(),
            orderbook_inclusion_desired,
        );
        assert_ok!(result);

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
fn cancel_order_ex_with_signature() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);
        create_account_test(sender);
        create_account_test(sender1);
        <ContractSelf<Test>>::put(sender);
        let (
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            calldata,
            replacement_pattern,
            static_extradata,
        ) = make_order_ex(sender, sender, sender, 0);
        let alice_pair = account_pair("Alice");
        let hash = WyvernExchange::hash_to_sign_ex(
            addrs.clone(),
            uints.clone(),
            fee_method.clone(),
            side.clone(),
            sale_kind.clone(),
            how_to_call.clone(),
            calldata.clone(),
            replacement_pattern.clone(),
            static_extradata.clone(),
        );
        let alice_sig = (<[u8; 64]>::from(alice_pair.sign(&hash))).to_vec();
        let sig = alice_sig;

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

        let alice_pair = account_pair("Alice");
        let bob_pair = account_pair("Bob");

        let (
            addrs_buy,
            uints_buy,
            fee_method_buy,
            side_buy,
            sale_kind_buy,
            how_to_call_buy,
            calldata_buy,
            replacement_pattern_buy,
            static_extradata_buy,
        ) = make_order_ex(sender, sender, sender, 0);
        let (
            addrs_sell,
            uints_sell,
            fee_method_sell,
            side_sell,
            sale_kind_sell,
            how_to_call_sell,
            calldata_sell,
            replacement_pattern_sell,
            static_extradata_sell,
        ) = make_order_ex(sender1, sender, sender1, 1);

        let hash_buy = WyvernExchange::hash_to_sign_ex(
            addrs_buy.clone(),
            uints_buy.clone(),
            fee_method_buy.clone(),
            side_buy.clone(),
            sale_kind_buy.clone(),
            how_to_call_buy.clone(),
            calldata_buy.clone(),
            replacement_pattern_buy.clone(),
            static_extradata_buy.clone(),
        );

        let hash_sell = WyvernExchange::hash_to_sign_ex(
            addrs_sell.clone(),
            uints_sell.clone(),
            fee_method_sell.clone(),
            side_sell.clone(),
            sale_kind_sell.clone(),
            how_to_call_sell.clone(),
            calldata_sell.clone(),
            replacement_pattern_sell.clone(),
            static_extradata_sell.clone(),
        );

        let alice_sig_buy = (<[u8; 64]>::from(alice_pair.sign(&hash_buy))).to_vec();
        let bob_sig_sell = (<[u8; 64]>::from(bob_pair.sign(&hash_sell))).to_vec();

        // let sig = vec![alice_sig_buy, bob_sig_sell];

        let mut addrs = addrs_buy;
        let mut addrs_sell = addrs_sell;
        addrs.append(&mut addrs_sell);
        let mut uints = uints_buy;
        let mut uints_sell = uints_sell;
        uints.append(&mut uints_sell);
        let fee_methods_sides_kinds_how_to_calls_buy: Vec<u8> = vec![
            fee_method_buy.value(),
            side_buy.value(),
            sale_kind_buy.value(),
            how_to_call_buy.value(),
        ]
        .to_vec();
        let mut fee_methods_sides_kinds_how_to_calls_sell: Vec<u8> = vec![
            fee_method_sell.value(),
            side_sell.value(),
            sale_kind_sell.value(),
            how_to_call_sell.value(),
        ]
        .to_vec();

        let mut fee_methods_sides_kinds_how_to_calls = fee_methods_sides_kinds_how_to_calls_buy;
        fee_methods_sides_kinds_how_to_calls.append(&mut fee_methods_sides_kinds_how_to_calls_sell);
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
            alice_sig_buy,
            bob_sig_sell,
            rss_metadata,
        );
        assert_ok!(result);
    });
}

#[test]
fn transfer_tokens() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);
        let amount = 42;
        create_account_test(sender);
        create_account_test(sender1);
        let result = ExchangeCore::transfer_tokens(&sender, &sender, &sender1, amount);
        assert_ok!(result);
        assert_eq!(
            <Test as wyvern_exchange::exchange_common::Trait>::Currency::free_balance(&sender),
            99999999999999958
        );
        assert_eq!(
            <Test as wyvern_exchange::exchange_common::Trait>::Currency::free_balance(&sender1),
            100000000000000042
        );
    });
}
