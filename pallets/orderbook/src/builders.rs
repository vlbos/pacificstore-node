//! # Pacific Store node - Orderbook pallet
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::sp_std::prelude::*;

pub use crate::types::*;

#[derive(Default)]
pub struct OrderBuilder<AccountId, Moment>
where
    AccountId: Default,
    Moment: Default,
{
    index: u64,
    order_id: OrderId,
    owner: AccountId,
    fields: Option<Vec<OrderField>>,
    created_date: Moment,
}

impl<AccountId, Moment> OrderBuilder<AccountId, Moment>
where
    AccountId: Default,
    Moment: Default,
{
    pub fn index_by(mut self, index: u64) -> Self {
        self.index = index;
        self
    }

    pub fn identified_by(mut self, order_id: OrderId) -> Self {
        self.order_id = order_id;
        self
    }

    pub fn owned_by(mut self, owner: AccountId) -> Self {
        self.owner = owner;
        self
    }

    pub fn with_fields(mut self, fields: Option<Vec<OrderField>>) -> Self {
        self.fields = fields;
        self
    }

    pub fn created_on(mut self, created_date: Moment) -> Self {
        self.created_date = created_date;
        self
    }

    pub fn build(self) -> OrderJSONType<AccountId, Moment> {
        OrderJSONType::<AccountId, Moment> {
            index: self.index,
            order_id: self.order_id,
            owner: self.owner,
            fields: self.fields,
            created_date: self.created_date,
        }
    }
}

// #[derive(Default)]
// pub struct OrderQueryBuilder<AccountId>
// where
//     AccountId: Default, 
// {
//     pub limit: Option<u64>,
//     pub offset: Option<u64>,
//     pub owner: Option<AccountId>,
//     pub token_ids: Option<Vec<TokenId>>,
//     pub params: Option<Vec<OrderField>>,
// }

// impl<AccountId> OrderQueryBuilder<AccountId>
// where
//     AccountId: Default,
// {
//     pub fn owned_by(mut self, owner: Option<AccountId>) -> Self {
//         self.owner = owner;
//         self
//     }

//     pub fn with_limit(mut self, limit: Option<u64>) -> Self {
//         self.limit = limit;
//         self
//     }

//     pub fn with_offset(mut self, offset: Option<u64>) -> Self {
//         self.offset = offset;
//         self
//     }
//     pub fn with_params(mut self, params: Option<Vec<OrderField>>) -> Self {
//         self.params = params;
//         self
//     }

//     pub fn with_token_ids(mut self, token_ids: Option<Vec<TokenId>>) -> Self {
//         self.token_ids = token_ids;
//         self
//     }

//     pub fn build(self) -> OrderQuery<AccountId> {
//         OrderQuery::<AccountId> {
//             limit: self.limit,
//             offset: self.offset,
//             owner: self.owner,
//             token_ids: self.token_ids,
//             params: self.params,
//         }
//     }
// }
// #[derive(Default)]
// pub struct AssetQueryBuilder<AccountId> 
// where
//     AccountId: Default,
// {
//     pub owner: Option<AccountId>,
//     pub asset_contract_address: Option<Vec<u8>>,
//     pub token_ids: Option<Vec<TokenId>>,
//     pub search: Option<Vec<u8>>,
//     pub order_by: Option<Vec<u8>>,
//     pub order_direction: Option<Vec<u8>>,
//     pub limit: Option<u64>,
//     pub offset: Option<u64>,
// }


// impl<AccountId> AssetQueryBuilder<AccountId>
// where
//     AccountId: Default,
// {
//     pub fn owned_by(mut self, owner: Option<AccountId>) -> Self {
//         self.owner = owner;
//         self
//     }

//     pub fn with_limit(mut self, limit: Option<u64>) -> Self {
//         self.limit = limit;
//         self
//     }

//     pub fn with_offset(mut self, offset: Option<u64>) -> Self {
//         self.offset = offset;
//         self
//     }
//     pub fn with_token_ids(mut self, token_ids: Option<Vec<TokenId>>) -> Self {
//         self.token_ids = token_ids;
//         self
//     }
//     pub fn with_search(mut self, search: Option<Vec<u8>>) -> Self {
//         self.search = search;
//         self
//     }

//    pub fn with_order_by(mut self, order_by: Option<Vec<u8>>) -> Self {
//         self.order_by = order_by;
//         self
//     }

//    pub fn with_order_direction(mut self, order_direction: Option<Vec<u8>>) -> Self {
//         self.order_direction = order_direction;
//         self
//     }

//     pub fn with_asset_contract_address(mut self, asset_contract_address: Option<Vec<u8>>) -> Self {
//         self.asset_contract_address = asset_contract_address;
//         self
//     }

//     pub fn build(self) -> AssetQuery<AccountId> {
//         AssetQuery::<AccountId> {
//             limit: self.limit,
//             offset: self.offset,
//             owner: self.owner,
//             token_ids: self.token_ids,
//             asset_contract_address: self.asset_contract_address,
//             search: self.search,
//             order_by: self.order_by,
//             order_direction:self.order_direction,
//         }
//     }
// }

