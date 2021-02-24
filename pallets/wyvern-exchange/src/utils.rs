//! # Substrate Enterprise Sample - OrderType Post example pallet

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "512"]

use codec::{Decode, Encode};
use core::convert::TryInto;
use core::result::Result;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
// use sp_std::convert::{TryFrom, TryInto};
use sp_std::if_std;

use frame_support::{
    debug, decl_error, decl_event, decl_module, decl_storage,
    dispatch::{DispatchError, DispatchResult, DispatchResultWithPostInfo},
    ensure,
    sp_io::hashing::keccak_256,
    sp_runtime::{
        print,
        traits::{
            DispatchInfoOf, Dispatchable, IdentifyAccount, Member, PostDispatchInfoOf, Printable,
            SaturatedConversion, Saturating, SignedExtension, Verify, Zero,
        },
        MultiSignature, RuntimeDebug,
    },
    sp_std::collections::btree_set::BTreeSet,
    sp_std::prelude::*,
    traits::{
        Currency, ExistenceRequirement::AllowDeath, Get, LockableCurrency, Randomness,
        ReservableCurrency,
    },
};
use balances::Call as BalancesCall;
use frame_system::{self as system, ensure_signed};
pub type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

use crate::types::*;

pub trait Trait: system::Trait + timestamp::Trait {
    type Currency: ReservableCurrency<Self::AccountId>
        + LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        OrderIdMissing,
        OrderIdTooLong,
        OrderIdExists,
        OrdersCannotMatch,
        OrdersCannotMatch1,
        OrderInvalidFieldName,
        ArraySizeNotAsSameAsDesired,
        ArraySizeNotAsSameAsMask,
        BuyTakerProtocolFeeGreaterThanSellTakerProtocolFee,
        BuyTakerRelayerFeeGreaterThanSellTakerRelayerFee,
        SellPaymentTokenEqualPaymentToken,
        SellTakerProtocolFeeGreaterThanBuyTakerProtocolFee,
        SellTakerRelayerFeeGreaterThanBuyTakerRelayerFee,
        ValueLessThanRequiredAmount,
        ValueNotZero,
        BuyPriceLessThanSellPrice,
        OrderHashMissing,
        OnlyMaker,
        OrderHashInvalid,
        OrderHashInvalid1,
        OrderHashInvalid2,
        OrderHashInvalid3,
        OrderHashInvalid4,
        OrderHashInvalid5,
        OrderHashInvalid6,
        OrderHashInvalid7,
    }
}

impl<T: Trait> Module<T> {
    //
    //Replace Vec<u8> in an array with Vec<u8> in another array, guarded by a bitmask
    //Efficiency of this fn is a bit unpredictable because of the EVM's word-specific model (arrays under 32 Vec<u8> will be slower)
    //#dev Mask must be the size of the byte array. A nonzero byte means the byte array can be changed.
    //#param array The original array
    //#param desired The target array
    //#param mask The mask specifying which bits can be changed
    //#return The updated byte array (the parameter will be modified inplace)
    //
    pub fn guarded_array_replace(
        array: &mut Vec<u8>,
        desired: &[u8],
        mask: &[u8],
    ) -> Result<bool, Error<T>> {
        ensure!(
            array.len() == desired.len(),
            Error::<T>::ArraySizeNotAsSameAsDesired
        );
        ensure!(
            array.len() == mask.len(),
            Error::<T>::ArraySizeNotAsSameAsMask
        );
        let arr = array.clone();
        for (i, &_item) in arr.iter().enumerate() {
            // Conceptually: array[i] = (!mask[i] && array[i]) || (mask[i] && desired[i]), bitwise in word chunks.
            array[i] = (!mask[i] & _item) | (mask[i] & desired[i]);
        }
        Ok(true)
    }

    //
    //Test if two arrays are equal
    //Source: https://github.com/GNSPS/solidity-Vec<u8>-utils/blob/master/contracts/BytesLib.sol
    //#dev Arrays must be of equal length, otherwise will return false
    //#param a First array
    //#param b Second array
    //#return Whether or not all Vec<u8> in the arrays are equal
    //
    pub fn array_eq(a: &[u8], b: &[u8]) -> Result<bool, Error<T>> {
        if a.len() != b.len() {
            return Ok(false);
        }

        Ok(a == b)
    }

    pub fn build_order_type_arr(
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

    pub fn build_order_type_arr2(
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
        // OrderType maker AccountId.
        maker: T::AccountId,
        // OrderType taker AccountId, if specified.
        taker: T::AccountId,
        // Maker relayer fee of the order, unused for taker order.
        maker_relayer_fee: BalanceOf<T>,
        // Taker relayer fee of the order, or maximum taker fee for a taker order.
        taker_relayer_fee: BalanceOf<T>,
        // Maker protocol fee of the order, unused for taker order.
        maker_protocol_fee: BalanceOf<T>,
        // Taker protocol fee of the order, or maximum taker fee for a taker order.
        taker_protocol_fee: BalanceOf<T>,
        // OrderType fee recipient or zero AccountId for taker order.
        fee_recipient: T::AccountId,
        // Fee method (protocol token or split fee).
        fee_method: FeeMethod,
        // Side (buy/sell).
        side: Side,
        // Kind of sale.
        sale_kind: SaleKind,
        // Target.
        target: T::AccountId,
        // Vec<u8>.
        how_to_call: HowToCall,
        // Calldata.
        calldata: Bytes,
        // Calldata replacement pattern, or an empty byte array for no replacement.
        replacement_pattern: Bytes,
        // Static call target, zero-AccountId for no static call.
        static_target: T::AccountId,
        // Static call extra data.
        static_extradata: Bytes,
        // Token used to pay for the order, or the zero-AccountId as a sentinel value for Ether.
        payment_token: T::AccountId,
        // Base price of the order (in paymentTokens).
        base_price: BalanceOf<T>,
        // Auction extra parameter - minimum bid increment for English auctions, starting/ending price difference.
        extra: T::Moment,
        // Listing timestamp.
        listing_time: T::Moment,
        // Expiration timestamp - 0 for no expiry.
        expiration_time: T::Moment,
        // OrderType salt, used to prevent duplicate hashes.
        salt: u64,
    ) -> OrderType<T::AccountId, T::Moment, BalanceOf<T>> {
        OrderType::<T::AccountId, T::Moment, BalanceOf<T>>::new(
            exchange,
            // OrderType maker AccountId.
            maker,
            // OrderType taker AccountId, if specified.
            taker,
            // Maker relayer fee of the order, unused for taker order.
            maker_relayer_fee,
            // Taker relayer fee of the order, or maximum taker fee for a taker order.
            taker_relayer_fee,
            // Maker protocol fee of the order, unused for taker order.
            maker_protocol_fee,
            // Taker protocol fee of the order, or maximum taker fee for a taker order.
            taker_protocol_fee,
            // OrderType fee recipient or zero AccountId for taker order.
            fee_recipient,
            // Fee method (protocol token or split fee).
            fee_method,
            // Side (buy/sell).
            side,
            // Kind of sale.
            sale_kind,
            // Target.
            target,
            // Vec<u8>.
            how_to_call,
            // Calldata.
            calldata,
            // Calldata replacement pattern, or an empty byte array for no replacement.
            replacement_pattern,
            // Static call target, zero-AccountId for no static call.
            static_target,
            // Static call extra data.
            static_extradata,
            // Token used to pay for the order, or the zero-AccountId as a sentinel value for Ether.
            payment_token,
            // Base price of the order (in paymentTokens).
            base_price,
            // Auction extra parameter - minimum bid increment for English auctions, starting/ending price difference.
            extra,
            // Listing timestamp.
            listing_time,
            // Expiration timestamp - 0 for no expiry.
            expiration_time,
            // OrderType salt, used to prevent duplicate hashes.
            salt,
        )
    }

    pub fn u64_to_balance_saturated(_input: u64) -> BalanceOf<T> {
        if let Some(b) = Self::u64_to_balance_option(_input) {
            b
        } else {
            Zero::zero()
        }
    }

    pub fn u64_to_moment_saturated(_input: u64) -> T::Moment {
        // let my_u32:u32 = _input as u32;
        //  my_u32.into()
        if let Some(b) = Self::u64_to_moment_option(_input) {
            b
        } else {
            Zero::zero()
        }
    }

    pub fn u64_to_moment_option(_input: u64) -> Option<T::Moment> {
        // use sp_std::convert::{TryFrom, TryInto};
        _input.try_into().ok()
        // Some(Zero::zero())
    }

    pub fn u64_to_balance_option(_input: u64) -> Option<BalanceOf<T>> {
        // use sp_std::convert::{TryFrom, TryInto};
        _input.try_into().ok()
        // Some(Zero::zero())
    }
    pub fn balance_to_u128(input: BalanceOf<T>) -> Option<u128> {
        // use sp_std::convert::{TryFrom, TryInto};
        TryInto::<u128>::try_into(input).ok()

        // Some(input.saturated_into::<u64>())
    }
    pub fn balance_to_u64_option(input: BalanceOf<T>) -> Option<u64> {
        // use sp_std::convert::{TryFrom, TryInto};
        TryInto::<u64>::try_into(input).ok()
    }

    pub fn moment_to_u64_option(input: T::Moment) -> Option<u64> {
        // use sp_std::convert::{TryFrom, TryInto};
        TryInto::<u64>::try_into(input).ok()
    }

    // pub fn balance_to_u64_saturated(input: BalanceOf<T>) -> u64 {
    //     input.saturated_into::<u64>()
    // }
    // pub fn moment_to_u64_saturated(input: T::Moment) -> u64 {
    //     input.saturated_into::<u64>()
    // }

    pub fn moment_to_balance(m: &T::Moment) -> BalanceOf<T> {
        let mut _b: BalanceOf<T> = Zero::zero();
        if let Some(m) = Self::moment_to_u64_option(*m) {
            if let Some(bo) = Self::u64_to_balance_option(m) {
                _b = bo;
            }
        }

        _b
    }
}
