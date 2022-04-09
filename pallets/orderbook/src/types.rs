//! # Pacific Store node - Orderbook pallet

use codec::{Decode, Encode};
use frame_support::{sp_runtime::RuntimeDebug, sp_std::prelude::*};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

// General constraints to limit data size
// Note: these could also be passed as trait config parameters
pub const ORDER_ID_MAX_LENGTH: usize = 36;
pub const ORDER_FIELD_NAME_MAX_LENGTH: usize = 200;
pub const ORDER_FIELD_VALUE_MAX_LENGTH: usize = 400;
pub const ORDER_MAX_FIELDS: usize = 54;
pub const ORDER_MAX_PARAMS: usize = 54;
pub const MAX_TOKEN_IDS: usize = 54;

// Custom types
pub type OrderId = Vec<u8>;
pub type TokenId = Vec<u8>;
pub type FieldName = Vec<u8>;
pub type FieldValue = Vec<u8>;

// Order contains master data (aka class-level) about a trade item.
// This data is typically created_date once by the order's manufacturer / supplier,
// to be shared with other network participants, and remains largely static.
// It can also be used for instance-level (lot) master data.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OrderJSONType<AccountId, Moment> {
    pub index: u64,
    // a numeric or alpha-numeric code with a well-defined data structure.
    pub order_id: OrderId,
    // This is account that represents the owner of this order, as in
    // the manufacturer or supplier providing this order within the value chain.
    pub owner: AccountId,
    // This a series of fields describing the order.
    // Typically, there would at least be a textual description.
    pub fields: Option<Vec<OrderField>>,
    // Timestamp (approximate) at which the Order was created_date on-chain.
    pub created_date: Moment,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OrderQuery<AccountId> {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub owner: Option<AccountId>,
    pub token_ids: Option<Vec<TokenId>>,
    pub params: Option<Vec<OrderField>>,
}

// Contains a name-value pair for a order fielderty e.g. description: Ingredient ABC
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
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

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct JSONField {
    // Name of the order field e.g. desc or description
    pub name: FieldName,
    // Value of the order field e.g. tokenjson
    pub json: Option<Vec<OrderField>>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AssetQuery<AccountId> {
    pub owner: Option<AccountId>,
    pub asset_contract_address: Option<Vec<u8>>,
    pub token_ids: Option<Vec<TokenId>>,
    pub search: Option<Vec<u8>>,
    pub order_by: Option<Vec<u8>>,
    pub order_direction: Option<Vec<u8>>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct JSONType {
    pub fields: Option<Vec<OrderField>>,
    pub jsons: Option<Vec<JSONField>>,
}

pub fn convert_assetquery_to_orderquery<AccountId>(
    asset_query: Option<AssetQuery<AccountId>>,
) -> Option<OrderQuery<AccountId>> {
    if let Some(asset_query) = asset_query {
        let mut token_address = Vec::new();
        if let Some(asset_contract_address) = asset_query.asset_contract_address {
            token_address = asset_contract_address;
        }

        return Some(OrderQuery::<AccountId> {
            limit: asset_query.limit,
            offset: asset_query.offset,
            owner: asset_query.owner,
            token_ids: asset_query.token_ids,
            params: Some(vec![OrderField::new(
                b"metadata.asset.address",
                &token_address,
            )]),
        });
    }
    None
}

pub fn convert_orderjsontype_to_jsontype<AccountId, Moment>(
    order_json: OrderJSONType<AccountId, Moment>,
) -> JSONType {
    JSONType {
        fields: order_json.fields,
        jsons: None,
    }
}
