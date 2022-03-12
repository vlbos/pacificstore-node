//! # Pacific Store - Wyvern Exchange pallet
//! Abstract over fixed-price sales and Dutch auctions, with the intent
//! of easily supporting additional methods of sale later.

#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::sp_runtime::traits::Zero;

	use crate::{exchange_common, exchange_common::*, types::*};

	#[pallet::config]
	pub trait Config: frame_system::Config + timestamp::Config {}
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);
	impl<T: Config + exchange_common::pallet::Config> Pallet<T> {
		// sale Kind interface
		// Check whether the parameters of a sale are valid
		// sale_kind Kind of sale
		// expiration_time OrderType expiration time
		//Whether the parameters were valid
		pub fn validate_parameters(sale_kind: &SaleKind, expiration_time: T::Moment) -> bool {
			// Auctions must have a set expiration date.
			*sale_kind == SaleKind::FixedPrice || expiration_time > Zero::zero()
		}

		// Return whether or not an order can be settled
		// Precondition: parameters have passed validate_parameters
		// listing_time OrderType listing time
		// expiration_time OrderType expiration time
		pub fn can_settle_order(listing_time: T::Moment, expiration_time: T::Moment) -> bool {
			let now: T::Moment = <timestamp::Pallet<T>>::now();
			(listing_time < now) && (expiration_time == Zero::zero() || now < expiration_time)
		}

		// Calculate the settlement price of an order
		// Precondition: parameters have passed validate_parameters.
		// side OrderType side
		// sale_kind Method of sale
		// base_price OrderType base price
		// extra OrderType extra price data
		// listing_time OrderType listing time
		// expiration_time OrderType expiration time
		pub fn calculate_final_price(
			side: &Side,
			sale_kind: &SaleKind,
			base_price: BalanceOf<T>,
			extra: T::Moment,
			listing_time: T::Moment,
			expiration_time: T::Moment,
		) -> BalanceOf<T> {
			if *sale_kind == SaleKind::FixedPrice {
				base_price
			} else if *sale_kind == SaleKind::DutchAuction {
				let now: T::Moment = Zero::zero();
				let diff: T::Moment =
					extra * (now - listing_time) / (expiration_time - listing_time);
				if *side == Side::Sell {
					// Sell-side - start price: base_price. End price: base_price - extra.
					base_price - <exchange_common::Pallet<T>>::moment_to_balance(&diff)
				} else {
					// Buy-side - start price: base_price. End price: base_price + extra.
					base_price - <exchange_common::Pallet<T>>::moment_to_balance(&diff)
				}
			} else {
				Zero::zero()
			}
		}
	}
}
