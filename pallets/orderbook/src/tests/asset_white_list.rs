// Tests to be written here

use super::*;
const TEST_SENDER: &str = "Alice";

//  owner?: string;
//     sale_kind?: SaleKind;
//     asset_contract_address?: string;
//     payment_token_address?: string;
//     is_english?: boolean;
//     is_expired?: boolean;
//     bundled?: boolean;
//     include_invalid?: boolean;
//     token_id?: number | string;
//     token_ids?: Array<number | string>;
//     listed_after?: number | string;
//     listed_before?: number | string;
//     limit?: number;
//     offset?: number;

#[test]
fn post_asset_white_list_with_valid_parameters() {
    new_test_ext().execute_with(|| {
        assert_ok!(Orderbook::post_asset_white_list(
            Origin::signed(account_key(TEST_SENDER)),
            b"field1".to_vec(),
            b"val1".to_vec(),
            b"field2".to_vec(),
        ));
    })
}


