// Tests to be written here

// use super::*;

// #[test]
// fn create_order_with_invalid_field_value() {
//     new_test_ext().execute_with(|| {
//         assert_noop!(
//             Orderbook::post_order(
//                 Origin::signed(account_key(TEST_SENDER)),
//                 TEST_ORDER_ID.as_bytes().to_owned(),
//                 account_key(TEST_ORGANIZATION),
//                 Some(vec![
//                     OrderField::new(b"field1", b"val1"),
//                     OrderField::new(b"field2", b"val2"),
//                     OrderField::new(b"field3", &LONG_VALUE.as_bytes().to_owned()),
//                 ])
//             ),
//             Error::<Test>::OrderInvalidFieldValue
//         );
//     })
// }

// decl_module! {for enum Call where origin: T::Origin
// pub struct Module<T: Trait>  {
// type Error = Error<T>;
// fn deposit_event() = default;

//     Send an order to the orderbook.
//     param order Order JSON to post to the orderbook
// #[weight = 10_000]
// pub fn post_order(
//     origin: T::Origin,
//     order_id: OrderId,
//     owner: T::AccountId,
//     fields: Option<Vec<OrderField>>,
// ) -> DispatchResult {
//     // T::CreateRoleOrigin::ensure_origin(origin.clone())?;
//     let who = ensure_signed(origin)?;
//     // Validate order ID
//     Self::validate_order_id(&order_id)?;

//     // Validate order fields
//     Self::validate_order_fields(&fields)?;

//     // Check order doesn't exist yet (1 DB read)
//     Self::validate_new_order(&order_id)?;

//     // TODO: if organization has an attribute w/ GS1 Company prefix,
//     //       additional validation could be applied to the order ID
//     //       to ensure its validity (same company prefix as org).

//     // Generate next collection ID
//     let next_id = NextOrderIndex::get()
//         .checked_add(1)
//         .expect("order id error");

//     NextOrderIndex::put(next_id);

//     if let Some(fields) = &fields {
//         for field in fields {
//             let mut index_arr: Vec<u64> = Vec::new();

//             if <OrdersByField>::contains_key(field.name(), field.value()) {
//                 index_arr = <OrdersByField>::get(field.name(), field.value());
//                 if !index_arr.contains(&next_id) {
//                     index_arr.push(next_id);
//                     <OrdersByField>::mutate(field.name(), field.value(), |arr| *arr = index_arr);
//                 }
//             // ensure!(!index_arr.contains(&next_id), "Account already has admin role");
//             } else {
//                 index_arr.push(next_id);
//                 <OrdersByField>::insert(field.name(), field.value(), index_arr);
//             }

//             //   <OrdersByField<T>>::append(&field, &next_id);
//         }
//     }

//     // Create a order instance
//     let mut order = Self::new_order()
//         .identified_by(order_id.clone())
//         .owned_by(owner.clone())
//         .registered_on(<timestamp::Module<T>>::now())
//         .with_fields(fields)
//         .build();
//     order.index = next_id;
//     // Add order & ownerOf (3 DB writes)
//     if !<Orders<T>>::contains_key(next_id.clone()) {
//         <Orders<T>>::insert(next_id, order);
//     }
//     if !<Orderi>::contains_key(order_id.clone()) {
//         <Orderi>::insert(&order_id, next_id);
//     }
//     // <OrdersByField<T>>::append(&owner, &order_id);
//     if !<OwnerOf<T>>::contains_key(order_id.clone()) {
//         <OwnerOf<T>>::insert(&order_id, &owner);
//     }

//     Self::deposit_event(RawEvent::OrderPosted(who, order_id, owner));

//     Ok(())
// }

// //
// //   Create a whitelist entry for an asset to prevent others from buying.
// //   Buyers will have to have verified at least one of the emails
// //   on an asset in order to buy.
// //   This will return error code if the given API key isn't allowed to create whitelist entries for this contract or asset.
// //    tokenAddress Address of the asset's contract
// //    tokenId The asset's token ID
// //    email The email allowed to buy.
// //
// //     postAssetWhitelist(tokenAddress: string, tokenId: string | number, email: string): Promise<boolean>;
// pub fn post_asset_white_list(
//     origin: u64,
//     token_address: Vec<u8>,
//     token_id: Vec<u8>,
//     email: Vec<u8>,
// ) -> DispatchResult {
//     if <AssetWhitelist>::contains_key(token_address, token_id) {
//         <AssetWhitelist>::mutate(token_address, token_id, |_email| *_email = email);
//     } else {
//         <AssetWhitelist>::insert(token_address, token_id, email);
//     }
//     Ok(())
// }

// }
// }
