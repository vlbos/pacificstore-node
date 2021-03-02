//! # Substrate Enterprise Sample - Order Post example pallet
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
    sp_runtime::RuntimeDebug, sp_std::collections::btree_set::BTreeSet, sp_std::prelude::*,
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
// traits::EnsureOrigin,
use frame_system::{self as system, ensure_signed};

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
    registered: Moment,
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

    pub fn registered_on(mut self, registered: Moment) -> Self {
        self.registered = registered;
        self
    }

    pub fn build(self) -> OrderJSONType<AccountId, Moment> {
        OrderJSONType::<AccountId, Moment> {
            index: self.index,
            order_id: self.order_id,
            owner: self.owner,
            fields: self.fields,
            registered: self.registered,
        }
    }
}
