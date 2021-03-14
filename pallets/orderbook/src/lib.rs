//! # Orderbook Pallet
//!
//! The Orderbook pallet allows post  and management for NFT Orders .
//! ## Overview
//!
//! The Orderbook pallet provides functionality for Orders management.
//! * Post Order
//! * Get Orders
//!postOrder``postAssetWhitelist`,`getOrder`,`getOrders`,`getAsset`,`getAssets

//! ### Goals
//!
//! The Orderbook  in Substrate is designed to make the following possible:
//!
//! It allows developers to access the official orderbook, filter it,
//! create buy orders (**offers**), create sell orders (**auctions**).
//!
//! ### Dispatchable Functions
//!
//! * `post_order` - Send an order to the orderbook.
//! * `post_asset_white_list`  -  Create a whitelist entry for an asset to prevent others from buying.Buyers will have to have verified at least one of the emails on an asset in order to buy.

//! ### Public Functions
//!
//! * `get_orders` - Get a list of orders from the orderbook, returning the page of orders
//!   and the count of total orders found.
//! * `get_asset` - Fetch an asset from the API, throwing if none is found
//! * `get_assets` - Fetch list of assets from the API, returning the page of assets and the count of total assets
//!

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
    sp_std::{collections::btree_set::BTreeSet,if_std}, sp_std::prelude::*,
};

use frame_system::{self as system, ensure_signed};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod types;
pub use crate::types::*;

mod builders;
use crate::builders::*;


pub trait Trait: system::Trait + timestamp::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Orderbook {
        NextOrderIndex: u64;
        pub Orders get(fn order_by_index): map hasher(blake2_128_concat) u64 => Option<OrderJSONType<T::AccountId, T::Moment>>;
        pub OrderIndices get(fn order_index_by_id): map hasher(blake2_128_concat) OrderId => u64;
        pub OrdersByField get(fn order_index_by_field): double_map hasher(blake2_128_concat) Vec<u8>, hasher(blake2_128_concat) Vec<u8>  => Vec<u64>;
        pub OwnerOf get(fn owner_of): map hasher(blake2_128_concat) OrderId => Option<T::AccountId>;
        pub AssetWhitelist get(fn asset_white_list): double_map hasher(blake2_128_concat) Vec<u8>, hasher(blake2_128_concat) Vec<u8>  => Vec<u8>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        OrderPosted(AccountId, OrderId, AccountId),
        AssetWhiteListPosted(Vec<u8>, Vec<u8>, Vec<u8>),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        OrderIdMissing,
        OrderIdTooLong,
        OrderIdExists,
        OrderTooManyFields,
        OrderInvalidFieldName,
        OrderInvalidFieldValue
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        /// Send an order to the orderbook.
        /// param order Order JSON to post to the orderbook
        #[weight = 10_000]
        pub fn post_order(
            origin,
            order_id: OrderId,
            owner: T::AccountId,
            fields: Option<Vec<OrderField>>,
        ) -> DispatchResult {
            // T::CreateRoleOrigin::ensure_origin(origin.clone())?;
            let who = ensure_signed(origin)?;
            // Validate order ID
            Self::validate_order_id(&order_id)?;

            // Validate order fields
            Self::validate_order_fields(&fields)?;

            // Check order doesn't exist yet
            Self::validate_new_order(&order_id)?;

            // Generate next order ID
            let next_index = NextOrderIndex::get()
                .checked_add(1)
                .expect("order id error");

            NextOrderIndex::put(next_index);

            if let Some(fields) = &fields {
                for field in fields {
                    let mut index_arr: Vec<u64> = Vec::new();

                    if <OrdersByField>::contains_key(field.name(), field.value()) {
                        index_arr = <OrdersByField>::get(field.name(), field.value());
                        if !index_arr.contains(&next_index) {
                            index_arr.push(next_index);
                            <OrdersByField>::mutate(field.name(), field.value(), |arr| *arr = index_arr);
                        }
                    } else {
                        index_arr.push(next_index);
                        <OrdersByField>::insert(field.name(), field.value(), index_arr);
                    }
               }
            }

            // Create a order instance
            let order = Self::new_order()
                .index_by(next_index)
                .identified_by(order_id.clone())
                .owned_by(owner.clone())
                .registered_on(<timestamp::Module<T>>::now())
                .with_fields(fields)
                .build();
            if !<Orders<T>>::contains_key(next_index.clone()) {
                <Orders<T>>::insert(next_index, order);
            }
            if !<OrderIndices>::contains_key(order_id.clone()) {
                <OrderIndices>::insert(&order_id, next_index);
            }
            // <OrdersByField<T>>::append(&owner, &order_id);
            if !<OwnerOf<T>>::contains_key(order_id.clone()) {
                <OwnerOf<T>>::insert(&order_id, &owner);
            }

            Self::deposit_event(RawEvent::OrderPosted(who, order_id, owner));

            Ok(())
        }

        /// Create a whitelist entry for an asset to prevent others from buying.
        /// Buyers will have to have verified at least one of the emails
        /// on an asset in order to buy.
        /// This will return error code if the given API key isn't allowed to create whitelist entries for this contract or asset.
        /// tokenAddress Address of the asset's contract
        /// tokenId The asset's token ID
        /// email The email allowed to buy.
        #[weight = 10_000]
        pub fn post_asset_white_list(
            origin,
            token_address: Vec<u8>,
            token_id: Vec<u8>,
            email: Vec<u8>,
        ) -> DispatchResult {
            if <AssetWhitelist>::contains_key(token_address.clone(), token_id.clone()) {
                <AssetWhitelist>::mutate(token_address.clone(), token_id.clone(), |_email| *_email = email.clone());
            } else {
                <AssetWhitelist>::insert(token_address.clone(), token_id.clone(), email.clone());
            }
            Self::deposit_event(RawEvent::AssetWhiteListPosted(token_address, token_id, email));
            Ok(())
        }

    }
}

impl<T: Trait> Module<T> {
    /// Helper methods
    fn new_order() -> OrderBuilder<T::AccountId, T::Moment> {
        OrderBuilder::<T::AccountId, T::Moment>::default()
    }

    pub fn validate_order_id(order_id: &[u8]) -> DispatchResult {
        // Basic order ID validation
        ensure!(!order_id.is_empty(), Error::<T>::OrderIdMissing);
        ensure!(
            order_id.len() <= ORDER_ID_MAX_LENGTH,
            Error::<T>::OrderIdTooLong
        );
        Ok(())
    }

    pub fn validate_new_order(order_id: &[u8]) -> DispatchResult {
        // Order existence check
        ensure!(
            !<OrderIndices>::contains_key(order_id),
            Error::<T>::OrderIdExists
        );
        Ok(())
    }

    pub fn validate_order_fields(fields: &Option<Vec<OrderField>>) -> DispatchResult {
        if let Some(fields) = fields {
            ensure!(
                fields.len() <= ORDER_MAX_FIELDS,
                Error::<T>::OrderTooManyFields,
            );
            for field in fields {
                ensure!(
                    field.name().len() <= ORDER_FIELD_NAME_MAX_LENGTH,
                    Error::<T>::OrderInvalidFieldName
                );
                ensure!(
                    field.value().len() <= ORDER_FIELD_VALUE_MAX_LENGTH,
                    Error::<T>::OrderInvalidFieldValue
                );
            }
        }
        Ok(())
    }

    /// Get an order from the orderbook, throwing if none is found.
    /// query Query to use for getting orders. A subset of parameters
    /// on the `OrderJSON` type is supported
    pub fn get_order(
        order_query: Option<OrderQuery<T::AccountId>>,
    ) -> Option<OrderJSONType<T::AccountId, T::Moment>> {
        if let Some(orders) = Self::get_orders(order_query, Some(1)) {
            if !orders.is_empty() {
                if let Some(order) = orders.get(0) {
                    return Some((*order).clone());
                }
            }
        }
        None
    }

    pub fn order_intersection(order_indices: &mut BTreeSet<u64>, indexes: Vec<u64>) {
        if !order_indices.is_empty() {
            let o = indexes.into_iter().collect::<BTreeSet<_>>();
            let orders: Vec<u64> = order_indices.intersection(&o).cloned().collect();
            *order_indices = orders.into_iter().collect::<BTreeSet<_>>();
        } else {
            *order_indices = indexes.into_iter().collect::<BTreeSet<_>>();
        }
    }

    pub fn get_order_by_token_ids(
        token_ids: Option<Vec<TokenId>>,
        order_indices: &mut BTreeSet<u64>,
    ) -> Option<()> {
        let field_name: Vec<u8> = b"metadata.asset.token_id".to_vec();
        let mut order_indices_by_token_ids = Vec::<u64>::new();
        if let Some(token_ids) = &token_ids {
            if token_ids.len() > MAX_TOKEN_IDS {
                if_std! {
                println!("token_ids' length is greater than ORDER_MAX_FIELDS ");
                        }
                return None;
            }
            for token_id in token_ids {
                if <OrdersByField>::contains_key(field_name.clone(), token_id.to_vec()) {
                    let mut order_indexes =
                        <OrdersByField>::get(field_name.clone(), token_id.to_vec());
                    if !order_indexes.is_empty() {
                        order_indices_by_token_ids.append(&mut order_indexes);
                    }
                }
            }
        }
        Self::order_intersection(order_indices, order_indices_by_token_ids);
        if order_indices.is_empty() {
            return None;
        }

        Some(())
    }

    pub fn get_order_by_params(
        params: Option<Vec<OrderField>>,
        order_indices: &mut BTreeSet<u64>,
    ) -> Option<()> {
        if let Some(params) = &params {
            if params.len() > ORDER_MAX_PARAMS {
                if_std! {
                println!("params' length is greater than ORDER_MAX_FIELDS ");
                        }
                return None;
            }
            for field in params {
                if <OrdersByField>::contains_key(field.name(), field.value()) {
                    Self::order_intersection(
                        order_indices,
                        <OrdersByField>::get(field.name(), field.value()),
                    );
                    if order_indices.is_empty() {
                        if_std! {
                        println!("order_indices is empty");
                                }
                        return None;
                    } 
                } else {
                    if_std! {
                    println!("OrdersByField doesn't contain {:#?}{:#?}",field.name(), field.value());
                            }
                }
            }

            if order_indices.is_empty() {
                if_std! {
                println!("order_indices is empty in get_order_by_params");
                        }
                return None;
            } 
        }
        Some(())
    }
    pub fn convert_option_to_size(value: Option<u64>, default_value: usize) -> usize {
        if let Some(value) = value {
            return value as usize;
        }
        default_value
    }

    pub fn get_orders_by_indices(
        temp_order_indices: BTreeSet<u64>,
        limit: usize,
        offset: usize,
    ) -> Option<Vec<OrderJSONType<T::AccountId, T::Moment>>> {
        if temp_order_indices.is_empty() {
            if_std! {
            println!("temp_order_indices is empty in get_orders_by_indices ");
                    }
            return None;
        }
        let mut result_orders: Vec<OrderJSONType<T::AccountId, T::Moment>> = Vec::new();
        let result_order_indices: Vec<u64> = temp_order_indices.into_iter().collect::<Vec<_>>();
        if result_order_indices.len() <= offset {
            if_std! {
            println!("result_order_indices'length is less than offset");
                    }
            return None;
        }
        let end = if result_order_indices.len() <= offset + limit {
            result_order_indices.len()
        } else {
            offset + limit
        };

        for i in offset..end {
            let index = i as usize;

            if <Orders<T>>::contains_key(result_order_indices[index]) {
                let o = <Orders<T>>::get(result_order_indices[index]);
                if let Some(o) = o {
                    result_orders.push(o);
                }
            }
        }

        Some(result_orders)
    }
    /// Get a list of orders from the orderbook, returning the page of orders
    /// and the count of total orders found.
    /// param query Query to use for getting orders. A subset of parameters
    /// on the `OrderJSON` type is supported
    /// param page Page number, defaults to 1. Can be overridden by
    /// `limit` and `offset` attributes from OrderQuery
    pub fn get_orders(
        order_query: Option<OrderQuery<T::AccountId>>,
        page: Option<u64>,
    ) -> Option<Vec<OrderJSONType<T::AccountId, T::Moment>>> {
        let mut _page = 1;
        if let Some(page) = page {
            _page = page
        }

        let mut temp_order_indices: BTreeSet<u64> = BTreeSet::new();
        if let Some(order_query) = order_query {
            if let None = Self::get_order_by_params(order_query.params, &mut temp_order_indices) {
                if_std! {
                    println!("get_order_by_params is empty in get_orders");
                }
                return None;
            }

            if temp_order_indices.is_empty() {
                if_std! {
                    println!("temp_order_indices is empty");
                }
                return None;
            }

            if let Some(token_ids) = order_query.token_ids {
                if !token_ids.is_empty() {
                    if let None =
                        Self::get_order_by_token_ids(Some(token_ids), &mut temp_order_indices)
                    {
                        if_std! {
                            println!("get_order_by_token_ids return empty in get_orders");
                        }
                        return None;
                    }
                }
            }

            let limit: usize = Self::convert_option_to_size(order_query.limit, 8);
            let offset: usize = Self::convert_option_to_size(order_query.offset, 0);
            if let Some(result_orders) =
                Self::get_orders_by_indices(temp_order_indices, limit, offset)
            {
                return Some(result_orders);
            }
        }

        None
    }

    /// Fetch an asset from the API, throwing if none is found
    /// tokenAddress Address of the asset's contract
    /// tokenId The asset's token ID, or null if ERC-20
    /// retries Number of times to retry if the service is unavailable for any reason
    pub fn get_asset(
        token_address: Option<Vec<u8>>,
        token_id: Option<Vec<u8>>,
    ) -> Option<JSONType> {
        let mut token_ids: Option<Vec<TokenId>> = None;
        if let Some(token_id) = token_id {
            token_ids = Some(vec![token_id]);
        }
        let query = AssetQuery::<T::AccountId> {
            owner: None,
            asset_contract_address: token_address,
            token_ids: token_ids,
            search: None,
            order_by: None,
            order_direction: None,
            limit: Some(8),
            offset: Some(0),
        };
        let page = 1;
        if let Some(jsons) = Self::get_assets(Some(query), Some(page)) {
            if !jsons.is_empty() {
                if let Some(json) = jsons.get(0) {
                    return Some((*json).clone());
                }
            }
        }
        None
    }

    /// Fetch list of assets from the API, returning the page of assets and the count of total assets
    /// query Query to use for getting orders. A subset of parameters on the `OpenSeaAssetJSON` type is supported
    /// page Page number, defaults to 1. Can be overridden by
    /// `limit` and `offset` attributes from OpenSeaAssetQuery
    pub fn get_assets(
        asset_query: Option<AssetQuery<T::AccountId>>,
        page: Option<u64>,
    ) -> Option<Vec<JSONType>> {
        let order_query = convert_assetquery_to_orderquery(asset_query);

        if let Some(orders) = Self::get_orders(order_query, page) {
            if !orders.is_empty() {
                let mut jsons: Vec<JSONType> = Vec::<JSONType>::with_capacity(orders.len());
                for order in orders {
                    jsons.push(convert_orderjsontype_to_jsontype(order));
                }
                return Some(jsons);
            }
        }

        None
    }
}
