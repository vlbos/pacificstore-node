//! # Pacific Store - Wyvern Exchange pallet

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
// use sp_std::if_std;

use frame_support::{
    sp_runtime::{MultiSignature, RuntimeDebug},
    sp_std::prelude::*,
};

// General constraints to limit data size
// Note: these could also be passed as trait config parameters
pub const ORDER_ID_MAX_LENGTH: usize = 36;
pub const ORDER_FIELD_NAME_MAX_LENGTH: usize = 10;
pub const ORDER_FIELD_VALUE_MAX_LENGTH: usize = 20;
pub const ORDER_MAX_FIELDS: usize = 30;
// // Inverse basis point.

pub const INVERSE_BASIS_POINT: u32 = 10000;

// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

// Some way of identifying an account on the chain. We intentionally make it equivalent
// to the public key of our transaction signing scheme.
// pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
// Custom types
// pub type AccountId =Vec<u8>;
pub type OrderId = Vec<u8>;
pub type FieldName = Vec<u8>;
pub type FieldValue = Vec<u8>;

pub type Bytes = Vec<u8>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Balancex(u128);

impl From<u128> for Balancex {
    fn from(value: u128) -> Self {
        Balancex(value)
    }
}

impl Into<u128> for Balancex {
    fn into(self) -> u128 {
        self.0
    }
}

//sale kind interface
#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq,scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq,scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum SaleKind {
    FixedPrice,
    DutchAuction,
}

// // Fee method: protocol fee or split fee.
// enum FeeMethod { ProtocolFee, SplitFee }
#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq,scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum FeeMethod {
    ProtocolFee,
    SplitFee,
}

#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq,scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum HowToCall {
    Call,
    DelegateCall,
}

impl Default for Side {
    fn default() -> Self {
        Self::Buy
    }
}

impl Default for SaleKind {
    fn default() -> Self {
        Self::FixedPrice
    }
}
impl Default for FeeMethod {
    fn default() -> Self {
        Self::ProtocolFee
    }
}

impl Default for HowToCall {
    fn default() -> Self {
        Self::Call
    }
}

impl HowToCall {
    pub fn value(&self) -> u8 {
        match *self {
            HowToCall::Call => 0x0,
            HowToCall::DelegateCall => 0x1,
        }
    }
}

impl From<u8> for HowToCall {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => return HowToCall::Call,
            _ => return HowToCall::DelegateCall,
        };
    }
}

impl FeeMethod {
    pub fn value(&self) -> u8 {
        match *self {
            FeeMethod::ProtocolFee => 0x0,
            FeeMethod::SplitFee => 0x1,
        }
    }
}

impl From<u8> for FeeMethod {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => return FeeMethod::ProtocolFee,
            _ => return FeeMethod::SplitFee,
        };
    }
}

impl SaleKind {
    pub fn value(&self) -> u8 {
        match *self {
            SaleKind::FixedPrice => 0x0,
            SaleKind::DutchAuction => 0x1,
        }
    }
}

impl From<u8> for SaleKind {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => return SaleKind::FixedPrice,
            _ => return SaleKind::DutchAuction,
        };
    }
}

impl Side {
    pub fn value(&self) -> u8 {
        match *self {
            Side::Buy => 0x0,
            Side::Sell => 0x1,
        }
    }
}

impl From<u8> for Side {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => return Side::Buy,
            _ => return Side::Sell,
        };
    }
}

// OrderType contains master data (aka class-level) about a trade item.
// This data is typically created_date once by the order's manufacturer / supplier,
// to be shared with other network participants, and remains largely static.
// It can also be used for instance-level (lot) master data.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug,scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OrderType<AccountId, Moment, Balance> {
    // // An order on the exchange.
    pub index: u64,
    // Exchange AccountId, intended as a versioning mechanism.
    pub exchange: AccountId,
    // OrderType maker AccountId.
    pub maker: AccountId,
    // OrderType taker AccountId, if specified.
    pub taker: AccountId,
    // Maker relayer fee of the order, unused for taker order.
    pub maker_relayer_fee: Balance,
    // Taker relayer fee of the order, or maximum taker fee for a taker order.
    pub taker_relayer_fee: Balance,
    // Maker protocol fee of the order, unused for taker order.
    pub maker_protocol_fee: Balance,
    // Taker protocol fee of the order, or maximum taker fee for a taker order.
    pub taker_protocol_fee: Balance,
    // OrderType fee recipient or zero AccountId for taker order.
    pub fee_recipient: AccountId,
    // Fee method (protocol token or split fee).
    pub fee_method: FeeMethod,
    // Side (buy/sell).
    pub side: Side,
    // Kind of sale.
    pub sale_kind: SaleKind,
    // Target.
    pub target: AccountId,
    // Vec<u8>.
    pub how_to_call: HowToCall,
    // Calldata.
    pub calldata: Bytes,
    // Calldata replacement pattern, or an empty byte array for no replacement.
    pub replacement_pattern: Bytes,
    // Static call target, zero-AccountId for no static call.
    pub static_target: AccountId,
    // Static call extra data.
    pub static_extradata: Bytes,
    // Token used to pay for the order, or the zero-AccountId as a sentinel value for Dot.
    pub payment_token: AccountId,
    // Base price of the order (in paymentTokens).
    pub base_price: Balance,
    // Auction extra parameter - minimum bid increment for English auctions, 
    // starting/ending price difference.
    pub extra: Moment,
    // Listing timestamp.
    pub listing_time: Moment,
    // Expiration timestamp - 0 for no expiry.
    pub expiration_time: Moment,
    // OrderType salt, used to prevent duplicate hashes.
    pub salt: u64,
    pub created_date: Moment,
}

impl<AccountId, Moment, Balance> OrderType<AccountId, Moment, Balance>
where
    AccountId: Default,
    Moment: Default,
    Balance: Default,
{
    pub fn new(
        exchange: AccountId,
        maker: AccountId,
        taker: AccountId,
        maker_relayer_fee: Balance,
        taker_relayer_fee: Balance,
        maker_protocol_fee: Balance,
        taker_protocol_fee: Balance,
        fee_recipient: AccountId,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        target: AccountId,
        how_to_call: HowToCall,
        calldata: Bytes,
        replacement_pattern: Bytes,
        static_target: AccountId,
        static_extradata: Bytes,
        payment_token: AccountId,
        base_price: Balance,
        extra: Moment,
        listing_time: Moment,
        expiration_time: Moment,
        salt: u64,
    ) -> Self {
        Self {
            index: 0,
            exchange: exchange,
            maker: maker,
            taker: taker,
            maker_relayer_fee: maker_relayer_fee,
            taker_relayer_fee: taker_relayer_fee,
            maker_protocol_fee: maker_protocol_fee,
            taker_protocol_fee: taker_protocol_fee,
            fee_recipient: fee_recipient,
            fee_method: fee_method,
            side: side,
            sale_kind: sale_kind,
            target: target,
            how_to_call: how_to_call,
            calldata: calldata,
            replacement_pattern: replacement_pattern,
            static_target: static_target,
            static_extradata: static_extradata,
            payment_token: payment_token,
            base_price: base_price,
            extra: extra,
            listing_time: listing_time,
            expiration_time: expiration_time,
            salt: salt,
            created_date: Moment::default(),
        }
    }

    pub fn maker(&self) -> &AccountId {
        &self.maker
    }

    pub fn taker(&self) -> &AccountId {
        &self.taker
    }

    pub fn payment_token(&self) -> &AccountId {
        &self.payment_token
    }
}
