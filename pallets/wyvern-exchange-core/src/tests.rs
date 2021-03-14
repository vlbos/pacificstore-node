// Tests to be written here

use super::*;
use crate::{mock::*, Error};
use codec::Encode;
use frame_support::{assert_noop, assert_ok, dispatch};
use sp_core::Pair;
const TEST_SENDER: &str = "Alice";
const TEST_SENDER_1: &str = "Bob";
// mod changes;
// mod orders;
// mod transfer;

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
        registered: time,
    }
}

#[test]
fn change_minimum_maker_protocol_fee() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let new_minimum_maker_protocol_fee = 42;
        let result = WyvernExchangeCore::change_minimum_maker_protocol_fee(
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
        let result = WyvernExchangeCore::change_minimum_taker_protocol_fee(
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
        let result = WyvernExchangeCore::change_protocol_fee_recipient(Origin::signed(sender), sender1);
        assert_ok!(result);
        assert_eq!(<ProtocolFeeRecipient<Test>>::get(), sender1);
    });
}

#[test]
fn hash_order() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);
        create_account_test(sender);
        create_account_test(sender1);
        <ContractSelf<Test>>::put(sender);
        let order = make_order(sender, sender, sender, 0);
        let hash =  WyvernExchangeCore::hash_order(&order).unwrap();
       
        assert_eq!(hash,vec![184, 203, 23, 235, 174, 183, 26, 41, 112, 218, 247, 173, 72, 27, 38, 62, 234, 163, 65, 237, 76, 63, 74, 53, 56, 89, 68, 126, 111, 179, 22, 53]);
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
        let hash =  WyvernExchangeCore::hash_to_sign(
            &order
        ).unwrap();
        let alice_pair = account_pair("Alice");
        let alice_sig = alice_pair.sign(&hash);
        let sig = alice_sig; 
        let result =  WyvernExchangeCore::require_valid_order(
            &order,&sig
        ).unwrap();
       
        // assert_ok!(result.clone());
        assert_eq!(result,vec![18, 231, 144, 31, 116, 56, 16, 202, 12, 253, 180, 169, 181, 224, 230, 51, 181, 200, 104, 251, 103, 137, 115, 3, 173, 51, 160, 222, 108, 37, 148, 52]);

    });
}

#[test]
fn validate_order_parameters() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);
        create_account_test(sender);
        create_account_test(sender1);
        <ContractSelf<Test>>::put(sender);
        let order = make_order(sender, sender, sender, 0);
        let result =  WyvernExchangeCore::validate_order_parameters(&order).unwrap();
       
        assert_eq!(result,true);
    });
}


#[test]
fn validate_order() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);
        create_account_test(sender);
        create_account_test(sender1);
        <ContractSelf<Test>>::put(sender);
        let  order = make_order(sender, sender, sender, 0);
        let hash =  WyvernExchangeCore::hash_to_sign(&order).unwrap();
        let alice_pair = account_pair("Alice");
        let alice_sig = alice_pair.sign(&hash);
        let sig = alice_sig; 
        let result =  WyvernExchangeCore::validate_order(&hash,&order,&sig).unwrap();
        assert_eq!(result,true);
    });
}

#[test]
fn approve_order() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);
        create_account_test(sender);
        create_account_test(sender1);
        let order = make_order(sender, sender, sender, 0);
        let orderbook_inclusion_desired: bool = false;
        let result = WyvernExchangeCore::approve_order(
            Origin::signed(sender),&order,orderbook_inclusion_desired,
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
        let alice_sig = alice_pair.sign(&calldatas);
        let sig = alice_sig; 
        let order = make_order(sender, sender, sender, 0);
        let orderbook_inclusion_desired: bool = false;
        let result = WyvernExchangeCore::approve_order(
            Origin::signed(sender),&order,orderbook_inclusion_desired,
        );
        assert_ok!(result);

        let result = WyvernExchangeCore::cancel_order(
            Origin::signed(sender),&order,&sig,
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
        let order = make_order(sender, sender, sender, 0);
        let alice_pair = account_pair("Alice");
        let hash =  WyvernExchangeCore::hash_to_sign(&order).unwrap();
        let alice_sig = alice_pair.sign(&hash);
        let sig = alice_sig; 
        let result = WyvernExchangeCore::cancel_order(
            Origin::signed(sender),&order,&sig,
        );
        assert_ok!(result);
    });
}

#[test]
fn atomic_match() {
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
        let buy = make_order(sender, sender, sender, 0);
        let sell = make_order(sender1, sender, sender1, 1);
        let hash_buy =  WyvernExchangeCore::hash_to_sign(&buy).unwrap();
        let hash_sell =  WyvernExchangeCore::hash_to_sign(&sell).unwrap();
        let alice_sig_buy = alice_pair.sign(&hash_buy);
        let bob_sig_sell = bob_pair.sign(&hash_sell);
        let rss_metadata = Vec::<u8>::new();
        let amount = 42;
        let result = WyvernExchangeCore::atomic_match(
            sender,amount,buy,alice_sig_buy,sell,bob_sig_sell,&rss_metadata,
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
        let result = WyvernExchangeCore::transfer_tokens(&sender, &sender, &sender1, amount);
        assert_ok!(result);
        assert_eq!(
            <Test as wyvern_exchange_core::exchange_common::Trait>::Currency::free_balance(&sender),
            99999999999999958
        );
        assert_eq!(
            <Test as wyvern_exchange_core::exchange_common::Trait>::Currency::free_balance(&sender1),
            100000000000000042
        );
     });
}
