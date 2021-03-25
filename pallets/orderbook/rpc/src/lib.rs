//! RPC interface for the transaction payment module.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
use orderbook_runtime_api::OrderbookApi as OrderbookRuntimeApi;
use orderbook::{OrderQuery,OrderJSONType,AssetQuery,JSONType,OrderField};
use codec::Codec;
use codec::{Decode, Encode};
// #[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[rpc] 
pub trait OrderbookApi<BlockHash,AccountId,Moment> {
	#[rpc(name = "orderbook_getOrder")]
    fn get_order(&self,
        order_query: Option<OrderQueryJSON<AccountId>>,  at: Option<BlockHash>
    ) -> Result<Option<OrderJSONType<AccountId, Moment>>>;
	#[rpc(name = "orderbook_getOrders")]
    fn get_orders(&self,
        order_query: Option<OrderQueryJSON<AccountId>>, page: Option<u64>, at: Option<BlockHash>
    ) -> Result<Option<Vec<OrderJSONType<AccountId, Moment>>>>;
	#[rpc(name = "orderbook_getAsset")]
    fn get_asset(&self,
        token_address: String,token_id: String, at: Option<BlockHash>
    ) -> Result<Option<JSONType>>;
    #[rpc(name = "orderbook_getAssets")]
    fn get_assets(&self,
        asset_query: Option<AssetQueryJSON<AccountId>>, page: Option<u64>, at: Option<BlockHash>
    ) -> Result<Option<Vec<JSONType>>>;
}

#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OrderQueryJSON<AccountId> {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub owner: Option<AccountId>,
    pub token_ids: Option<Vec<String>>,
    pub params: Option<Vec<QueryParameter>>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QueryParameter {
    // Name of the order field e.g. desc or description
    pub name: String,
    // Value of the order field e.g. tokenjson
    pub value: String,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AssetQueryJSON<AccountId> {
    pub owner: Option<AccountId>,
    pub asset_contract_address: Option<String>,
    pub token_ids: Option<Vec<String>>,
    pub search: Option<String>,
    pub order_by: Option<String>,
    pub order_direction: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

pub fn convert_json_to_assetquery<AccountId>(
    asset_query_json: Option<AssetQueryJSON<AccountId>>,
) -> Option<AssetQuery<AccountId>> {

    if let Some(json) = asset_query_json{
        let mut asset_query = AssetQuery::<AccountId>{
                    limit: json.limit,
                    offset: json.offset,
                    owner: json.owner,
                    token_ids: None,
                    asset_contract_address: None,
                    search: None,
                    order_by: None,
                    order_direction:None,
        };

        if let Some(token_ids) = json.token_ids{
        asset_query.token_ids  = Some(token_ids.into_iter().map(|t|from_hex(t)).collect());      
        }
        if let Some(asset_contract_address) = json.asset_contract_address{
        asset_query.asset_contract_address  =  Some(from_hex(asset_contract_address));
        }
        if let Some(search) = json.search{
        asset_query.search  =  Some(from_hex(search));
        }
        if let Some(order_by) = json.order_by{
        asset_query.order_by =Some(from_hex(order_by)) ;
        }
        if let Some(order_direction) = json.order_direction{
        asset_query.order_direction =Some(from_hex(order_direction)) ;
        }
        return Some(asset_query);
         }               

        None
}

pub fn convert_json_to_orderquery<AccountId>(
    order_query_json: Option<OrderQueryJSON<AccountId>>
) -> Option<OrderQuery<AccountId>> {
    if let Some(json) = order_query_json{
        let  mut order_query = OrderQuery::<AccountId> {
            limit: json.limit,
            offset: json.offset,
            owner: json.owner,
            token_ids: None,
            params: None,
        };
        if let Some(token_ids) = json.token_ids{
        order_query.token_ids = Some(token_ids.into_iter().map(|t|from_hex(t)).collect());
        }
        if let Some(params) = json.params{
        order_query.params  =  Some(params.into_iter().map(|t|OrderField::new(&from_hex(t.name),&from_hex(t.value))).collect());
        }
        return Some(order_query);
   
        }

    None
}

pub fn from_hex(str: String) -> Vec<u8> {
    if let Some(s)= str.strip_prefix("0x"){
         return decode_hex(&s);
    }
    str.into_bytes()
}
pub fn decode_hex(s: &str) -> Vec<u8> {
    let len = if s.len()%2!=0{s.len()-1}else{s.len()};

    if 0==len{
      return Vec::<u8>::new();
    }

    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

/// A struct that implements the `OrderbookApi`.
pub struct Orderbook<C,  M> {
	// If you have more generics, no need to Orderbook<C, M, N, P, ...>
	// just use a tuple like Orderbook<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C,  M> Orderbook<C,  M> {
	/// Create new `Orderbook` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_marker: Default::default(),
		}
	}
}

impl<C, Block,AccountId,Moment> OrderbookApi<<Block as BlockT>::Hash,AccountId,Moment> for Orderbook<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: OrderbookRuntimeApi<Block,AccountId, Moment>,
    AccountId:Codec,
    Moment:Codec
    {
        fn get_order(&self,
            order_query: Option<OrderQueryJSON<AccountId>>,  at:Option<<Block as BlockT>::Hash>
        ) -> Result<Option<OrderJSONType<AccountId, Moment>>>{
            let api = self.client.runtime_api();
            let at = BlockId::hash(at.unwrap_or_else(||
                // If the block hash is not supplied assume the best block.
                self.client.info().best_hash));

            let runtime_api_result = api.get_order(&at,convert_json_to_orderquery::<AccountId>(order_query));
            runtime_api_result.map_err(|e| RpcError {
                code: ErrorCode::ServerError(9876), // No real reason for this value
                message: "Something wrong".into(),
                data: Some(format!("{:?}", e).into()),
            })
        }

        fn get_orders(&self,
            order_query: Option<OrderQueryJSON<AccountId>>, page: Option<u64>, at:Option<<Block as BlockT>::Hash>
        ) -> Result<Option<Vec<OrderJSONType<AccountId, Moment>>>>{
            let api = self.client.runtime_api();
            let at = BlockId::hash(at.unwrap_or_else(||
                // If the block hash is not supplied assume the best block.
                self.client.info().best_hash));

            let runtime_api_result = api.get_orders(&at,convert_json_to_orderquery::<AccountId>(order_query),page);
            runtime_api_result.map_err(|e| RpcError {
                code: ErrorCode::ServerError(9876), // No real reason for this value
                message: "Something wrong".into(),
                data: Some(format!("{:?}", e).into()),
            })
        }

        fn get_asset(&self,
            token_address: String,token_id: String, at:Option<<Block as BlockT>::Hash>
        ) -> Result<Option<JSONType>>{
            let api = self.client.runtime_api();
            let at = BlockId::hash(at.unwrap_or_else(||
                // If the block hash is not supplied assume the best block.
                self.client.info().best_hash));

            let runtime_api_result = api.get_asset(&at,Some(from_hex(token_address.clone())),Some(from_hex(token_id.clone())));
            runtime_api_result.map_err(|e| RpcError {
                code: ErrorCode::ServerError(9876), // No real reason for this value
                message: "Something wrong".into(),
                data: Some(format!("{:?}", e).into()),
            })
        }
        fn get_assets(&self,
            asset_query: Option<AssetQueryJSON<AccountId>>, page: Option<u64>, at:Option<<Block as BlockT>::Hash>
        ) -> Result<Option<Vec<JSONType>>>{
            let api = self.client.runtime_api();
            let at = BlockId::hash(at.unwrap_or_else(||
                // If the block hash is not supplied assume the best block.
                self.client.info().best_hash));

            let runtime_api_result = api.get_assets(&at,convert_json_to_assetquery::<AccountId>(asset_query),page);
            runtime_api_result.map_err(|e| RpcError {
                code: ErrorCode::ServerError(9876), // No real reason for this value
                message: "Something wrong".into(),
                data: Some(format!("{:?}", e).into()),
            })
        }
    }



