//! # Substrate Enterprise Sample - OrderType Post example pallet

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use core::result::Result;


use frame_support::{
    debug,  decl_event, decl_module, decl_storage,
    dispatch::{DispatchResult},
    ensure,
    sp_io::hashing::keccak_256,
    sp_runtime::{
        print,
        traits::{
             IdentifyAccount, Member, 
             Verify, Zero,
        },
        
    },
    sp_std::prelude::*,
    traits::{
        Currency, 
    },
};

use frame_system::{self as system, ensure_signed};

use crate::types::*;


use crate::utils;
use crate::utils::Error;
use crate::utils::BalanceOf;
use crate::sale_kind_interface;



pub trait Trait: system::Trait + timestamp::Trait+ sale_kind_interface::Trait+ utils::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Public: IdentifyAccount<AccountId = Self::AccountId> + Clone;
    type Signature: Verify<Signer = Self::Public> + Member + Decode + Encode;
    // type Currency: ReservableCurrency<Self::AccountId>
    //     + LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
}

decl_storage! {
    trait Store for Module<T: Trait> as ExchangeCore {
        NextOrderIndex: BalanceOf<T>;
        pub ContractSelf:T::AccountId;
        // // The token used to pay exchange fees.
        pub ExchangeToken:T::AccountId;

        // // Cancelled / finalized orders, by hash.
        // mapping(Vec<u8> => bool) public CancelledOrFinalized;
        pub CancelledOrFinalized get(fn cancelled_or_finalized): map hasher(blake2_128_concat) Vec<u8> => bool;
        // // Orders verified by on-chain approval (alternative to ECDSA signatures so that smart contracts can place orders directly).
        // mapping(Vec<u8> => bool) public ApprovedOrders;
        pub ApprovedOrders get(fn approved_orders): map hasher(blake2_128_concat) Vec<u8> => bool;
        // // For split fee orders, minimum required protocol maker fee, in basis points. Paid to owner (who can change it).
        // BalanceOf<T> public MinimumMakerProtocolFee = 0;
        pub MinimumMakerProtocolFee:BalanceOf<T>;
        // // For split fee orders, minimum required protocol taker fee, in basis points. Paid to owner (who can change it).
        // BalanceOf<T> public MinimumTakerProtocolFee = 0;
        pub MinimumTakerProtocolFee:BalanceOf<T>;
        // // Recipient of protocol fees.
        // AccountId public ProtocolFeeRecipient;
        pub ProtocolFeeRecipient:T::AccountId;


 }
}


decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Balance = BalanceOf<T>,
        Moment = <T as timestamp::Trait>::Moment,
    {
        // event OrderApprovedPartOne    (Vec<u8> indexed hash, AccountId exchange, AccountId indexed maker, AccountId taker,
        // BalanceOf<T> maker_relayer_fee, BalanceOf<T> taker_relayer_fee, BalanceOf<T> maker_protocol_fee, BalanceOf<T> taker_protocol_fee,
        // AccountId indexed fee_recipient, FeeMethod fee_method, SaleKindInterface.Side side, SaleKindInterface.SaleKind sale_kind, AccountId target);
        // event OrderApprovedPartTwo    (Vec<u8> indexed hash, AuthenticatedProxy.Vec<u8> how_to_call, Vec<u8> calldata, Vec<u8> replacement_pattern,
        // AccountId static_target, Vec<u8> static_extradata, AccountId payment_token, BalanceOf<T> base_price,
        // BalanceOf<T> extra, BalanceOf<T> listing_time, BalanceOf<T> expiration_time, BalanceOf<T> salt, bool orderbook_inclusion_desired);
        // event OrderCancelled          (Vec<u8> indexed hash);
        // event OrdersMatched           (Vec<u8> buy_hash, Vec<u8> sell_hash, AccountId indexed maker, AccountId indexed taker, BalanceOf<T> price, Vec<u8> indexed metadata);
        OrderApprovedPartOne(
            Vec<u8>,
            AccountId,
            AccountId,
            AccountId,
            Balance,
            Balance,
            Balance,
            Balance,
            AccountId,
            FeeMethod,
            Side,
            SaleKind,
            AccountId,
        ),
        OrderApprovedPartTwo(
            Vec<u8>,
            HowToCall,
            Vec<u8>,
            Vec<u8>,
            AccountId,
            Vec<u8>,
            AccountId,
            Balance,
            Moment,
            Moment,
            Moment,
            u64,
            bool,
        ),
        OrderCancelled(Vec<u8>),
        OrdersMatched(Vec<u8>, Vec<u8>, AccountId, AccountId, Balance, Vec<u8>),
        MinimumMakerProtocolFeeChanged(Balance),
        MinimumTakerProtocolFeeChanged(Balance),
        ProtocolFeeRecipientChanged(AccountId, AccountId),
    }
);

// decl_error! {
//     pub enum Error for Module<T: Trait> {
//         OrderIdMissing,
//         OrderIdTooLong,
//         OrderIdExists,
//         OrdersCannotMatch,
//         OrdersCannotMatch1,
//         OrderInvalidFieldName,
//         ArraySizeNotAsSameAsDesired,
//         ArraySizeNotAsSameAsMask,
//         BuyTakerProtocolFeeGreaterThanSellTakerProtocolFee,
//         BuyTakerRelayerFeeGreaterThanSellTakerRelayerFee,
//         SellPaymentTokenEqualPaymentToken,
//         SellTakerProtocolFeeGreaterThanBuyTakerProtocolFee,
//         SellTakerRelayerFeeGreaterThanBuyTakerRelayerFee,
//         ValueLessThanRequiredAmount,
//         ValueNotZero,
//         BuyPriceLessThanSellPrice,
//         OrderHashMissing,
//         OnlyMaker,
//         OrderHashInvalid,
//         OrderHashInvalid1,
//         OrderHashInvalid2,
//         OrderHashInvalid3,
//         OrderHashInvalid4,
//         OrderHashInvalid5,
//         OrderHashInvalid6,
//         OrderHashInvalid7,
//     }
// }

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

    //exchange core
//
//#dev Change the minimum maker fee paid to the protocol (only:owner)
//#param newMinimumMakerProtocolFee New fee to set in basis points
//
#[weight = 10_000]
    pub fn change_minimum_maker_protocol_fee(
  origin,
        new_minimum_maker_protocol_fee: BalanceOf<T>,
    ) -> DispatchResult
    {
// onlyOwner
        frame_support::debug::RuntimeLogger::init();
        debug::error!("exchange is contract self.");

let _user = ensure_signed(origin)?;
        MinimumMakerProtocolFee::<T>::put(new_minimum_maker_protocol_fee);
   Self::deposit_event(RawEvent::MinimumMakerProtocolFeeChanged(new_minimum_maker_protocol_fee));

        Ok(())
    }

//
//#dev Change the minimum taker fee paid to the protocol (only:owner)
//#param new_minimum_taker_protocol_fee New fee to set in basis points
//
#[weight = 10_000]
    pub fn change_minimum_taker_protocol_fee(
  origin,
        new_minimum_taker_protocol_fee: BalanceOf<T>,
    ) -> DispatchResult {
        // onlyOwner
let _user = ensure_signed(origin)?;

        MinimumTakerProtocolFee::<T>::put(new_minimum_taker_protocol_fee);
           Self::deposit_event(RawEvent::MinimumTakerProtocolFeeChanged(new_minimum_taker_protocol_fee));

Ok(())
    }

//
//#dev Change the protocol fee recipient (only:owner)
//#param new_protocol_fee_recipient New protocol fee recipient AccountId
//
#[weight = 10_000]
pub fn change_protocol_fee_recipient(
origin,
new_protocol_fee_recipient: T::AccountId,
) -> DispatchResult {
print("================");
// onlyOwner
let _user = ensure_signed(origin)?;

ProtocolFeeRecipient::<T>::put(new_protocol_fee_recipient.clone());
           Self::deposit_event(RawEvent::ProtocolFeeRecipientChanged(_user,new_protocol_fee_recipient.clone()));
Ok(())
}


 }
}


impl<T: Trait> Module<T> {
    //
    //#dev Transfer tokens
    //#param token Token to transfer
    //#param from AccountId to charge fees
    //#param to AccountId to receive fees
    //#param amount Amount of protocol tokens to charge
    //
    pub fn transfer_tokens(
        _token: &T::AccountId,
        _from: &T::AccountId,
        _to: &T::AccountId,
        _amount: BalanceOf<T>,
    ) -> Result<(), Error<T>> {
        if _amount > Zero::zero() {
            let _ = T::Currency::transfer(
                &_from,
                &_to,
                _amount,
                frame_support::traits::ExistenceRequirement::AllowDeath,
            );
        }
        Ok(())
    }

    pub fn transfer_tokens_fee(
        _token: &T::AccountId,
        _from: &T::AccountId,
        _to: &T::AccountId,
        _amount: BalanceOf<T>,
        _price: &BalanceOf<T>,
    ) -> Result<(), Error<T>> {
        if _amount > Zero::zero() {
            let _amount = _amount * *_price / INVERSE_BASIS_POINT.into();
            Self::transfer_tokens(_token, _from, _to, _amount)?;
        }
        Ok(())
    }

    pub fn transfer_tokens_fee_sell(
        _token: &T::AccountId,
        _from: &T::AccountId,
        _to: &T::AccountId,
        _amount: BalanceOf<T>,
        _price: &BalanceOf<T>,
        receive_or_required_amount: &mut BalanceOf<T>,
        is_maker: bool,
    ) -> Result<(), Error<T>> {
        if _amount > Zero::zero() {
            let _fee = _amount * *_price / INVERSE_BASIS_POINT.into();
            let mut _from_ = (*_from).clone();
            if *_token == ContractSelf::<T>::get() {
                if is_maker {
                    *receive_or_required_amount -= _fee;
                } else {
                    *receive_or_required_amount += _fee;
                };

                _from_ = ContractSelf::<T>::get();
            }

            Self::transfer_tokens(_token, _from, _to, _amount)?;
        }
        Ok(())
    }

    //#dev Charge a fee in protocol tokens
    //#param from AccountId to charge fees
    //#param to AccountId to receive fees
    //#param amount Amount of protocol tokens to charge
    //
    pub fn charge_protocol_fee(
        from: &T::AccountId,
        to: &T::AccountId,
        amount: BalanceOf<T>,
    ) -> Result<(), Error<T>> {
        Self::transfer_tokens(&ExchangeToken::<T>::get(), &from, &to, amount)
    }

    //
    //#dev Hash an order, returning the canonical order hash, without the message prefix
    //#param order OrderType to hash
    //#return Hash of order
    //
    pub fn hash_order(
        order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
    ) -> Result<Vec<u8>, Error<T>> {
        // hash := keccak256(add(array, 0x20), size)
        //    sp_io::hashing::blake2_256(&h).into()
        Ok(keccak_256(&order.encode()).into())
        // }
        // }
        // return hash;
    }

    //
    //#dev Hash an order, returning the hash that a client must sign, including the standard message prefix
    //#param order OrderType to hash
    //#return Hash of message prefix and order hash per Ethereum format
    //
    pub fn hash_to_sign(
        order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
    ) -> Result<Vec<u8>, Error<T>> {
        Ok(keccak_256(&Self::hash_order(&order)?).to_vec())
    }

    //
    //#dev Assert an order is valid and return its hash
    //#param order OrderType to validate
    //#param sig ECDSA signature
    //
    pub fn require_valid_order(
        order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
        sig: &T::Signature,
    ) -> Result<Vec<u8>, Error<T>> {
        let hash: Vec<u8> = Self::hash_to_sign(&order)?;
        ensure!(
            Self::validate_order(&hash, order, sig)?,
            Error::<T>::OrderHashInvalid
        );
        Ok(hash)
    }

    //
    //#dev Validate order parameters (does *not* check validity:signature)
    //#param order OrderType to validate
    //
    pub fn validate_order_parameters(
        order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
    ) -> Result<bool, Error<T>> {
        // OrderType must be targeted at this protocol version (this contract:Exchange).
        //TODO
        if order.exchange != ContractSelf::<T>::get() {
            return Ok(false);
        }

        // OrderType must possess valid sale kind parameter combination.
        if !<sale_kind_interface::Module<T>>::validate_parameters(&order.sale_kind, order.expiration_time)? {
            return Ok(false);
        }

        // If using the split fee method, order must have sufficient protocol fees.
        if order.fee_method == FeeMethod::SplitFee
            && (order.maker_protocol_fee < MinimumMakerProtocolFee::<T>::get()
                || order.taker_protocol_fee < MinimumTakerProtocolFee::<T>::get())
        {
            return Ok(false);
        }

        Ok(true)
    }

    //
    //#dev Validate a provided previously approved / signed order, hash, and signature.
    //#param hash OrderType hash (calculated:already, passed to recalculation:avoid)
    //#param order OrderType to validate
    //#param sig ECDSA signature
    //
    pub fn validate_order(
        hash: &[u8],
        order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
        sig: &T::Signature,
    ) -> Result<bool, Error<T>> {
        // Not done in an if-conditional to prevent unnecessary ecrecover evaluation, which seems to happen even though it should short-circuit.
        frame_support::debug::RuntimeLogger::init();
        debug::error!("exchange is contract self.");
        print("================");
        // OrderType must have valid parameters.
        if !Self::validate_order_parameters(&order)? {
            return Ok(false);
        }

        // OrderType must have not been canceled or already filled.
        if CancelledOrFinalized::get(hash) {
            return Ok(false);
        }

        // OrderType authentication. OrderType must be either:
        // (a) previously approved
        if ApprovedOrders::get(hash) {
            return Ok(true);
        }

        // or (b) ECDSA-signed by maker.
        // if ecrecover(hash, sig.v, sig.r, sig.s) == order.maker {
        //     return true;
        // }
        if Self::check_signature(&sig, &hash, order.maker()).is_ok() {
            return Ok(true);
        }
        Ok(false)
    }

    // An alterantive way to validate a signature is:
    // Import the codec and traits:
    // Example function to verify the signature.

    pub fn check_signature(
        _signature: &T::Signature,
        _msg: &[u8],
        _signer: &T::AccountId,
    ) -> Result<(), Error<T>> {
        // let mut bytes = [u8; 32];
        // T::AccountId::decode(&mut &bytes[..]).unwrap_or_default();
        if _signature.verify(_msg, _signer) {
            Ok(())
        } else {
            Err(Error::<T>::OrderIdMissing.into())
        }
    }

    //
    //#dev Approve an order and optionally mark it for orderbook inclusion. Must be called by the maker of the order
    //#param order OrderType to approve
    //#param orderbook_inclusion_desired Whether orderbook providers should include the order in their orderbooks
    //
    pub fn approve_order(
        origin: T::Origin,
        order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
        orderbook_inclusion_desired: bool,
    ) -> DispatchResult {
        // CHECKS
        let _user = ensure_signed(origin)?;
        // Assert sender is authorized to approve order.
        ensure!(_user == order.maker, Error::<T>::OnlyMaker);

        // Calculate order hash.
        let hash: Vec<u8> = Self::hash_to_sign(&order)?;

        // Assert order has not already been approved.
        ensure!(
            !ApprovedOrders::get(hash.clone()),
            Error::<T>::OrderHashMissing
        );

        // EFFECTS

        // Mark order as approved.
        ApprovedOrders::insert(hash.clone(), true);

        // Log approval event. Must be split in two due to Solidity stack size limitations.
        Self::deposit_event(RawEvent::OrderApprovedPartOne(
            hash.clone(),
            order.exchange.clone(),
            order.maker.clone(),
            order.taker.clone(),
            order.maker_relayer_fee,
            order.taker_relayer_fee,
            order.maker_protocol_fee,
            order.taker_protocol_fee,
            order.fee_recipient.clone(),
            order.fee_method.clone(),
            order.side.clone(),
            order.sale_kind.clone(),
            order.target.clone(),
        ));

        Self::deposit_event(RawEvent::OrderApprovedPartTwo(
            hash.clone(),
            order.how_to_call.clone(),
            order.calldata.clone(),
            order.replacement_pattern.clone(),
            order.static_target.clone(),
            order.static_extradata.clone(),
            order.payment_token.clone(),
            order.base_price.clone(),
            order.extra.clone(),
            order.listing_time.clone(),
            order.expiration_time.clone(),
            order.salt.clone(),
            orderbook_inclusion_desired,
        ));

        Ok(())
    }

    //
    //#dev Cancel an order, preventing it from being matched. Must be called by the maker of the order
    //#param order OrderType to cancel
    //#param sig ECDSA signature
    //
    pub fn cancel_order(
        origin: T::Origin,
        order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
        sig: &T::Signature,
    ) -> DispatchResult {
        // CHECKS
        let _user = ensure_signed(origin)?;

        // Assert sender is authorized to cancel order.
        ensure!(_user == order.maker, Error::<T>::OnlyMaker);

        // Calculate order hash.
        let hash = Self::require_valid_order(order, sig)?;
        // EFFECTS
        // Mark order as cancelled, preventing it from being matched.
        CancelledOrFinalized::insert(hash.clone(), true);

        // Log cancel event.
        Self::deposit_event(RawEvent::OrderCancelled(hash.clone()));

        Ok(())
    }

    //
    //#dev Calculate the current price of an order (fn:convenience)
    //#param order OrderType to calculate the price of
    //#return The current price of the order
    //
    pub fn calculate_current_price(
        order: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
    ) -> Result<BalanceOf<T>, Error<T>> {
        <sale_kind_interface::Module<T>>::calculate_final_price(
            &order.side,
            &order.sale_kind,
            order.base_price,
            order.extra,
            order.listing_time,
            order.expiration_time,
        )
    }

    //
    //#dev Calculate the price two orders would match at, if in fact they would match (fail:otherwise)
    //#param buy Buy-side order
    //#param sell Sell-side order
    //#return Match price
    //
    pub fn calculate_match_price(
        buy: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
        sell: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
    ) -> Result<BalanceOf<T>, Error<T>> {
        // Calculate sell price.
        let sell_price: BalanceOf<T> = <sale_kind_interface::Module<T>>::calculate_final_price(
            &sell.side,
            &sell.sale_kind,
            sell.base_price,
            sell.extra,
            sell.listing_time,
            sell.expiration_time,
        )?;

        // Calculate buy price.
        let buy_price: BalanceOf<T> = <sale_kind_interface::Module<T>>::calculate_final_price(
            &buy.side,
            &buy.sale_kind,
            buy.base_price,
            buy.extra,
            buy.listing_time,
            buy.expiration_time,
        )?;

        // Require price cross.
        ensure!(
            buy_price >= sell_price,
            Error::<T>::BuyPriceLessThanSellPrice
        );

        // Maker/taker priority.
        let price: BalanceOf<T> = if sell.fee_recipient != ContractSelf::<T>::get() {
            sell_price
        } else {
            buy_price
        };

        Ok(price)
    }

    //
    //#dev Execute all ERC20 token / Ether transfers associated with an order match (fees and buyer => transfer:seller)
    //#param buy Buy-side order
    //#param sell Sell-side order
    //
    pub fn execute_funds_transfer(
        msg_value: BalanceOf<T>,
        buy: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
        sell: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
    ) -> Result<BalanceOf<T>, Error<T>> {
        // let originprotocol_fee_recipient = ProtocolFeeRecipient::<T>::get();
        // Only payable in the special case of unwrapped Ether.
        if sell.payment_token != ContractSelf::<T>::get() {
            ensure!(msg_value == Zero::zero(), Error::<T>::ValueNotZero);
        }

        // Calculate match price.
        let price: BalanceOf<T> = Self::calculate_match_price(&buy, &sell)?;

        // If paying using a token (Ether:not), transfer tokens. This is done prior to fee payments to that a seller will have tokens before being charged fees.
        if price > Zero::zero() && sell.payment_token != ContractSelf::<T>::get() {
            Self::transfer_tokens(sell.payment_token(), &buy.maker(), sell.maker(), price)?;
        }

        // Amount that will be received by seller (Ether:for).
        let mut receive_amount: BalanceOf<T> = price;

        // Amount that must be sent by buyer (Ether:for).
        let mut required_amount: BalanceOf<T> = price;

        // Determine maker/taker and charge fees accordingly.
        if sell.fee_recipient != ContractSelf::<T>::get() {
            // Sell-side order is maker.
            Self::execute_funds_transfer_sell_side(
                buy,
                sell,
                &price,
                &mut receive_amount,
                &mut required_amount,
            )?;
        // // Assert taker fee is less than or equal to maximum fee specified by buyer.
        // ensure!(
        //     sell.taker_relayer_fee <= buy.taker_relayer_fee,
        //     Error::<T>::OrderIdMissing
        // );

        // if sell.fee_method == FeeMethod::SplitFee {
        //     // Assert taker fee is less than or equal to maximum fee specified by buyer.
        //     ensure!(
        //         sell.taker_protocol_fee <= buy.taker_protocol_fee,
        //         Error::<T>::OrderIdMissing
        //     );

        //     // Maker fees are deducted from the token amount that the maker receives. Taker fees are extra tokens that must be paid by the taker.

        //     if sell.maker_relayer_fee > Zero::zero() {
        //         let maker_relayer_fee: BalanceOf<T> = sell.maker_relayer_fee * price / INVERSE_BASIS_POINT.into();
        //         if sell.payment_token == ContractSelf::<T>::get() {
        //             receive_amount = receive_amount - maker_relayer_fee;
        //             // sell.fee_recipient.transfer(maker_relayer_fee);
        //           Self::transfer_tokens(
        //                 &ContractSelf::<T>::get(),
        //                 &ContractSelf::<T>::get(),
        //                 &sell.fee_recipient,
        //                 maker_relayer_fee,
        //             )?;
        //         } else {
        //           Self::transfer_tokens(
        //                 sell.payment_token(),
        //                 sell.maker(),
        //                 &sell.fee_recipient,
        //                 maker_relayer_fee,
        //             )?;
        //         }
        //     }

        //     if sell.taker_relayer_fee > Zero::zero() {
        //         let taker_relayer_fee: BalanceOf<T> = sell.taker_relayer_fee * price / INVERSE_BASIS_POINT.into();
        //         if sell.payment_token == ContractSelf::<T>::get() {
        //             required_amount = required_amount + taker_relayer_fee;
        //             // sell.fee_recipient.transfer(taker_relayer_fee);
        //           Self::transfer_tokens(
        //                 &ContractSelf::<T>::get(),
        //                 &ContractSelf::<T>::get(),
        //                 &sell.fee_recipient,
        //                 taker_relayer_fee,
        //             )?;
        //         } else {
        //           Self::transfer_tokens(
        //                 sell.payment_token(),
        //                 buy.maker(),
        //                 &sell.fee_recipient,
        //                 taker_relayer_fee,
        //             )?;
        //         }
        //     }

        //     if sell.maker_protocol_fee > Zero::zero() {
        //         let maker_protocol_fee: BalanceOf<T> = sell.maker_protocol_fee * price / INVERSE_BASIS_POINT.into();
        //         if sell.payment_token == ContractSelf::<T>::get() {
        //             receive_amount = receive_amount - maker_protocol_fee;
        //             // ProtocolFeeRecipient.transfer(maker_protocol_fee);
        //           Self::transfer_tokens(
        //                 &ContractSelf::<T>::get(),
        //                 &ContractSelf::<T>::get(),
        //                 &originprotocol_fee_recipient,
        //                 maker_protocol_fee,
        //             )?;
        //         } else {
        //           Self::transfer_tokens(
        //                 sell.payment_token(),
        //                 sell.maker(),
        //                 &originprotocol_fee_recipient,
        //                 maker_protocol_fee,
        //             )?;
        //         }
        //     }

        //     if sell.taker_protocol_fee > Zero::zero() {
        //         let taker_protocol_fee: BalanceOf<T> = sell.taker_protocol_fee * price / INVERSE_BASIS_POINT.into();
        //         if sell.payment_token == ContractSelf::<T>::get() {
        //             required_amount = required_amount + taker_protocol_fee;
        //             // ProtocolFeeRecipient.transfer(taker_protocol_fee);
        //           Self::transfer_tokens(
        //                 &ContractSelf::<T>::get(),
        //                 &ContractSelf::<T>::get(),
        //                 &originprotocol_fee_recipient,
        //                 taker_protocol_fee,
        //             )?;
        //         } else {
        //           Self::transfer_tokens(
        //                 sell.payment_token(),
        //                 buy.maker(),
        //                 &originprotocol_fee_recipient,
        //                 taker_protocol_fee,
        //             )?;
        //         }
        //     }
        // } else {
        //     // Charge maker fee to seller.
        //   Self::charge_protocol_fee(&sell.maker, &sell.fee_recipient, sell.maker_relayer_fee)?;

        //     // Charge taker fee to buyer.
        //   Self::charge_protocol_fee(&buy.maker, &sell.fee_recipient, sell.taker_relayer_fee)?;
        // }
        } else {
            // Buy-side order is maker.
            Self::execute_funds_transfer_buy_side(buy, sell, &price)?;

            // // Assert taker fee is less than or equal to maximum fee specified by seller.
            // ensure!(
            //     buy.taker_relayer_fee <= sell.taker_relayer_fee,
            //     Error::<T>::OrderIdMissing
            // );

            // if sell.fee_method == FeeMethod::SplitFee {
            //     // The Exchange does not escrow Ether, so direct Ether can only be used to with sell-side maker / buy-side taker orders.
            //     ensure!(sell.payment_token != ContractSelf::<T>::get(), Error::<T>::OrderIdMissing);

            //     // Assert taker fee is less than or equal to maximum fee specified by seller.
            //     ensure!(
            //         buy.taker_protocol_fee <= sell.taker_protocol_fee,
            //         Error::<T>::OrderIdMissing
            //     );

            //     if buy.maker_relayer_fee > Zero::zero() {
            //        let maker_relayer_fee =buy.maker_relayer_fee * price / INVERSE_BASIS_POINT.into();
            //       Self::transfer_tokens(
            //             sell.payment_token(),
            //             buy.maker(),
            //             &buy.fee_recipient,
            //             maker_relayer_fee,
            //         )?;
            //     }

            //     if buy.taker_relayer_fee > Zero::zero() {
            //        let taker_relayer_fee = buy.taker_relayer_fee * price / INVERSE_BASIS_POINT.into();
            //       Self::transfer_tokens(
            //             sell.payment_token(),
            //             sell.maker(),
            //             &buy.fee_recipient,
            //             taker_relayer_fee,
            //         )?;
            //     }

            //     if buy.maker_protocol_fee > Zero::zero() {
            //        let maker_protocol_fee = buy.maker_protocol_fee * price / INVERSE_BASIS_POINT.into();
            //       Self::transfer_tokens(
            //             sell.payment_token(),
            //             buy.maker(),
            //             &originprotocol_fee_recipient,
            //             maker_protocol_fee,
            //         )?;
            //     }

            //     if buy.taker_protocol_fee > Zero::zero() {
            //         let taker_protocol_fee = buy.taker_protocol_fee * price / INVERSE_BASIS_POINT.into();
            //       Self::transfer_tokens(
            //             &sell.payment_token,
            //             &sell.maker,
            //             &originprotocol_fee_recipient,
            //             taker_protocol_fee,
            //         )?;
            //     }

            // } else {
            //     // Charge maker fee to buyer.
            //   Self::charge_protocol_fee(&buy.maker, &buy.fee_recipient, buy.maker_relayer_fee)?;

            //     // Charge taker fee to seller.
            //   Self::charge_protocol_fee(&sell.maker, &buy.fee_recipient, buy.taker_relayer_fee)?;
            // }
        }

        if sell.payment_token == ContractSelf::<T>::get() {
            // Special-case Ether, order must be matched by buyer.
            ensure!(
                msg_value >= required_amount,
                Error::<T>::ValueLessThanRequiredAmount
            );
            // sell.maker.transfer(receive_amount);
            Self::transfer_tokens(
                &ContractSelf::<T>::get(),
                &ContractSelf::<T>::get(),
                &sell.maker,
                receive_amount,
            )?;
            // Allow overshoot for variable-price auctions, refund difference.
            let diff: BalanceOf<T> = msg_value - required_amount;
            if diff > Zero::zero() {
                // buy.maker.transfer(diff);
                Self::transfer_tokens(
                    &ContractSelf::<T>::get(),
                    &ContractSelf::<T>::get(),
                    buy.maker(),
                    diff,
                )?;
            }
        }

        // This contract should never hold Ether, however, we cannot assert this, since it is impossible to prevent anyone from sending Ether e.g. with selfdestruct.

        Ok(price)
    }

    //
    //#dev Execute all ERC20 token / Ether transfers associated with an order match (fees and buyer => transfer:seller)
    //#param buy Buy-side order
    //#param sell Sell-side order
    //
    pub fn execute_funds_transfer_sell_side(
        buy: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
        sell: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
        price: &BalanceOf<T>,
        receive_amount: &mut BalanceOf<T>,
        required_amount: &mut BalanceOf<T>,
    ) -> Result<BalanceOf<T>, Error<T>> {
        let originprotocol_fee_recipient = ProtocolFeeRecipient::<T>::get();

        // Determine maker/taker and charge fees accordingly.
        // Sell-side order is maker.

        // Assert taker fee is less than or equal to maximum fee specified by buyer.
        ensure!(
            sell.taker_relayer_fee <= buy.taker_relayer_fee,
            Error::<T>::SellTakerRelayerFeeGreaterThanBuyTakerRelayerFee
        );

        if sell.fee_method == FeeMethod::SplitFee {
            // Assert taker fee is less than or equal to maximum fee specified by buyer.
            ensure!(
                sell.taker_protocol_fee <= buy.taker_protocol_fee,
                Error::<T>::SellTakerProtocolFeeGreaterThanBuyTakerProtocolFee
            );

            // Maker fees are deducted from the token amount that the maker receives. Taker fees are extra tokens that must be paid by the taker.

            Self::transfer_tokens_fee_sell(
                sell.payment_token(),
                sell.maker(),
                &sell.fee_recipient,
                sell.maker_relayer_fee,
                price,
                receive_amount,
                true,
            )?;

            Self::transfer_tokens_fee_sell(
                sell.payment_token(),
                buy.maker(),
                &sell.fee_recipient,
                sell.taker_relayer_fee,
                price,
                required_amount,
                false,
            )?;

            Self::transfer_tokens_fee_sell(
                sell.payment_token(),
                sell.maker(),
                &originprotocol_fee_recipient,
                sell.maker_protocol_fee,
                price,
                receive_amount,
                true,
            )?;

            Self::transfer_tokens_fee_sell(
                sell.payment_token(),
                buy.maker(),
                &originprotocol_fee_recipient,
                sell.taker_protocol_fee,
                price,
                required_amount,
                false,
            )?;
        } else {
            // Charge maker fee to seller.
            Self::charge_protocol_fee(&sell.maker, &sell.fee_recipient, sell.maker_relayer_fee)?;

            // Charge taker fee to buyer.
            Self::charge_protocol_fee(&buy.maker, &sell.fee_recipient, sell.taker_relayer_fee)?;
        }

        // This contract should never hold Ether, however, we cannot assert this, since it is impossible to prevent anyone from sending Ether e.g. with selfdestruct.

        Ok(*price)
    }

    //
    //#dev Execute all ERC20 token / Ether transfers associated with an order match (fees and buyer => transfer:seller)
    //#param buy Buy-side order
    //#param sell Sell-side order
    //
    pub fn execute_funds_transfer_buy_side(
        buy: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
        sell: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
        price: &BalanceOf<T>,
    ) -> Result<BalanceOf<T>, Error<T>> {
        let originprotocol_fee_recipient = ProtocolFeeRecipient::<T>::get();

        // Determine maker/taker and charge fees accordingly.
        // Buy-side order is maker.

        // Assert taker fee is less than or equal to maximum fee specified by seller.
        ensure!(
            buy.taker_relayer_fee <= sell.taker_relayer_fee,
            Error::<T>::BuyTakerRelayerFeeGreaterThanSellTakerRelayerFee
        );

        if sell.fee_method == FeeMethod::SplitFee {
            // The Exchange does not escrow Ether, so direct Ether can only be used to with sell-side maker / buy-side taker orders.
            ensure!(
                sell.payment_token != ContractSelf::<T>::get(),
                Error::<T>::SellPaymentTokenEqualPaymentToken
            );

            // Assert taker fee is less than or equal to maximum fee specified by seller.
            ensure!(
                buy.taker_protocol_fee <= sell.taker_protocol_fee,
                Error::<T>::BuyTakerProtocolFeeGreaterThanSellTakerProtocolFee
            );

            Self::transfer_tokens_fee(
                sell.payment_token(),
                buy.maker(),
                &buy.fee_recipient,
                buy.maker_relayer_fee,
                price,
            )?;

            Self::transfer_tokens_fee(
                sell.payment_token(),
                sell.maker(),
                &buy.fee_recipient,
                buy.taker_relayer_fee,
                price,
            )?;

            Self::transfer_tokens_fee(
                sell.payment_token(),
                buy.maker(),
                &originprotocol_fee_recipient,
                buy.maker_protocol_fee,
                price,
            )?;

            Self::transfer_tokens_fee(
                &sell.payment_token,
                &sell.maker,
                &originprotocol_fee_recipient,
                buy.taker_protocol_fee,
                price,
            )?;
        } else {
            // Charge maker fee to buyer.
            Self::charge_protocol_fee(&buy.maker, &buy.fee_recipient, buy.maker_relayer_fee)?;

            // Charge taker fee to seller.
            Self::charge_protocol_fee(&sell.maker, &buy.fee_recipient, buy.taker_relayer_fee)?;
        }

        // This contract should never hold Ether, however, we cannot assert this, since it is impossible to prevent anyone from sending Ether e.g. with selfdestruct.

        Ok(*price)
    }

    //
    //#dev Return whether or not two orders can be matched with each other by basic parameters (does not check order signatures / calldata or perform calls:static)
    //#param buy Buy-side order
    //#param sell Sell-side order
    //#return Whether or not the two orders can be matched
    //
    pub fn orders_can_match(
        buy: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
        sell: &OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
    ) -> Result<bool, Error<T>> {
        //  Must be opposite-side.
        Ok((buy.side == Side::Buy && sell.side == Side::Sell) &&
            // Must use same fee method.
            (buy.fee_method == sell.fee_method) &&
            // Must use same payment token. 
            (buy.payment_token == sell.payment_token) &&
            // Must match maker/taker addresses. 
            (sell.taker == ContractSelf::<T>::get() || sell.taker == buy.maker) &&
            (buy.taker == ContractSelf::<T>::get() || buy.taker == sell.maker) &&
            // One must be maker and the other must be taker (no bool XOR Solidity:in). 
            ((sell.fee_recipient == ContractSelf::<T>::get() && buy.fee_recipient != ContractSelf::<T>::get()) || (sell.fee_recipient != ContractSelf::<T>::get() && buy.fee_recipient == ContractSelf::<T>::get())) &&
            // Must match target. 
            (buy.target == sell.target) &&
            // Must match how_to_call. 
            (buy.how_to_call == sell.how_to_call) &&
            // Buy-side order must be settleable. 
            <sale_kind_interface::Module<T>>::can_settle_order(buy.listing_time, buy.expiration_time)? &&
            // Sell-side order must be settleable. 
            <sale_kind_interface::Module<T>>::can_settle_order(sell.listing_time, sell.expiration_time)?)
    }

    //
    //#dev Atomically match two orders, ensuring validity of the match, and execute all associated state transitions. Protected against reentrancy by a contract-global lock.
    //#param buy Buy-side order
    //#param buy_sig Buy-side order signature
    //#param sell Sell-side order
    //#param sell_sig Sell-side order signature
    //
    pub fn atomic_match(
        msg_sender: T::AccountId,
        msg_value: BalanceOf<T>,
        buy: OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
        buy_sig: T::Signature,
        sell: OrderType<T::AccountId, T::Moment, BalanceOf<T>>,
        sell_sig: T::Signature,
        metadata: &[u8],
    ) -> Result<(), Error<T>> {
        //reentrancyGuard
        // CHECKS

        // Ensure buy order validity and calculate hash if necessary.
        let mut buy_hash: Vec<u8> = vec![];
        if buy.maker == msg_sender {
            ensure!(
                Self::validate_order_parameters(&buy)?,
                Error::<T>::OrderIdTooLong
            );
        } else {
            buy_hash = Self::require_valid_order(&buy, &buy_sig)?;
        }

        // Ensure sell order validity and calculate hash if necessary.
        let mut sell_hash: Vec<u8> = vec![];
        if sell.maker == msg_sender {
            ensure!(
                Self::validate_order_parameters(&sell)?,
                Error::<T>::OrderIdExists
            );
        } else {
            sell_hash = Self::require_valid_order(&sell, &sell_sig)?;
        }

        // Must be matchable.
        ensure!(
            Self::orders_can_match(&buy, &sell)?,
            Error::<T>::OrdersCannotMatch
        );

        // Target must exist (prevent malicious selfdestructs just prior to settlement:order).
        // BalanceOf<T> size;
        // AccountId target = sell.target;
        // assembly {
        //     size := extcodesize(target)
        // }
        // ensure!(size > 0, Error::<T>::OrderIdMissing);

        // Must match calldata after replacement, if specified.
        let mut buycalldata = buy.calldata.clone();
        let mut sellcalldata = sell.calldata.clone();
        if buy.replacement_pattern.len() > 0 {
            <utils::Module<T>>::guarded_array_replace(
                &mut buycalldata,
                &sell.calldata,
                &buy.replacement_pattern,
            )?;
        }
        if sell.replacement_pattern.len() > 0 {
            <utils::Module<T>>::guarded_array_replace(
                &mut sellcalldata,
                &buy.calldata,
                &sell.replacement_pattern,
            )?;
        }
        ensure!(
            <utils::Module<T>>::array_eq(&buycalldata, &sellcalldata)?,
            Error::<T>::OrderInvalidFieldName
        );

        // Mark previously signed or approved orders as finalized.
        let buymaker: T::AccountId = buy.maker.clone();
        if msg_sender != buymaker {
            CancelledOrFinalized::insert(buy_hash.clone(), true);
        }
        let sellmaker: T::AccountId = sell.maker.clone();
        if msg_sender != sellmaker {
            CancelledOrFinalized::insert(sell_hash.clone(), true);
        }

        debug::info!(
            "[product_tracking_ocw] Error reading product_tracking_ocw::last_proccessed_block."
        );

        // INTERACTIONS

        // Execute funds transfer and pay fees.
        let price: BalanceOf<T> = Self::execute_funds_transfer(msg_value, &buy, &sell)?;

        // Execute specified call through proxy.
        //TODO
        // ensure!(
        //     proxy.proxy(sell.target, sell.how_to_call, sell.calldata),
        //     Error::<T>::OrderIdMissing
        // );

        // Static calls are intentionally done after the effectful call so they can check resulting state.

        // Handle buy-side static call if specified.
        // if buy.static_target != ContractSelf::<T>::get() {
        //     ensure!(Self::staticCall(buy.static_target, sell.calldata, buy.static_extradata), Error::<T>::OrderIdMissing);
        // }

        // // Handle sell-side static call if specified.
        // if sell.static_target != ContractSelf::<T>::get() {
        //     ensure!(Self::staticCall(sell.static_target, sell.calldata, sell.static_extradata), Error::<T>::OrderIdMissing);
        // }

        // Log match event.
        Self::deposit_event(RawEvent::OrdersMatched(
            buy_hash.clone(),
            sell_hash.clone(),
            if sell.fee_recipient != ContractSelf::<T>::get() {
                sell.maker.clone()
            } else {
                buy.maker.clone()
            },
            if sell.fee_recipient != ContractSelf::<T>::get() {
                buy.maker.clone()
            } else {
                sell.maker.clone()
            },
            price,
            metadata.to_vec(),
        ));

        Ok(())
    }
}
