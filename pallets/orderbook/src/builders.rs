//! # Pacific Store node - Orderbook pallet

use frame_support::sp_std::prelude::*;

pub use crate::types::*;

#[derive(Default)]
pub struct OrderBuilder<AccountId, Moment>
where
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
    Moment: Default,
{
    pub fn new(owner: AccountId)->Self{
        Self{  index: 0,
    order_id: Vec::new(),
    owner,
    fields:None,
    created_date: Moment::default(),}
    }
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
