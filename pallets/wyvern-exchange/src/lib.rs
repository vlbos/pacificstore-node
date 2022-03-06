//! # WyvernExchange Pallet
//!
//!
//! ## Overview
//!
//! This is an outer pallet with public or convenience functions and includes no state-modifying
//! functions. The WyvernExchange pallet provides functionality for WyvernExchanges management.
//!
//! * Approve Order
//! * Cancel Order
//! * Hash Order
//! * Validate Order
//! * AtomicMatch Order
//!
//! ### Goals
//!
//! The WyvernExchange system in Substrate is designed to make the following possible:
//!
//! *Autonomously governed decentralized digital asset exchange.
//!
//! ### Dispatchable Functions
//!
//! * `approve_order_ex` - Approve an order and optionally mark it for orderbook inclusion. Must be
//!   called by the maker of the order
//! * `cancel_order_ex` - Cancel an order, preventing it from being matched. Must be called by the
//!   maker of the order
//! * `atomic_match_ex` -Atomically match two orders, ensuring validity of the match, and execute
//!   all associated state transitions. Protected against reentrancy by a contract-global lock.
//!
//! ### Public Functions
//!
//! * `hash_order_ex` - Hash an order, returning the canonical order hash, without the message
//!   prefix
//! * `hash_to_sign_ex` - Hash an order, returning the hash that a client must sign.
//! * `require_valid_order_ex` - Assert an order is valid and return its hash order OrderType to
//!   validate sig  signature.
//! * `validate_order_ex` - Validate a provided previously approved / signed order, hash, and
//!   signature.
//! * `validate_order_parameters_ex` - Validate order parameters (doesnot check validity-signature)
//! * `calculate_current_price_ex` - Calculate the current price of an order (fn -convenience)
//! * `calculate_match_price_ex` - Calculate the price two orders would match at, if in fact they
//!   would match (fail -otherwise).
//! * `orders_can_match_ex` - Return whether or not two orders can be matched with each other by
//!   basic parameters (does not check order signatures / calldata or perform calls -static).
//! * `order_calldata_can_match_ex` - Return whether or not two orders' calldata specifications can match.
//! * `calculate_final_price_ex` - Calculate the settlement price of an order;   Precondition:
//!   parameters have passed validate_parameters.

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "512"]
// use core::result::Result;
pub use pallet::*;

// use frame_support::{
//     decl_module, decl_storage,
//     dispatch::DispatchResult,
//     sp_runtime::{
//         traits::{Zero},
//     },
//     sp_std::prelude::*,
// };

// use frame_system::{ ensure_signed};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use exchange_core;

// mod types;

// pub mod exchange_common;
pub use exchange_core::{exchange_common, exchange_common::BalanceOf, sale_kind_interface, Error};
// pub mod sale_kind_interface;
// pub mod exchange_core;
// pub use crate::exchange_core::Event;
#[frame_support::pallet]
pub mod pallet {
	pub use exchange_core::{
		exchange_common, exchange_common::BalanceOf, sale_kind_interface, types::*, Error,
	};
	use frame_support::{
		dispatch::DispatchResult, pallet_prelude::*, sp_runtime::traits::Zero, sp_std::prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use pallet_contracts::chain_extension::UncheckedFrom;

	#[pallet::config]
	pub trait Config: frame_system::Config + exchange_core::Config {}
	// decl_storage! {
	//     trait Store for Pallet<T: Trait> as WyvernExchange {

	//  }
	// }
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T>	where
		T::AccountId: UncheckedFrom<T::Hash>,
		T::AccountId: AsRef<[u8]>, {
		// decl_module! {
		//     pub struct Pallet<T: Trait> for enum Call where origin: T::Origin {
		// type Error = Error<T>;
		// fn deposit_event() = default;
		// Call approve_order - .
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn approve_order_ex(
			origin: OriginFor<T>,
			addrs: Vec<T::AccountId>,
			uints: Vec<u64>,
			fee_method: FeeMethod,
			side: Side,
			sale_kind: SaleKind,
			how_to_call: HowToCall,
			calldata: Vec<u8>,
			replacement_pattern: Vec<u8>,
			static_extradata: Vec<u8>,
			orderbook_inclusion_desired: bool,
		) -> DispatchResult {
			let _user = ensure_signed(origin.clone())?;
			let order: OrderType<T::AccountId, T::Moment, BalanceOf<T>> =
				<exchange_common::Pallet<T>>::build_order_type_from_array_parameters(
					addrs,
					uints,
					fee_method,
					side,
					sale_kind,
					how_to_call,
					&calldata,
					&replacement_pattern,
					&static_extradata,
				);
			<exchange_core::Pallet<T>>::approve_order(origin, &order, orderbook_inclusion_desired)
		}

		// Call cancel_order - .
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn cancel_order_ex(
			origin: OriginFor<T>,
			addrs: Vec<T::AccountId>,
			uints: Vec<u64>,
			fee_method: FeeMethod,
			side: Side,
			sale_kind: SaleKind,
			how_to_call: HowToCall,
			calldata: Vec<u8>,
			replacement_pattern: Vec<u8>,
			static_extradata: Vec<u8>,
			sig: Vec<u8>,
		) -> DispatchResult {
			let _user = ensure_signed(origin.clone())?;
			<exchange_core::Pallet<T>>::cancel_order(
				origin,
				&<exchange_common::Pallet<T>>::build_order_type_from_array_parameters(
					addrs,
					uints,
					fee_method,
					side,
					sale_kind,
					how_to_call,
					&calldata,
					&replacement_pattern,
					&static_extradata,
				),
				&sig,
			)
		}

		// Call atomic_match - .
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn atomic_match_ex(
			origin: OriginFor<T>,
			addrs: Vec<T::AccountId>,
			uints: Vec<u64>,
			fee_methods_sides_kinds_how_to_calls: Vec<u8>,
			calldata_buy: Vec<u8>,
			calldata_sell: Vec<u8>,
			replacement_pattern_buy: Vec<u8>,
			replacement_pattern_sell: Vec<u8>,
			static_extradata_buy: Vec<u8>,
			static_extradata_sell: Vec<u8>,
			sig_buy: Vec<u8>,
			sig_sell: Vec<u8>,
			rss_metadata: Vec<u8>,
		) -> DispatchResult {
			let _user = ensure_signed(origin)?;
			let buy_sell_orders = <exchange_common::Pallet<T>>::build_buy_sell_order_type(
				addrs,
				uints,
				&fee_methods_sides_kinds_how_to_calls,
				&calldata_buy,
				&calldata_sell,
				&replacement_pattern_buy,
				&replacement_pattern_sell,
				&static_extradata_buy,
				&static_extradata_sell,
			);
			if let Err(err) = <exchange_core::Pallet<T>>::atomic_match(
				_user,
				Zero::zero(),
				buy_sell_orders[0].clone(),
				sig_buy.clone(),
				buy_sell_orders[1].clone(),
				sig_sell.clone(),
				&rss_metadata,
			) {
				frame_support::log::error!("==atomic_match_ex==debug::error============={:?}", err);
				return Err(err)
			}
			Ok(())
		}

		//  }
	}

	impl<T: Config> Pallet<T> where
		T::AccountId: UncheckedFrom<T::Hash>,
		T::AccountId: AsRef<[u8]>,{
		//  Call calculate_final_price - library exposed for testing.
		pub fn calculate_final_price_ex(
			side: Side,
			sale_kind: SaleKind,
			base_price: u64,
			extra: T::Moment,
			listing_time: T::Moment,
			expiration_time: T::Moment,
		) -> u64 {
			let mut base_pricex: BalanceOf<T> = Zero::zero();
			if let Some(base_price) =
				<exchange_common::Pallet<T>>::u64_to_balance_option(base_price)
			{
				base_pricex = base_price;
			}

			let _price = <sale_kind_interface::Pallet<T>>::calculate_final_price(
				&side,
				&sale_kind,
				base_pricex,
				extra,
				listing_time,
				expiration_time,
			);

			if let Some(price) = <exchange_common::Pallet<T>>::balance_to_u64_option(_price) {
				return price
			}

			0
		}

		// Call hash_order - .
		pub fn hash_order_ex(
			addrs: Vec<T::AccountId>,
			uints: Vec<u64>,
			fee_method: FeeMethod,
			side: Side,
			sale_kind: SaleKind,
			how_to_call: HowToCall,
			calldata: Vec<u8>,
			replacement_pattern: Vec<u8>,
			static_extradata: Vec<u8>,
		) -> Vec<u8> {
			<exchange_core::Pallet<T>>::hash_order(
				&<exchange_common::Pallet<T>>::build_order_type_from_array_parameters(
					addrs,
					uints,
					fee_method,
					side,
					sale_kind,
					how_to_call,
					&calldata,
					&replacement_pattern,
					&static_extradata,
				),
			)
			.unwrap()
		}

		// Call hash_to_sign - .
		pub fn hash_to_sign_ex(
			addrs: Vec<T::AccountId>,
			uints: Vec<u64>,
			fee_method: FeeMethod,
			side: Side,
			sale_kind: SaleKind,
			how_to_call: HowToCall,
			calldata: Vec<u8>,
			replacement_pattern: Vec<u8>,
			static_extradata: Vec<u8>,
		) -> Vec<u8> {
			<exchange_core::Pallet<T>>::hash_to_sign(
				&<exchange_common::Pallet<T>>::build_order_type_from_array_parameters(
					addrs,
					uints,
					fee_method,
					side,
					sale_kind,
					how_to_call,
					&calldata,
					&replacement_pattern,
					&static_extradata,
				),
			)
			.unwrap()
		}

		// Call validate_order_parameters - .
		pub fn validate_order_parameters_ex(
			addrs: Vec<T::AccountId>,
			uints: Vec<u64>,
			fee_method: FeeMethod,
			side: Side,
			sale_kind: SaleKind,
			how_to_call: HowToCall,
			calldata: Vec<u8>,
			replacement_pattern: Vec<u8>,
			static_extradata: Vec<u8>,
		) -> bool {
			let order: OrderType<T::AccountId, T::Moment, BalanceOf<T>> =
				<exchange_common::Pallet<T>>::build_order_type_from_array_parameters(
					addrs,
					uints,
					fee_method,
					side,
					sale_kind,
					how_to_call,
					&calldata,
					&replacement_pattern,
					&static_extradata,
				);
			<exchange_core::Pallet<T>>::validate_order_parameters(&order)
		}

		// Call validate_order - .
		pub fn validate_order_ex(
			addrs: Vec<T::AccountId>,
			uints: Vec<u64>,
			fee_method: FeeMethod,
			side: Side,
			sale_kind: SaleKind,
			how_to_call: HowToCall,
			calldata: Vec<u8>,
			replacement_pattern: Vec<u8>,
			static_extradata: Vec<u8>,
			sig: Vec<u8>,
		) -> bool {
			let order: OrderType<T::AccountId, T::Moment, BalanceOf<T>> =
				<exchange_common::Pallet<T>>::build_order_type_from_array_parameters(
					addrs,
					uints,
					fee_method,
					side,
					sale_kind,
					how_to_call,
					&calldata,
					&replacement_pattern,
					&static_extradata,
				);
			<exchange_core::Pallet<T>>::validate_order(
				&<exchange_core::Pallet<T>>::hash_to_sign(&order).unwrap(),
				&order,
				&sig,
			)
			.unwrap()
		}

		// Call require valid order - .
		pub fn require_valid_order_ex(
			addrs: Vec<T::AccountId>,
			uints: Vec<u64>,
			fee_method: FeeMethod,
			side: Side,
			sale_kind: SaleKind,
			how_to_call: HowToCall,
			calldata: Vec<u8>,
			replacement_pattern: Vec<u8>,
			static_extradata: Vec<u8>,
			sig: Vec<u8>,
		) -> Vec<u8> {
			let order: OrderType<T::AccountId, T::Moment, BalanceOf<T>> =
				<exchange_common::Pallet<T>>::build_order_type_from_array_parameters(
					addrs,
					uints,
					fee_method,
					side,
					sale_kind,
					how_to_call,
					&calldata,
					&replacement_pattern,
					&static_extradata,
				);
			<exchange_core::Pallet<T>>::require_valid_order(&order, &sig).unwrap()
		}

		// Call calculate_current_price - .
		pub fn calculate_current_price_ex(
			addrs: Vec<T::AccountId>,
			uints: Vec<u64>,
			fee_method: FeeMethod,
			side: Side,
			sale_kind: SaleKind,
			how_to_call: HowToCall,
			calldata: Vec<u8>,
			replacement_pattern: Vec<u8>,
			static_extradata: Vec<u8>,
		) -> u64 {
			let _price = <exchange_core::Pallet<T>>::calculate_current_price(
				&<exchange_common::Pallet<T>>::build_order_type_from_array_parameters(
					addrs,
					uints,
					fee_method,
					side,
					sale_kind,
					how_to_call,
					&calldata,
					&replacement_pattern,
					&static_extradata,
				),
			)
			.unwrap();

			if let Some(_balance) = <exchange_common::Pallet<T>>::balance_to_u64_option(_price) {
				return _balance
			}

			0
		}

		// Call orders_can_match - .
		pub fn orders_can_match_ex(
			addrs: Vec<T::AccountId>,
			uints: Vec<u64>,
			fee_methods_sides_kinds_how_to_calls: Vec<u8>,
			calldata_buy: Vec<u8>,
			calldata_sell: Vec<u8>,
			replacement_pattern_buy: Vec<u8>,
			replacement_pattern_sell: Vec<u8>,
			static_extradata_buy: Vec<u8>,
			static_extradata_sell: Vec<u8>,
		) -> bool {
			let buy_sell_orders = <exchange_common::Pallet<T>>::build_buy_sell_order_type(
				addrs,
				uints,
				&fee_methods_sides_kinds_how_to_calls,
				&calldata_buy,
				&calldata_sell,
				&replacement_pattern_buy,
				&replacement_pattern_sell,
				&static_extradata_buy,
				&static_extradata_sell,
			);
			<exchange_core::Pallet<T>>::orders_can_match(&buy_sell_orders[0], &buy_sell_orders[1])
		}

		// Return whether or not two orders' calldata specifications can match
		// buy_calldata Buy-side order calldata
		// buy_replacement_pattern Buy-side order calldata replacement mask
		// sell_calldata Sell-side order calldata
		// sell_replacement_pattern Sell-side order calldata replacement mask
		// Whether the orders' calldata can be matched
		pub fn order_calldata_can_match_ex(
			buy_calldata: Vec<u8>,
			buy_replacement_pattern: Vec<u8>,
			sell_calldata: Vec<u8>,
			sell_replacement_pattern: Vec<u8>,
		) -> bool {
			let mut tmpbuy_calldata = buy_calldata.clone();
			let mut tmpsell_calldata = sell_calldata.clone();
			if buy_replacement_pattern.len() > 0 {
				if !<exchange_common::Pallet<T>>::guarded_array_replace(
					&mut tmpbuy_calldata,
					&sell_calldata,
					&buy_replacement_pattern,
				) {
					return false;
				}
			}
			if sell_replacement_pattern.len() > 0 {
				if !<exchange_common::Pallet<T>>::guarded_array_replace(
					&mut tmpsell_calldata,
					&buy_calldata,
					&sell_replacement_pattern,
				) {
					return false;
				}
			}

			<exchange_common::Pallet<T>>::array_eq(&tmpbuy_calldata, &tmpsell_calldata)
		}

		// Call calculate_match_price - .
		pub fn calculate_match_price_ex(
			addrs: Vec<T::AccountId>,
			uints: Vec<u64>,
			fee_methods_sides_kinds_how_to_calls: Vec<u8>,
			calldata_buy: Vec<u8>,
			calldata_sell: Vec<u8>,
			replacement_pattern_buy: Vec<u8>,
			replacement_pattern_sell: Vec<u8>,
			static_extradata_buy: Vec<u8>,
			static_extradata_sell: Vec<u8>,
		) -> u64 {
			let buy_sell_orders = <exchange_common::Pallet<T>>::build_buy_sell_order_type(
				addrs,
				uints,
				&fee_methods_sides_kinds_how_to_calls,
				&calldata_buy,
				&calldata_sell,
				&replacement_pattern_buy,
				&replacement_pattern_sell,
				&static_extradata_buy,
				&static_extradata_sell,
			);
			let _price = <exchange_core::Pallet<T>>::calculate_match_price(
				&buy_sell_orders[0],
				&buy_sell_orders[1],
			)
			.unwrap();

			if let Some(_balance) = <exchange_common::Pallet<T>>::balance_to_u64_option(_price) {
				return _balance
			}

			0
		}
	}
}
