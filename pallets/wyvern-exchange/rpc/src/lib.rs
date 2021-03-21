//! RPC interface for the transaction payment module.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use sp_core::bytes;
use codec::Codec;
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



// fn parse_hex(hex_asm: &str) -> Vec<u8> {
//     let mut hex_bytes = hex_asm.as_bytes().iter().filter_map(|b| {
//         match b {
//             b'0'..=b'9' => Some(b - b'0'),
//             b'a'..=b'f' => Some(b - b'a' + 10),
//             b'A'..=b'F' => Some(b - b'A' + 10),
//             _ => None,
//         }
//     }).fuse();

//     let mut bytes = Vec::new();
//     while let (Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
//         bytes.push(h << 4 | l)
//     }
//     bytes
// }



// fn parse_hex(hex_asm: &str) -> Vec<u8> {
//     let hex_chars: Vec<char> = hex_asm.as_bytes().iter().filter_map(|b| {
//         let ch = char::from(*b);
//         if ('0' <= ch && ch <= '9') || ('a' <= ch && ch <= 'f') || ('A' <= ch && ch <= 'F') {
//             Some(ch)
//         } else {
//             None
//         }
//     }).collect();

//     let mut index = 0usize;
//     let (odd_chars, even_chars): (Vec<char>, Vec<char>) = hex_chars.into_iter().partition(|_| { 
//         index = index + 1;
//         index % 2 == 1
//     });

//     odd_chars.into_iter().zip(even_chars.into_iter()).map(|(c0, c1)| {
//         fn hexchar2int(ch: char) -> u8 {
//             if '0' <= ch && ch <= '9' {
//                 ch as u8 - '0' as u8
//             } else {
//                 0xa + 
//                 if 'a' <= ch && ch <= 'f' {
//                     ch as u8 - 'a' as u8
//                 } else if 'A' <= ch && ch <= 'F' {
//                     ch as u8 - 'A' as u8
//                 } else {
//                     unreachable!()
//                 }
//             }
//         }
//         hexchar2int(c0) * 0x10 + hexchar2int(c1)            
//     }).collect::<Vec<u8>>()
// }


// use std::{fmt::Write, num::ParseIntError};

// pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
//     (0..s.len())
//         .step_by(2)
//         .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
//         .collect()
// }

// pub fn encode_hex(bytes: &[u8]) -> String {
//     let mut s = String::with_capacity(bytes.len() * 2);
//     for &b in bytes {
//         write!(&mut s, "{:02x}", b);
//     }
//     s
// }

pub fn from_hex(str: String) -> Vec<u8> {
decode_hex(&(str.strip_prefix("0x").unwrap()))
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