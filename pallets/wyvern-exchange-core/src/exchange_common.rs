//! # Pacific Store - Wyvern Exchange pallet

#![cfg_attr(not(feature = "std"), no_std)]

use core::convert::TryInto;
use core::result::Result;

use sp_std::if_std;

use frame_support::{
    decl_module, ensure,
    sp_runtime::traits::Zero,
    sp_std::prelude::*,
    traits::{Currency, LockableCurrency, ReservableCurrency},
};
use frame_system::{self as system};
pub type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

use crate::types::*;

pub trait Trait: system::Trait + timestamp::Trait {
    type Currency: ReservableCurrency<Self::AccountId>
        + LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    }
}


impl<T: Trait> Module<T> {
    //Replace Vec<u8> in an array with Vec<u8> in another array, guarded by a bitmask
    //Efficiency of this fn is a bit unpredictable because of the EVM's word-specific model (arrays under 32 Vec<u8> will be slower)
    // Mask must be the size of the byte array. A nonzero byte means the byte array can be changed.
    // array The original array
    // desired The target array
    // mask The mask specifying which bits can be changed
    // The updated byte array (the parameter will be modified inplace)
    pub fn guarded_array_replace(
        array: &mut Vec<u8>,
        desired: &[u8],
        mask: &[u8],
    ) -> bool {
        if    array.len() != desired.len()||array.len() != mask.len(){
             return false;
        }

        let arr = array.clone();
        for (i, &_item) in arr.iter().enumerate() {
            // Conceptually: array[i] = (!mask[i] && array[i]) || (mask[i] && desired[i]), bitwise in word chunks.
            array[i] = (!mask[i] & _item) | (mask[i] & desired[i]);
        }
        true
    }

    //Test if two arrays are equal
    // Arrays must be of equal length, otherwise will return false
    // a First array
    // b Second array
    // Whether or not all Vec<u8> in the arrays are equal
    pub fn array_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        a == b
    }

    pub fn build_order_type_from_array_parameters(
        addrs: Vec<T::AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: &[u8],
        replacement_pattern: &[u8],
        static_extradata: &[u8],
    ) -> OrderType<T::AccountId, T::Moment, BalanceOf<T>> {

        Self::build_order_type(
            addrs[0].clone(),
            addrs[1].clone(),
            addrs[2].clone(),
            Self::u64_to_balance_saturated(uints[0]),
            Self::u64_to_balance_saturated(uints[1]),
            Self::u64_to_balance_saturated(uints[2]),
            Self::u64_to_balance_saturated(uints[3]),
            addrs[3].clone(),
            fee_method,
            side,
            sale_kind,
            addrs[4].clone(),
            how_to_call,
            calldata.to_vec(),
            replacement_pattern.to_vec(),
            addrs[5].clone(),
            static_extradata.to_vec(),
            addrs[6].clone(),
            Self::u64_to_balance_saturated(uints[4]),
            Self::u64_to_moment_saturated(uints[5]),
            Self::u64_to_moment_saturated(uints[6]),
            Self::u64_to_moment_saturated(uints[7]),
            uints[8],
        )
    }

    pub fn build_buy_sell_order_type(
        addrs: Vec<T::AccountId>,
        uints: Vec<u64>,
        fee_methods_sides_kinds_how_to_calls: &[u8],
        calldata_buy: &[u8],
        calldata_sell: &[u8],
        replacement_pattern_buy: &[u8],
        replacement_pattern_sell: &[u8],
        static_extradata_buy: &[u8],
        static_extradata_sell: &[u8],
    ) -> Vec<OrderType<T::AccountId, T::Moment, BalanceOf<T>>> {
 
        let buy: OrderType<T::AccountId, T::Moment, BalanceOf<T>> = Self::build_order_type(
            addrs[0].clone(),
            addrs[1].clone(),
            addrs[2].clone(),
            Self::u64_to_balance_saturated(uints[0]),
            Self::u64_to_balance_saturated(uints[1]),
            Self::u64_to_balance_saturated(uints[2]),
            Self::u64_to_balance_saturated(uints[3]),
            addrs[3].clone(),
            FeeMethod::from(fee_methods_sides_kinds_how_to_calls[0]),
            Side::from(fee_methods_sides_kinds_how_to_calls[1]),
            SaleKind::from(fee_methods_sides_kinds_how_to_calls[2]),
            addrs[4].clone(),
            HowToCall::from(fee_methods_sides_kinds_how_to_calls[3]),
            calldata_buy.to_vec(),
            replacement_pattern_buy.to_vec(),
            addrs[5].clone(),
            static_extradata_buy.to_vec(),
            addrs[6].clone(),
            Self::u64_to_balance_saturated(uints[4]),
            Self::u64_to_moment_saturated(uints[5]),
            Self::u64_to_moment_saturated(uints[6]),
            Self::u64_to_moment_saturated(uints[7]),
            uints[8],
        );
        let sell: OrderType<T::AccountId, T::Moment, BalanceOf<T>> = Self::build_order_type(
            addrs[7].clone(),
            addrs[8].clone(),
            addrs[9].clone(),
            Self::u64_to_balance_saturated(uints[9]),
            Self::u64_to_balance_saturated(uints[10]),
            Self::u64_to_balance_saturated(uints[11]),
            Self::u64_to_balance_saturated(uints[12]),
            addrs[10].clone(),
            FeeMethod::from(fee_methods_sides_kinds_how_to_calls[4]),
            Side::from(fee_methods_sides_kinds_how_to_calls[5]),
            SaleKind::from(fee_methods_sides_kinds_how_to_calls[6]),
            addrs[11].clone(),
            HowToCall::from(fee_methods_sides_kinds_how_to_calls[7]),
            calldata_sell.to_vec(),
            replacement_pattern_sell.to_vec(),
            addrs[12].clone(),
            static_extradata_sell.to_vec(),
            addrs[13].clone(),
            Self::u64_to_balance_saturated(uints[13]),
            Self::u64_to_moment_saturated(uints[14]),
            Self::u64_to_moment_saturated(uints[15]),
            Self::u64_to_moment_saturated(uints[16]),
            uints[17].into(),
        );
        vec![buy, sell]
    }

    pub fn build_order_type(
        exchange: T::AccountId,
        maker: T::AccountId,
        taker: T::AccountId,
        maker_relayer_fee: BalanceOf<T>,
        taker_relayer_fee: BalanceOf<T>,
        maker_protocol_fee: BalanceOf<T>,
        taker_protocol_fee: BalanceOf<T>,
        fee_recipient: T::AccountId,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        target: T::AccountId,
        how_to_call: HowToCall,
        calldata: Bytes,
        replacement_pattern: Bytes,
        static_target: T::AccountId,
        static_extradata: Bytes,
        payment_token: T::AccountId,
        base_price: BalanceOf<T>,
        extra: T::Moment,
        listing_time: T::Moment,
        expiration_time: T::Moment,
        salt: u64,
    ) -> OrderType<T::AccountId, T::Moment, BalanceOf<T>> {

        OrderType::<T::AccountId, T::Moment, BalanceOf<T>>::new(
            exchange,
                maker,
                taker,
                maker_relayer_fee,
                taker_relayer_fee,
                maker_protocol_fee,
                taker_protocol_fee,
                fee_recipient,
                fee_method,
                side,
                sale_kind,
                target,
                how_to_call,
                calldata,
                replacement_pattern,
                static_target,
                static_extradata,
                payment_token,
                base_price,
                extra,
                listing_time,
                expiration_time,
                salt,
        )
    }

    pub fn u64_to_balance_saturated(_input: u64) -> BalanceOf<T> {
        if let Some(_balance) = Self::u64_to_balance_option(_input) {
           return _balance;
        } 
        Zero::zero()
    }

    pub fn u64_to_moment_saturated(_input: u64) -> T::Moment {
        if let Some(_moment) = Self::u64_to_moment_option(_input) {
            return _moment;
        }
        Zero::zero()
    }

    pub fn u64_to_moment_option(_input: u64) -> Option<T::Moment> {
        _input.try_into().ok()
    }

    pub fn u64_to_balance_option(_input: u64) -> Option<BalanceOf<T>> {
        _input.try_into().ok()
    }
    pub fn balance_to_u128(input: BalanceOf<T>) -> Option<u128> {
        TryInto::<u128>::try_into(input).ok()
    }
    pub fn balance_to_u64_option(input: BalanceOf<T>) -> Option<u64> {
        TryInto::<u64>::try_into(input).ok()
    }

    pub fn moment_to_u64_option(input: T::Moment) -> Option<u64> {
        TryInto::<u64>::try_into(input).ok()
    }

    pub fn moment_to_balance(_moment: &T::Moment) -> BalanceOf<T> {
        if let Some(_moment) = Self::moment_to_u64_option(*_moment) {
            if let Some(_balance) = Self::u64_to_balance_option(_moment) {
                return _balance;
            }
        }

        Zero::zero()
    }
}
