// Tests to be written here

use super::*;
use crate::{mock::*, Error};
use codec::Encode;
use frame_support::{assert_noop, assert_ok};
use sp_core::Pair;
const TEST_SENDER: &str = "Alice";
const TEST_SENDER_1: &str = "Bob";
// mod changes;
// mod orders;
// mod transfer;

type AccountId = <Test as system::Config>::AccountId;
type Moment = <Test as timestamp::Config>::Moment;
type Balance = <Test as balances::Config>::Balance;
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
		maker,
		taker,
		maker_relayer_fee: fee,
		taker_relayer_fee: fee,
		maker_protocol_fee: fee,
		taker_protocol_fee: fee,
		fee_recipient,
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

#[test]
fn change_minimum_maker_protocol_fee() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_SENDER);
		let new_minimum_maker_protocol_fee = 42;
		let result = WyvernExchangeCore::change_owner(Origin::signed(sender), sender);
		assert_ok!(result);
		let result = WyvernExchangeCore::change_minimum_maker_protocol_fee(
			Origin::signed(sender),
			new_minimum_maker_protocol_fee,
		);
		assert_ok!(result);
		assert_eq!(<MinimumMakerProtocolFee<Test>>::get(), new_minimum_maker_protocol_fee);

		// Event is raised
		assert!(System::events().iter().any(|er| er.event ==
			TestEvent::wyvern_exchange_core(RawEvent::MinimumMakerProtocolFeeChanged(
				new_minimum_maker_protocol_fee,
			))));
	});
}

#[test]
fn change_minimum_taker_protocol_fee() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_SENDER);
		let min_taker_protocol_fee = 42;
		let result = WyvernExchangeCore::change_owner(Origin::signed(sender), sender);
		assert_ok!(result);
		let result = WyvernExchangeCore::change_minimum_taker_protocol_fee(
			Origin::signed(sender),
			min_taker_protocol_fee,
		);
		assert_ok!(result);
		assert_eq!(<MinimumTakerProtocolFee<Test>>::get(), min_taker_protocol_fee);

		// Event is raised
		assert!(System::events().iter().any(|er| er.event ==
			TestEvent::wyvern_exchange_core(RawEvent::MinimumTakerProtocolFeeChanged(
				min_taker_protocol_fee,
			))));
	});
}

#[test]
fn change_protocol_fee_recipient() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_SENDER);
		let sender1 = account_key(TEST_SENDER_1);
		let result = WyvernExchangeCore::change_owner(Origin::signed(sender), sender);
		assert_ok!(result);
		let result =
			WyvernExchangeCore::change_protocol_fee_recipient(Origin::signed(sender), sender1);
		assert_ok!(result);
		assert_eq!(<ProtocolFeeRecipient<Test>>::get(), sender1);
		// Event is raised
		assert!(System::events().iter().any(|er| er.event ==
			TestEvent::wyvern_exchange_core(RawEvent::ProtocolFeeRecipientChanged(
				sender, sender1,
			))));
	});
}

#[test]
fn change_protocol_fee_recipient_with_no_owner() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_SENDER);
		let sender1 = account_key(TEST_SENDER_1);
		assert_noop!(
			WyvernExchangeCore::change_protocol_fee_recipient(Origin::signed(sender1), sender),
			Error::<Test>::OnlyOwner,
		);
	});
}

#[test]
fn change_owner() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_SENDER);
		let sender1 = account_key(TEST_SENDER_1);
		let result = WyvernExchangeCore::change_owner(Origin::signed(sender), sender1);
		assert_ok!(result);
		assert_eq!(<Owner<Test>>::get(), sender1);
		// Event is raised
		assert!(System::events().iter().any(|er| er.event ==
			TestEvent::wyvern_exchange_core(RawEvent::OwnerChanged(sender, sender1,))));
	});
}

#[test]
fn set_contract_self() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_SENDER);
		let sender1 = account_key(TEST_SENDER_1);
		let result = WyvernExchangeCore::set_contract_self(Origin::signed(sender), sender1);
		assert_ok!(result);
		assert_eq!(<ContractSelf<Test>>::get(), sender1);
		// Event is raised
		assert!(System::events().iter().any(|er| er.event ==
			TestEvent::wyvern_exchange_core(RawEvent::ContractSelfChanged(sender, sender1,))));
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
		let hash = WyvernExchangeCore::hash_order(&order).unwrap();

		assert_eq!(
			hash,
			vec![
				107, 177, 222, 136, 236, 39, 224, 125, 86, 231, 153, 29, 58, 206, 67, 184, 37, 27,
				93, 126, 117, 244, 8, 182, 157, 180, 136, 15, 64, 140, 90, 33
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
		let hash = WyvernExchangeCore::hash_to_sign(&order).unwrap();
		let alice_pair = account_pair("Alice");
		let alice_sig = <[u8; 64]>::from(alice_pair.sign(&hash));
		let sig = alice_sig;
		let result = WyvernExchangeCore::require_valid_order(&order, &sig).unwrap();

		// assert_ok!(result.clone());
		assert_eq!(
			result,
			vec![
				36, 44, 71, 101, 200, 18, 243, 245, 214, 239, 142, 44, 51, 131, 143, 154, 248, 13,
				117, 34, 151, 20, 255, 242, 47, 171, 182, 241, 209, 227, 159, 112
			]
		);
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
		let result = WyvernExchangeCore::validate_order_parameters(&order);

		assert_eq!(result, true);
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
		let order = make_order(sender, sender, sender, 0);
		let hash = WyvernExchangeCore::hash_to_sign(&order).unwrap();
		let alice_pair = account_pair("Alice");
		let alice_sig = <[u8; 64]>::from(alice_pair.sign(&hash));
		let sig = alice_sig;
		let result = WyvernExchangeCore::validate_order(&hash, &order, &sig).unwrap();
		assert_eq!(result, true);
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
			Origin::signed(sender),
			&order,
			orderbook_inclusion_desired,
		);
		assert_ok!(result);
	});
}

#[test]
fn cancel_order_with_approved_order() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_SENDER);
		let sender1 = account_key(TEST_SENDER_1);
		create_account_test(sender);
		create_account_test(sender1);
		<ContractSelf<Test>>::put(sender);
		let alice_pair = account_pair("Alice");
		let calldatas = "calldata.to_vec()".encode();
		let alice_sig = <[u8; 64]>::from(alice_pair.sign(&calldatas));
		let sig = alice_sig;
		let order = make_order(sender, sender, sender, 0);
		let orderbook_inclusion_desired: bool = false;
		let result = WyvernExchangeCore::approve_order(
			Origin::signed(sender),
			&order,
			orderbook_inclusion_desired,
		);
		assert_ok!(result);

		let result = WyvernExchangeCore::cancel_order(Origin::signed(sender), &order, &sig);
		assert_ok!(result);
	});
}

#[test]
fn cancel_order_with_signature() {
	new_test_ext().execute_with(|| {
		let sender = account_key(TEST_SENDER);
		let sender1 = account_key(TEST_SENDER_1);
		create_account_test(sender);
		create_account_test(sender1);
		<ContractSelf<Test>>::put(sender);
		let order = make_order(sender, sender, sender, 0);
		let alice_pair = account_pair("Alice");
		let hash = WyvernExchangeCore::hash_to_sign(&order).unwrap();
		let alice_sig = <[u8; 64]>::from(alice_pair.sign(&hash));
		let sig = alice_sig;
		let result = WyvernExchangeCore::cancel_order(Origin::signed(sender), &order, &sig);
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
		let buy = make_order(sender, sender1, sender, 0);
		let sell = make_order(sender1, sender, AccountId::default(), 1);
		let hash_buy = WyvernExchangeCore::hash_to_sign(&buy).unwrap();
		let hash_sell = WyvernExchangeCore::hash_to_sign(&sell).unwrap();
		let alice_sig_buy = <[u8; 64]>::from(alice_pair.sign(&hash_buy));
		let bob_sig_sell = <[u8; 64]>::from(bob_pair.sign(&hash_sell));
		let rss_metadata = Vec::<u8>::new();
		let amount = 42;
		let result = WyvernExchangeCore::atomic_match(
			sender,
			amount,
			buy,
			(&alice_sig_buy).to_vec(),
			sell,
			(&bob_sig_sell).to_vec(),
			&rss_metadata,
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
			<Test as wyvern_exchange_core::exchange_common::Config>::Currency::free_balance(&sender),
			99999999999999958
		);
		assert_eq!(
			<Test as wyvern_exchange_core::exchange_common::Config>::Currency::free_balance(
				&sender1,
			),
			100000000000000042
		);
	});
}
