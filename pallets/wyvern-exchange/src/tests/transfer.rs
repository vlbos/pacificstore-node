// Tests to be written here

use super::*;

#[test]
fn transfer_tokens() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let sender1 = account_key(TEST_SENDER_1);

        let amount = 42;
        create_account_test(sender);
        create_account_test(sender1);
        let result = WyvernExchange::transfer_tokens(&sender, &sender, &sender1, amount);

        assert_ok!(result);

        assert_eq!(
            <Test as Trait>::Currency::free_balance(&sender),
            99999999999999958
        );
        assert_eq!(
            <Test as Trait>::Currency::free_balance(&sender1),
            100000000000000042
        );
        // 		assert_eq!(<Test as Config>::Currency::free_balance(&alice()), 100);
        // 		// 10% of the 50 units is unlocked automatically for Alice
        // 		assert_eq!(<Test as Config>::VestingSchedule::vesting_balance(&alice()), Some(45));
        // 		assert_eq!(<Test as Config>::Currency::free_balance(&bob()), 250);
        // 		// A max of 10 units is unlocked automatically for Bob
        // 		assert_eq!(<Test as Config>::VestingSchedule::vesting_balance(&bob()), Some(140));
        // 		// Status is completed.
        // 		assert_eq!(
        // 			Accounts::<Test>::get(alice()),
        // 			AccountStatus {
        // 				validity: AccountValidity::Completed,
        // 				free_balance: 50,
        // 				locked_balance: 50,
        // 				signature: alice_signature().to_vec(),
        // 				vat: Permill::zero(),
        // 			}
        // 		);
    });
}
