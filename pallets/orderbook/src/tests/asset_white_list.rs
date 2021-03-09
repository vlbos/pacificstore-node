// Tests to be written here

use super::*;
const TEST_SENDER: &str = "Alice";

#[test]
fn post_asset_white_list_with_valid_parameters() {
    new_test_ext().execute_with(|| {
        assert_ok!(Orderbook::post_asset_white_list(
            Origin::signed(account_key(TEST_SENDER)),
            b"token_id".to_vec(),
            b"token id val1".to_vec(),
            b"email".to_vec(),
        ));
    })
}
