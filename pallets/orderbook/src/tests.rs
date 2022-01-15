// Tests to be written here

use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, dispatch};
mod test_utils;
use self::test_utils::*;
mod asset_white_list;
mod get_orders;
mod post_order;

const TEST_ORDER_ID: &str = "00012345600012";
const TEST_SENDER: &str = "Alice";
#[test]
fn get_asset_with_valid_parameters() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let order_id = TEST_ORDER_ID.as_bytes().to_owned();
        let owner = sender;
        let now = 42;
        Timestamp::set_timestamp(now);
        let order_fields = get_test_order();

        let result = Orderbook::post_order(
            Origin::signed(sender),
            order_id.clone(),
            owner.clone(),
            Some(order_fields.clone()),
        );

        assert_ok!(result);

        assert_eq!(
            Orderbook::get_asset(
                Some(b"0x16baf0de678e52367adc69fd067e5edd1d33e3bf".to_vec()),
                Some(b"505".to_vec()),
            ),
            Some(JSONType {
                jsons: None,
                fields: Some(order_fields),
            })
        );
    })
}

#[test]
fn get_asset_without_parameters() {
    new_test_ext().execute_with(|| {
        assert_eq!(Orderbook::get_asset(None, None), None);
    })
}

#[test]
fn get_assets_with_valid_parameters() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let order_id = TEST_ORDER_ID.as_bytes().to_owned();
        let owner = sender;
        let now = 42;
        Timestamp::set_timestamp(now);
        let order_fields = get_test_order();

        let result = Orderbook::post_order(
            Origin::signed(sender),
            order_id.clone(),
            owner.clone(),
            Some(order_fields.clone()),
        );

        assert_ok!(result);

        assert_eq!(
            Orderbook::get_assets(
                Some(AssetQuery {
                    limit: None,
                    offset: None,
                    owner: Some(owner),
                    token_ids: Some(vec![b"505".to_vec()]),
                    asset_contract_address: Some(
                        b"0x16baf0de678e52367adc69fd067e5edd1d33e3bf".to_vec()
                    ),
                    search: None,
                    order_by: None,
                    order_direction: None,
                }),
                None
            ),
            Some(vec![JSONType {
                jsons: None,
                fields: Some(order_fields),
            }])
        );
    })
}

#[test]
fn get_assets_without_parameters() {
    new_test_ext().execute_with(|| {
        assert_eq!(Orderbook::get_assets(None, None), None);
    })
}
