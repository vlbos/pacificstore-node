// Tests to be written here

use super::*;
const TEST_SENDER: &str = "Alice";

#[test]
fn post_asset_white_list_with_valid_parameters() {
    new_test_ext().execute_with(|| {
        let token_address = b"token_address".to_vec();
        let token_id = b"token id".to_vec();
        let email = b"email".to_vec();
        assert_ok!(Orderbook::post_asset_white_list(
            Origin::signed(account_key(TEST_SENDER)),
            token_address.clone(),
            token_id.clone(),
            email.clone(),
        ));

        // Event is raised
        assert!(System::events().iter().any(|er| er.event
            == TestEvent::orderbook(RawEvent::AssetWhiteListPosted(
                token_address.clone(),
                token_id.clone(),
                email.clone(),
            ))));
    })
}

#[test]
fn remove_asset_white_list_on_chain() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let owner = sender;
        let token_address = b"token_address".to_vec();
        let token_id = b"token id".to_vec();
        let email = b"email".to_vec();
        assert_ok!(Orderbook::post_asset_white_list(
            Origin::signed(account_key(TEST_SENDER)),
            token_address.clone(),
            token_id.clone(),
            email.clone(),
        ));

        let result = Orderbook::change_owner(Origin::signed(sender), owner.clone());

        assert_ok!(result);

        assert_ok!(Orderbook::remove_asset_white_list(
            Origin::signed(account_key(TEST_SENDER)),
            token_address.clone(),
            token_id.clone(),
        ));
    })
}
