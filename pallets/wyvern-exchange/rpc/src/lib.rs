//! RPC interface for the transaction payment module.

use codec::Codec;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::bytes;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
use wyvern_exchange::{FeeMethod, HowToCall, SaleKind, Side};
use wyvern_exchange_runtime_api::WyvernExchangeApi as WyvernExchangeRuntimeApi;

#[rpc]
pub trait WyvernExchangeApi<BlockHash, AccountId, Balance, Moment, Signature> {
    #[rpc(name = "wyvernExchange_calculateFinalPriceEx")]
    fn calculate_final_price_ex(
        &self,
        side: Side,
        sale_kind: SaleKind,
        base_price: u64,
        extra: Moment,
        listing_time: Moment,
        expiration_time: Moment,
        at: Option<BlockHash>,
    ) -> Result<u64>;

    #[rpc(name = "wyvernExchange_hashOrderEx")]
    fn hash_order_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
        at: Option<BlockHash>,
    ) -> Result<Vec<u8>>;

    #[rpc(name = "wyvernExchange_hashToSignEx")]
    fn hash_to_sign_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
        at: Option<BlockHash>,
    ) -> Result<Vec<u8>>;

    #[rpc(name = "wyvernExchange_validateOrderParametersEx")]
    fn validate_order_parameters_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
        at: Option<BlockHash>,
    ) -> Result<bool>;

    #[rpc(name = "wyvernExchange_validateOrderEx")]
    fn validate_order_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
        sig: String,
        at: Option<BlockHash>,
    ) -> Result<bool>;

    #[rpc(name = "wyvernExchange_requireValidOrderEx")]
    fn require_valid_order_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
        sig: String,
        at: Option<BlockHash>,
    ) -> Result<Vec<u8>>;

    #[rpc(name = "wyvernExchange_calculateCurrentPriceEx")]
    fn calculate_current_price_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
        at: Option<BlockHash>,
    ) -> Result<u64>;

    #[rpc(name = "wyvernExchange_ordersCanMatchEx")]
    fn orders_can_match_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_methods_sides_kinds_how_to_calls: String,
        calldata_buy: String,
        calldata_sell: String,
        replacement_pattern_buy: String,
        replacement_pattern_sell: String,
        static_extradata_buy: String,
        static_extradata_sell: String,
        at: Option<BlockHash>,
    ) -> Result<bool>;

    #[rpc(name = "wyvernExchange_calculateMatchPriceEx")]
    fn calculate_match_price_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_methods_sides_kinds_how_to_calls: String,
        calldata_buy: String,
        calldata_sell: String,
        replacement_pattern_buy: String,
        replacement_pattern_sell: String,
        static_extradata_buy: String,
        static_extradata_sell: String,
        at: Option<BlockHash>,
    ) -> Result<u64>;
}

/// A struct that implements the `WyvernExchangeApi`.
pub struct WyvernExchange<C, M> {
    // If you have more generics, no need to WyvernExchange<C, M, N, P, ...>
    // just use a tuple like WyvernExchange<C, (M, N, P, ...)>
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}

impl<C, M> WyvernExchange<C, M> {
    /// Create new `WyvernExchange` instance with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId, Balance, Moment, Signature>
    WyvernExchangeApi<<Block as BlockT>::Hash, AccountId, Balance, Moment, Signature>
    for WyvernExchange<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: WyvernExchangeRuntimeApi<Block, AccountId, Balance, Moment, Signature>,
    AccountId: Codec,
    Balance: Codec,
    Moment: Codec,
    Signature: Codec,
{
    fn calculate_final_price_ex(
        &self,
        side: Side,
        sale_kind: SaleKind,
        base_price: u64,
        extra: Moment,
        listing_time: Moment,
        expiration_time: Moment,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<u64> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.calculate_final_price_ex(
            &at,
            side,
            sale_kind,
            base_price,
            extra,
            listing_time,
            expiration_time,
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
    fn hash_order_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Vec<u8>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.hash_order_ex(
            &at,
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            from_hex(calldata),
            from_hex(replacement_pattern),
            from_hex(static_extradata),
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn hash_to_sign_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Vec<u8>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
		// If the block hash is not supplied assume the best block.
		self.client.info().best_hash));
        let runtime_api_result = api.hash_to_sign_ex(
            &at,
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            from_hex(calldata),
            from_hex(replacement_pattern),
            from_hex(static_extradata),
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
    fn validate_order_parameters_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<bool> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.validate_order_parameters_ex(
            &at,
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            from_hex(calldata),
            from_hex(replacement_pattern),
            from_hex(static_extradata),
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
    fn validate_order_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
        sig: String,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<bool> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));
        let runtime_api_result = api.validate_order_ex(
            &at,
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            from_hex(calldata),
            from_hex(replacement_pattern),
            from_hex(static_extradata),
            from_hex(sig),
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn require_valid_order_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
        sig: String,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Vec<u8>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));
        let runtime_api_result = api.require_valid_order_ex(
            &at,
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            from_hex(calldata),
            from_hex(replacement_pattern),
            from_hex(static_extradata),
            from_hex(sig),
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn calculate_current_price_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_method: FeeMethod,
        side: Side,
        sale_kind: SaleKind,
        how_to_call: HowToCall,
        calldata: String,
        replacement_pattern: String,
        static_extradata: String,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<u64> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.calculate_current_price_ex(
            &at,
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            from_hex(calldata),
            from_hex(replacement_pattern),
            from_hex(static_extradata),
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
    fn orders_can_match_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_methods_sides_kinds_how_to_calls: String,
        calldata_buy: String,
        calldata_sell: String,
        replacement_pattern_buy: String,
        replacement_pattern_sell: String,
        static_extradata_buy: String,
        static_extradata_sell: String,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<bool> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.orders_can_match_ex(
            &at,
            addrs,
            uints,
            from_hex(fee_methods_sides_kinds_how_to_calls),
            from_hex(calldata_buy),
            from_hex(calldata_sell),
            from_hex(replacement_pattern_buy),
            from_hex(replacement_pattern_sell),
            from_hex(static_extradata_buy),
            from_hex(static_extradata_sell),
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
    fn calculate_match_price_ex(
        &self,
        addrs: Vec<AccountId>,
        uints: Vec<u64>,
        fee_methods_sides_kinds_how_to_calls: String,
        calldata_buy: String,
        calldata_sell: String,
        replacement_pattern_buy: String,
        replacement_pattern_sell: String,
        static_extradata_buy: String,
        static_extradata_sell: String,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<u64> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

        let runtime_api_result = api.calculate_match_price_ex(
            &at,
            addrs,
            uints,
            from_hex(fee_methods_sides_kinds_how_to_calls),
            from_hex(calldata_buy),
            from_hex(calldata_sell),
            from_hex(replacement_pattern_buy),
            from_hex(replacement_pattern_sell),
            from_hex(static_extradata_buy),
            from_hex(static_extradata_sell),
        );
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(9876), // No real reason for this value
            message: "Something wrong".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
}

pub fn from_hex(str: String) -> Vec<u8> {
    if let Some(s) = str.strip_prefix("0x") {
        return decode_hex(&s);
    }
    str.into_bytes()
}
pub fn decode_hex(s: &str) -> Vec<u8> {
    let len = if s.len() % 2 != 0 {
        s.len() - 1
    } else {
        s.len()
    };

    if 0 == len {
        return Vec::<u8>::new();
    }

    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}
