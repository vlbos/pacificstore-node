//! RPC interface for the transaction payment module.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
// use sp_core::Bytes;
use codec::Codec;
use std::sync::Arc;
use wyvern_exchange_core::{OrderType,FeeMethod, HowToCall, SaleKind, Side};
use wyvern_exchange_core_runtime_api::WyvernExchangeCoreApi as WyvernExchangeCoreRuntimeApi;

#[rpc]
pub trait WyvernExchangeCoreApi<BlockHash, AccountId, Balance, Moment, Signature> {
    #[rpc(name = "wyvernExchangeCore_hashOrder")]
    fn hash_order(
        &self,
        order: OrderType<AccountId, Moment, Balance>,
        at: Option<BlockHash>,
    ) -> Result<Vec<u8>>;

    #[rpc(name = "wyvernExchangeCore_hashToSign")]
    fn hash_to_sign(
        &self,
        order: OrderType<AccountId, Moment, Balance>,
        at: Option<BlockHash>,
    ) -> Result<Vec<u8>>;

    #[rpc(name = "wyvernExchangeCore_validateOrderParameters")]
    fn validate_order_parameters(
        &self,
        order: OrderType<AccountId, Moment, Balance>,
        at: Option<BlockHash>,
    ) -> Result<bool>;

    #[rpc(name = "wyvernExchangeCore_validateOrder")]
    fn validate_order(
        &self,
        hash:String,
        order: OrderType<AccountId, Moment, Balance>,
        sig: Signature,
        at: Option<BlockHash>,
    ) -> Result<bool>;

    #[rpc(name = "wyvernExchangeCore_requireValidOrder")]
    fn require_valid_order(
        &self,
        order: OrderType<AccountId, Moment, Balance>,
        sig: Signature,
        at: Option<BlockHash>,
    ) -> Result<Vec<u8>>;

    #[rpc(name = "wyvernExchangeCore_calculateCurrentPrice")]
    fn calculate_current_price(
        &self,
        order: OrderType<AccountId, Moment, Balance>,
        at: Option<BlockHash>,
    ) -> Result<Balance>;

    #[rpc(name = "wyvernExchangeCore_ordersCanMatch")]
    fn orders_can_match(
        &self,
        buy: OrderType<AccountId, Moment, Balance>,
        sell: OrderType<AccountId, Moment, Balance>,
        at: Option<BlockHash>,
    ) -> Result<bool>;

    #[rpc(name = "wyvernExchangeCore_calculateMatchPrice")]
    fn calculate_match_price(
        &self,
        buy: OrderType<AccountId, Moment, Balance>,
        sell: OrderType<AccountId, Moment, Balance>,
        at: Option<BlockHash>,
    ) -> Result<Balance>;
}

/// A struct that implements the `WyvernExchangeCoreApi`.
pub struct WyvernExchangeCore<C, M> {
    // If you have more generics, no need to WyvernExchangeCore<C, M, N, P, ...>
    // just use a tuple like WyvernExchangeCore<C, (M, N, P, ...)>
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}

impl<C, M> WyvernExchangeCore<C, M> {
    /// Create new `WyvernExchangeCore` instance with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId, Balance, Moment, Signature>
    WyvernExchangeCoreApi<<Block as BlockT>::Hash, AccountId, Balance, Moment, Signature>
    for WyvernExchangeCore<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: WyvernExchangeCoreRuntimeApi<Block, AccountId, Balance, Moment, Signature>,
    AccountId: Codec,
    Balance: Codec,
    Moment: Codec,
    Signature: Codec,
{
      fn hash_order(
        &self,
        order: OrderType<AccountId, Moment, Balance>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Vec<u8>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.hash_order(
            &at,
            order,
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn hash_to_sign(
        &self,
        order: OrderType<AccountId, Moment, Balance>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Vec<u8>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.hash_to_sign(
            &at,
            order,
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
    fn validate_order_parameters(
        &self,
        order: OrderType<AccountId, Moment, Balance>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<bool> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.validate_order_parameters(
            &at,
            order,
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
    fn validate_order(
        &self,
        hash:String,
        order: OrderType<AccountId, Moment, Balance>,
        sig: Signature,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<bool> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.validate_order(
            &at,
            hash.clone().into_bytes(),
            order,
            sig,
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn require_valid_order(
        &self,
        order: OrderType<AccountId, Moment, Balance>,
        sig: Signature,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Vec<u8>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.require_valid_order(
            &at,
            order,
            sig,
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn calculate_current_price(
        &self,
        order: OrderType<AccountId, Moment, Balance>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Balance> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.calculate_current_price(
            &at,
            order,
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
    fn orders_can_match(
        &self,
        buy: OrderType<AccountId, Moment, Balance>,
        sell: OrderType<AccountId, Moment, Balance>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<bool> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.orders_can_match(
            &at,
            buy,
            sell,
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
    fn calculate_match_price(
        &self,
        buy: OrderType<AccountId, Moment, Balance>,
        sell: OrderType<AccountId, Moment, Balance>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Balance> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.calculate_match_price(
            &at,
            buy,
            sell,
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
}
