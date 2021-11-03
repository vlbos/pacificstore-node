// Tests to be written here

use super::*;

const TEST_ORDER_ID: &str = "00012345600012";
const TEST_SENDER: &str = "Alice";
const LONG_VALUE : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
Donec aliquam ut tortor nec congue. PellenteLorem ipsum dolor sit amet, consectetur \
adipiscing elit. Donec aliquam ut tortor nec congue. PellenteLorem ipsum dolor sit amet, \
consectetur adipiscing elit. Donec aliquam ut tortor nec congue. PellenteLorem ipsum \
dolor sit amet, consectetur adipiscing elit. Donec aliquam ut tortor nec congue. Pellente";

#[test]
fn post_order_without_fields() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let order_id = TEST_ORDER_ID.as_bytes().to_owned();
        let owner = sender;
        let now = 42;
        let index = 1;
        Timestamp::set_timestamp(now);

        let result = Orderbook::post_order(
            Origin::signed(sender),
            order_id.clone(),
            owner.clone(),
            None,
        );

        assert_ok!(result);

        assert_eq!(
            Orderbook::order_by_index(index),
            Some(OrderJSONType {
                index: 1,
                order_id: order_id.clone(),
                owner: owner,
                created_date: now,
                fields: None
            })
        );

        // Event is raised
        assert!(System::events().iter().any(|er| er.event
            == TestEvent::orderbook(RawEvent::OrderPosted(sender, order_id.clone(), owner))));
    });
}

#[test]
fn post_order_with_valid_fields() {
    new_test_ext().execute_with(|| {
        let fields = get_test_order();
        let sender = account_key(TEST_SENDER);
        let order_id = TEST_ORDER_ID.as_bytes().to_owned();
        let owner = sender;
        let now = 42;
        let index = 1;
        Timestamp::set_timestamp(now);

        let result = Orderbook::post_order(
            Origin::signed(sender),
            order_id.clone(),
            owner.clone(),
            Some(fields.clone()),
        );

        assert_ok!(result);

        assert_eq!(
            Orderbook::order_by_index(index),
            Some(OrderJSONType {
                index: index,
                order_id: order_id.clone(),
                owner: owner,
                created_date: now,
                fields: Some(fields.clone()),
            })
        );

        // Event is raised
        assert!(System::events().iter().any(|er| er.event
            == TestEvent::orderbook(RawEvent::OrderPosted(sender, order_id.clone(), owner))));
    });
}

#[test]
fn post_order_with_invalid_sender() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Orderbook::post_order(Origin::none(), vec!(), account_key(TEST_SENDER), None),
            dispatch::DispatchError::BadOrigin
        );
    });
}

#[test]
fn post_order_with_missing_id() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Orderbook::post_order(
                Origin::signed(account_key(TEST_SENDER)),
                vec!(),
                account_key(TEST_SENDER),
                None
            ),
            Error::<Test>::OrderIdMissing
        );
    });
}

#[test]
fn post_order_with_long_id() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Orderbook::post_order(
                Origin::signed(account_key(TEST_SENDER)),
                LONG_VALUE.as_bytes().to_owned(),
                account_key(TEST_SENDER),
                None
            ),
            Error::<Test>::OrderIdTooLong
        );
    })
}

#[test]
fn post_order_with_existing_id() {
    new_test_ext().execute_with(|| {
        let existing_order = TEST_ORDER_ID.as_bytes().to_owned();

        store_test_order_index::<Test>(1, existing_order.clone());

        assert_noop!(
            Orderbook::post_order(
                Origin::signed(account_key(TEST_SENDER)),
                existing_order,
                account_key(TEST_SENDER),
                None
            ),
            Error::<Test>::OrderIdExists
        );
    })
}

#[test]
fn post_order_with_too_many_fields() {
    new_test_ext().execute_with(|| {
        let mut s = Vec::with_capacity(60);
        for _ in 1..60 {
            s.push(OrderField::new(b"field1", b"val1"));
        }
        assert_noop!(
            Orderbook::post_order(
                Origin::signed(account_key(TEST_SENDER)),
                TEST_ORDER_ID.as_bytes().to_owned(),
                account_key(TEST_SENDER),
                Some(s.to_vec())
            ),
            Error::<Test>::OrderTooManyFields
        );
    })
}

#[test]
fn post_order_with_invalid_field_name() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Orderbook::post_order(
                Origin::signed(account_key(TEST_SENDER)),
                TEST_ORDER_ID.as_bytes().to_owned(),
                account_key(TEST_SENDER),
                Some(vec![
                    OrderField::new(b"field1", b"val1"),
                    OrderField::new(b"field2", b"val2"),
                    OrderField::new(&LONG_VALUE.as_bytes().to_owned(), b"val3"),
                ])
            ),
            Error::<Test>::OrderInvalidFieldName
        );
    })
}

#[test]
fn post_order_with_invalid_field_value() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Orderbook::post_order(
                Origin::signed(account_key(TEST_SENDER)),
                TEST_ORDER_ID.as_bytes().to_owned(),
                account_key(TEST_SENDER),
                Some(vec![
                    OrderField::new(b"field1", b"val1"),
                    OrderField::new(b"field2", b"val2"),
                    OrderField::new(b"field3", &LONG_VALUE.as_bytes().to_owned()),
                ])
            ),
            Error::<Test>::OrderInvalidFieldValue
        );
    })
}



#[test]
fn post_order_with_exceed_limits() {
    new_test_ext().execute_with(|| {
        let test_order_ids: Vec<&str> = vec![
            "00012345600013",
            "00012345600014",
            "00012345600015",
            "00012345600016",
        ];
        let sender = account_key(TEST_SENDER);
        let order_id = TEST_ORDER_ID.as_bytes().to_owned();
        let owner = sender;
        let now = 42;
        Timestamp::set_timestamp(now);

        let orders = get_test_orders();

        for i in 0..orders.len()-1 {
            if let Some(fields) = orders.get(i){
                if let Some(order_id) = test_order_ids.get(i){
                    let result = Orderbook::post_order(
                        Origin::signed(sender),
                        order_id.as_bytes().to_owned().clone(),
                        owner.clone(),
                        Some(fields.clone()),
                    );

                    assert_ok!(result);
                }
            }
        }

        if let Some(fields) = orders.get(3){
            assert_noop!(
                Orderbook::post_order(
                    Origin::signed(sender),
                    order_id.clone(),
                    owner.clone(),
                    Some(fields.clone()),
                ),
                Error::<Test>::OrderLimitsExceed,
            );
        }
     });
}


#[test]
fn set_order_limits() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let owner = sender;
        let result = Orderbook::change_owner(
            Origin::signed(sender),
            owner.clone(),
        );

        assert_ok!(result);
        let limits = 1000;
        let result = Orderbook::set_order_limits(
            Origin::signed(sender),
            limits,
        );

        assert_ok!(result);

        assert_eq!(
            <OrderLimits>::get(),
            limits,
        );

        // Event is raised
        assert!(System::events().iter().any(|er| er.event
            == TestEvent::orderbook(RawEvent::OrderLimitsChanged(limits))));
 
    });
}


#[test]
fn set_asset_white_list_limits() {
    new_test_ext().execute_with(|| {
        let sender = account_key(TEST_SENDER);
        let owner = sender;
        let result = Orderbook::change_owner(
            Origin::signed(sender),
            owner.clone(),
        );

        assert_ok!(result);
        let limits = 2000;
        let result = Orderbook::set_asset_white_list_limits(
            Origin::signed(sender),
            limits,
        );

        assert_ok!(result);

        assert_eq!(
            <AssetWhiteListLimits>::get(),
            limits,
        );

        // Event is raised
        assert!(System::events().iter().any(|er| er.event
            == TestEvent::orderbook(RawEvent::AssetWhiteListLimitsChanged(limits))));
 
    });
}

#[test]
fn remove_order_on_chain() {
    new_test_ext().execute_with(|| {
        let fields = get_test_order();
        let sender = account_key(TEST_SENDER);
        let order_id = TEST_ORDER_ID.as_bytes().to_owned();
        let owner = sender;
        let now = 42;
        let index = 1;
        Timestamp::set_timestamp(now);
        let result = Orderbook::change_owner(
            Origin::signed(sender),
            owner.clone(),
        );

        assert_ok!(result);
        let result = Orderbook::post_order(
            Origin::signed(sender),
            order_id.clone(),
            owner.clone(),
            Some(fields.clone()),
        );

        assert_ok!(result);

        assert_eq!(
            Orderbook::order_by_index(index),
            Some(OrderJSONType {
                index: index,
                order_id: order_id.clone(),
                owner: owner,
                created_date: now,
                fields: Some(fields.clone()),
            })
        );

        let result = Orderbook::remove_order(
            Origin::signed(sender),
            index,
         );

        assert_ok!(result);
    });
}