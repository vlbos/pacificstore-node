// Tests to be written here

use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, dispatch};
mod test_utils;
use self::test_utils::*;
mod asset_white_list;
mod create_order;
mod get_orders;

const TEST_ORDER_ID: &str = "00012345600012";
const TEST_ORGANIZATION: &str = "Northwind";
const TEST_SENDER: &str = "Alice";
const LONG_VALUE : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec aliquam ut tortor nec congue. PellenteLorem ipsum dolor sit amet, consectetur adipiscing elit. Donec aliquam ut tortor nec congue. PellenteLorem ipsum dolor sit amet, consectetur adipiscing elit. Donec aliquam ut tortor nec congue. PellenteLorem ipsum dolor sit amet, consectetur adipiscing elit. Donec aliquam ut tortor nec congue. Pellente";
#[test]
fn get_asset_without_parameters() {
    new_test_ext().execute_with(|| {
        assert_eq!(Orderbook::get_asset(None,None), None);
    })
}

#[test]
fn get_assets_without_parameters() {
    new_test_ext().execute_with(|| {
        assert_eq!(Orderbook::get_assets(None,None), None);
    })
}

