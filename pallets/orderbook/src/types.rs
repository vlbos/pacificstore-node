//! # Substrate Enterprise Sample - Order Post example pallet
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::{ DispatchResult},
    ensure,
    sp_runtime::RuntimeDebug,
    sp_std::collections::btree_set::BTreeSet,
    sp_std::prelude::*,
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
// traits::EnsureOrigin,
use frame_system::{self as system, ensure_signed};


// General constraints to limit data size
// Note: these could also be passed as trait config parameters
pub const ORDER_ID_MAX_LENGTH: usize = 36;
pub const ORDER_FIELD_NAME_MAX_LENGTH: usize = 200;
pub const ORDER_FIELD_VALUE_MAX_LENGTH: usize = 400; 
pub const ORDER_MAX_FIELDS: usize = 54;

// Custom types
pub type OrderId = Vec<u8>;
pub type FieldName = Vec<u8>;
pub type FieldValue = Vec<u8>;

// Order contains master data (aka class-level) about a trade item.
// This data is typically registered once by the order's manufacturer / supplier,
// to be shared with other network participants, and remains largely static.
// It can also be used for instance-level (lot) master data.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OrderJSONType<AccountId, Moment> {
    pub index: u64,
    // The order ID would typically be a GS1 GTIN (Global Trade Item Number),
    // or ASIN (Amazon Standard Identification Number), or similar,
    // a numeric or alpha-numeric code with a well-defined data structure.
    pub order_id: OrderId,
    // This is account that represents the owner of this order, as in
    // the manufacturer or supplier providing this order within the value chain.
    pub owner: AccountId,
    // This a series of fields describing the order.
    // Typically, there would at least be a textual description, and SKU(Stock-keeping unit).
    // It could also contain instance / lot master data e.g. expiration, weight, harvest date.
    pub fields: Option<Vec<OrderField>>,
    // Timestamp (approximate) at which the Order was registered on-chain.
    pub registered: Moment,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OrderQuery<AccountId> {
   pub limit: Option<u64>,
   pub offset: Option<u64>,
   pub owner: Option<AccountId>,
   pub token_ids: Option<Vec<OrderId>>,
   pub params: Option<Vec<OrderField>>,
}

//   owner?: string,
//   sale_kind?: SaleKind,
//   asset_contract_address?: string,
//   payment_token_address?: string,
//   is_english?: boolean
//   is_expired?: boolean
//   bundled?: boolean
//   include_invalid?: boolean
//   token_id?: number | string
//   token_ids?: Array<number | string>
//   // This means listing_time > value in seconds
//   listed_after?: number | string
//   // This means listing_time <= value in seconds
//   listed_before?: number | string
//   limit?: number
//   offset?: number

// Contains a name-value pair for a order fielderty e.g. description: Ingredient ABC
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OrderField {
    // Name of the order fielderty e.g. desc or description
    pub name: FieldName,
    // Value of the order fielderty e.g. Ingredient ABC
    pub value: FieldValue,
}

impl OrderField {
    pub fn new(name: &[u8], value: &[u8]) -> Self {
        Self {
            name: name.to_vec(),
            value: value.to_vec(),
        }
    }

    pub fn name(&self) -> &[u8] {
        self.name.as_ref()
    }

    pub fn value(&self) -> &[u8] {
        self.value.as_ref()
    }
}
