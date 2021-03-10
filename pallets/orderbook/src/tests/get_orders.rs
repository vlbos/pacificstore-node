// Tests to be written here

use super::*;
const TEST_SENDER: &str = "Alice";
const TEST_ORDER_ID: &str = "00012345600012";

#[test]
fn get_orders_with_valid_parameters() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let order_id = TEST_ORDER_ID.as_bytes().to_owned();
        let owner = sender;
        let now = 42;
        let index = 1;
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
            Orderbook::get_orders(
                Some(OrderQuery {
                    limit: None,
                    offset: None,
                    owner: Some(owner),
                    token_ids: None,
                    params: Some(order_fields.clone())
                }),
                None
            ),
            Some(vec![OrderJSONType {
                index: index,
                order_id: order_id.clone(),
                owner: owner,
                registered: now,
                fields: Some(order_fields),
            }])
        );
    })
}

#[test]
fn get_orders_without_parameters() {
    new_test_ext().execute_with(|| {
        assert_eq!(Orderbook::get_orders(None, None), None);
    })
}

#[test]
fn get_order_with_valid_parameters() {
    new_test_ext().execute_with(|| {
 let sender = account_key(TEST_SENDER);
        let order_id = TEST_ORDER_ID.as_bytes().to_owned();
        let owner = sender;
        let now = 42;
        let index = 1;
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
            Orderbook::get_order(
                Some(OrderQuery {
                    limit: None,
                    offset: None,
                    owner: Some(owner),
                    token_ids: None,
                    params: Some(order_fields.clone())
                })
            ),
            Some(OrderJSONType {
                index: index,
                order_id: order_id.clone(),
                owner: owner,
                registered: now,
                fields: Some(order_fields),
            })
        );
    })
}

#[test]
fn get_order_without_parameters() {
    new_test_ext().execute_with(|| {
        assert_eq!(Orderbook::get_order(None), None);
    })
}
