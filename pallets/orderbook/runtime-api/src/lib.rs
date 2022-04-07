#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::Codec;
use orderbook::{AssetQuery, JSONType, OrderJSONType, OrderQuery};
use sp_std::vec::Vec;

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait OrderbookApi<AccountId,  Moment> where
		AccountId: Codec,
		Moment: Codec,
		{
			fn get_order(
				order_query: Option<OrderQuery<AccountId>>,
			) -> Option<OrderJSONType<AccountId, Moment>>;
			fn get_orders(
				order_query: Option<OrderQuery<AccountId>>, page: Option<u64>,
			) -> Option<Vec<OrderJSONType<AccountId, Moment>>>;
			fn get_asset(
				token_address: Option<Vec<u8>>,token_id: Option<Vec<u8>>,
			) -> Option<JSONType>;
			fn get_assets(
				asset_query: Option<AssetQuery<AccountId>>, page: Option<u64>,
			) -> Option<Vec<JSONType>>;
		}
}
