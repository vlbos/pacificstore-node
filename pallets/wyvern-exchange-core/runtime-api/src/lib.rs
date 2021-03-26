#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]
use codec::Codec;
use sp_std::vec::Vec;
use wyvern_exchange_core::OrderType;
// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
    pub trait WyvernExchangeCoreApi<AccountId,Balance, Moment,Signature> where
        AccountId: Codec,
        Balance: Codec,
        Moment: Codec,
        Signature:Codec,
        {
            fn hash_order(
             order: OrderType<AccountId, Moment, Balance>,
            ) -> Vec<u8>;

            fn hash_to_sign(
                order: OrderType<AccountId, Moment, Balance>,
            ) -> Vec<u8>;
            fn validate_order_parameters(
                order: OrderType<AccountId, Moment, Balance>,
            ) -> bool;
            fn validate_order(
                hash:  Vec<u8>,
                order: OrderType<AccountId, Moment, Balance>,
                sig: Vec<u8>,
            ) -> bool;
            fn require_valid_order(
                order: OrderType<AccountId, Moment, Balance>,
                sig: Vec<u8>,
            ) -> Vec<u8>;
            fn calculate_current_price(
                order: OrderType<AccountId, Moment, Balance>,
            ) -> Balance;
            fn orders_can_match(
                buy: OrderType<AccountId, Moment, Balance>,
                sell: OrderType<AccountId, Moment, Balance>,
            ) -> bool;
            fn calculate_match_price(
                buy: OrderType<AccountId, Moment, Balance>,
                sell: OrderType<AccountId, Moment, Balance>,
            ) -> Balance;
    }
}
