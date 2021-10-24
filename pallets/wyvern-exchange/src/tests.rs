// Tests to be written here

use super::*;
use crate::{mock::*};
use codec::Encode;
use frame_support::{assert_ok};
use sp_core::Pair;
const TEST_SENDER: &str = "Alice";
const TEST_SENDER_1: &str = "Bob";

type AccountId = <Test as system::Config>::AccountId;
type Moment = <Test as timestamp::Config>::Moment;
type Balance = <Test as balances::Trait>::Balance;
fn make_order(
    maker: AccountId,
    taker: AccountId,
    fee_recipient: AccountId,
    side: u8,
) -> OrderType<AccountId, Moment, Balance> {
    let sender = account_key(TEST_SENDER);
    let fee: u64 = 0;
    let bytes = vec![];
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
        target: AccountId::default(),
        how_to_call: HowToCall::from(0),
        calldata: bytes.clone(),
        replacement_pattern: bytes.clone(),
        static_target: AccountId::default(),
        static_extradata: bytes.clone(),
        payment_token: AccountId::default(),
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
                147, 233, 158, 149, 155, 212, 67, 206, 192, 117, 59, 117, 31, 121, 168, 212, 124,
                91, 122, 155, 102, 96, 113, 59, 169, 68, 43, 127, 136, 240, 214, 169
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
                144, 223, 135, 224, 228, 211, 197, 17, 23, 67, 219, 239, 162, 220, 227, 194, 8,
                77, 64, 59, 252, 167, 84, 189, 45, 125, 52, 218, 160, 131, 2, 14
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
        let order = make_order(sender, sender, sender, 0);
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

        // Event is raised
        assert!(System::events().iter().any(|er| er.event
            == TestEvent::exchange_core(RawEvent::OrderApprovedPartOne(
                hash.clone(),
                order.exchange.clone(),
                order.maker.clone(),
                order.taker.clone(),
                order.maker_relayer_fee,
                order.taker_relayer_fee,
                order.maker_protocol_fee,
                order.taker_protocol_fee,
                order.fee_recipient.clone(),
                order.fee_method.clone(),
                order.side.clone(),
                order.sale_kind.clone(),
                order.target.clone(),
        ))));
    
        assert!(System::events().iter().any(|er| er.event
            == TestEvent::exchange_core(RawEvent::OrderApprovedPartTwo(
                hash.clone(),
                order.how_to_call.clone(),
                order.calldata.clone(),
                order.replacement_pattern.clone(),
                order.static_target.clone(),
                order.static_extradata.clone(),
                order.payment_token.clone(),
                order.base_price,
                order.extra,
                order.listing_time,
                order.expiration_time,
                order.salt,
                orderbook_inclusion_desired,
        ))));
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

       // Event is raised
        assert!(System::events().iter().any(|er| er.event
            == TestEvent::exchange_core(RawEvent::OrderCancelled(hash.clone()))));
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
        let buy = make_order(sender, sender1, sender, 0);
        let sell = make_order(sender1, sender, AccountId::default(), 1);
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
        ) = make_order_ex(sender, sender1, sender, 0);
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
        ) = make_order_ex(sender1, sender, AccountId::default(), 1);

        let buy_hash = WyvernExchange::hash_to_sign_ex(
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

        let sell_hash = WyvernExchange::hash_to_sign_ex(
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

        let alice_sig_buy = (<[u8; 64]>::from(alice_pair.sign(&buy_hash))).to_vec();
        let bob_sig_sell = (<[u8; 64]>::from(bob_pair.sign(&sell_hash))).to_vec();

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
        let rss_metadata = vec![];
        let price = WyvernExchange::calculate_match_price_ex(
            addrs.clone(),
            uints.clone(),
            fee_methods_sides_kinds_how_to_calls.clone(),
            calldata_buy.clone(),
            calldata_sell.clone(),
            replacement_pattern_buy.clone(),
            replacement_pattern_sell.clone(),
            static_extradata_buy.clone(),
            static_extradata_sell.clone(),
        );

        let result = WyvernExchange::atomic_match_ex(
            Origin::signed(sender),
            addrs.clone(),
            uints.clone(),
            fee_methods_sides_kinds_how_to_calls.clone(),
            calldata_buy.clone(),
            calldata_sell.clone(),
            replacement_pattern_buy.clone(),
            replacement_pattern_sell.clone(),
            static_extradata_buy.clone(),
            static_extradata_sell.clone(),
            alice_sig_buy.clone(),
            bob_sig_sell.clone(),
            rss_metadata.clone(),
        );
        assert_ok!(result);
        
        // Event is raised
        assert!(System::events().iter().any(|er| er.event
            == TestEvent::exchange_core(RawEvent::OrdersMatched(
                vec![],//buy_hash.clone(),
                sell_hash.clone(),
                if sell.fee_recipient != AccountId::default() {
                    sell.maker.clone()
                } else {
                    buy.maker.clone()
                },
                if sell.fee_recipient != AccountId::default() {
                    buy.maker.clone()
                } else {
                    sell.maker.clone()
                },
                price,
                rss_metadata.clone(),
        ))));
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
