//! # Pacific Store - Wyvern Exchange pallet

#![cfg_attr(not(feature = "std"), no_std)]

use core::result::Result;


use frame_support::{decl_error,decl_module, ensure, sp_runtime::traits::Zero, sp_std::prelude::*};
use frame_system::{self as system};

use crate::types::*;

use crate::exchange_common;
use crate::exchange_common::BalanceOf;
use crate::exchange_common::Error;
pub trait Trait: exchange_common::Trait  {}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}


impl<T: Trait> Module<T> {
    // sale Kind interface
    // Check whether the parameters of a sale are valid
    // sale_kind Kind of sale
    // expiration_time OrderType expiration time
    //Whether the parameters were valid
    pub fn validate_parameters(
        sale_kind: &SaleKind,
        expiration_time: T::Moment,
    ) -> Result<bool, Error<T>> {
        // Auctions must have a set expiration date.
        Ok(*sale_kind == SaleKind::FixedPrice || expiration_time > Zero::zero())
    }

    // Return whether or not an order can be settled
    // Precondition: parameters have passed validate_parameters
    // listing_time OrderType listing time
    // expiration_time OrderType expiration time
    pub fn can_settle_order(
        listing_time: T::Moment,
        expiration_time: T::Moment,
    ) -> Result<bool, Error<T>> {

        let now: T::Moment = <timestamp::Module<T>>::now(); //Self::u64_to_moment_saturated(100); //<timestamp::Module<T>>::now();//<system::Module<T>>::block_number() ;////<timestamp::Module<T>>::now();
        ensure!(
            (listing_time < now) && (expiration_time == Zero::zero() || now < expiration_time),
            Error::<T>::ListingTimeExpired
        );
        Ok((listing_time < now) && (expiration_time == Zero::zero() || now < expiration_time))
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
    ) -> Result<BalanceOf<T>, Error<T>> {
        if *sale_kind == SaleKind::FixedPrice {
            Ok(base_price)
        } else if *sale_kind == SaleKind::DutchAuction {
            let now: T::Moment = Zero::zero(); // <system::Module<T>>::block_number();//<timestamp::Module<T>>::now() ;
            let diff: T::Moment = extra * (now - listing_time) / (expiration_time - listing_time);
            if *side == Side::Sell {
                // Sell-side - start price: base_price. End price: base_price - extra.
                Ok(base_price - <exchange_common::Module<T>>::moment_to_balance(&diff))
            } else {
                // Buy-side - start price: base_price. End price: base_price + extra.
                Ok(base_price - <exchange_common::Module<T>>::moment_to_balance(&diff))
            }
        } else {
            Ok(Zero::zero())
        }
    }
}
