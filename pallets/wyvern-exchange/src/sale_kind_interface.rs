//! # Substrate Enterprise Sample - OrderType Post example pallet

#![cfg_attr(not(feature = "std"), no_std)]

use core::result::Result;

use sp_std::if_std;

use frame_support::{
     decl_module, 
    ensure,
    sp_runtime::{
        traits::{
            Zero,
        },
    },
    sp_std::prelude::*,
};
use frame_system::{self as system};

use crate::types::*;

use crate::utils;
use crate::utils::Error;
use crate::utils::BalanceOf;
pub trait Trait: system::Trait + timestamp::Trait+ utils::Trait {

}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}


impl<T: Trait> Module<T> {
    // sale Kind interface
    //
    //#dev Check whether the parameters of a sale are valid
    //#param sale_kind Kind of sale
    //#param expiration_time OrderType expiration time
    //#return Whether the parameters were valid
    //
    pub fn validate_parameters(
        sale_kind: &SaleKind,
        expiration_time: T::Moment,
    ) -> Result<bool, Error<T>> {
        // Auctions must have a set expiration date.
        Ok(*sale_kind == SaleKind::FixedPrice || expiration_time > Zero::zero())
    }

    //
    //#dev Return whether or not an order can be settled
    //#dev Precondition: parameters have passed validate_parameters
    //#param listing_time OrderType listing time
    //#param expiration_time OrderType expiration time
    //
    pub fn can_settle_order(
        listing_time: T::Moment,
        expiration_time: T::Moment,
    ) -> Result<bool, Error<T>> {
        if_std! {
            // This code is only being compiled and executed when the `std` feature is enabled.
            println!("Hello native world!");
            println!("My value is: {:#?}", listing_time);
            println!("The caller account is: {:#?}", <timestamp::Module<T>>::now());
        }
        let now: T::Moment = <timestamp::Module<T>>::now(); //Self::u64_to_moment_saturated(100); //<timestamp::Module<T>>::now();//<system::Module<T>>::block_number() ;////<timestamp::Module<T>>::now();
        ensure!(
            (listing_time < now) && (expiration_time == Zero::zero() || now < expiration_time),
            Error::<T>::OrdersCannotMatch1
        );
        Ok((listing_time < now) && (expiration_time == Zero::zero() || now < expiration_time))
    }

    //
    //#dev Calculate the settlement price of an order
    //#dev Precondition: parameters have passed validate_parameters.
    //#param side OrderType side
    //#param sale_kind Method of sale
    //#param base_price OrderType base price
    //#param extra OrderType extra price data
    //#param listing_time OrderType listing time
    //#param expiration_time OrderType expiration time
    //
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
                Ok(base_price - <utils::Module<T>>::moment_to_balance(&diff))
            } else {
                // Buy-side - start price: base_price. End price: base_price + extra.
                Ok(base_price - <utils::Module<T>>::moment_to_balance(&diff))
            }
        } else {
            Ok(Zero::zero())
        }
    }
}
