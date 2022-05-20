//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use contracts_node_runtime::{opaque::Block, AccountId, Balance, BlockNumber, Hash, Index, Moment, Signature};
use orderbook_rpc::OrderbookApiServer;
use orderbook_runtime_api;
use pallet_contracts_rpc::{ContractsRpc, ContractsApiServer};
use jsonrpsee::RpcModule;
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use wyvern_exchange_core_rpc::WyvernExchangeCoreApiServer;
use wyvern_exchange_core_runtime_api;
 use wyvern_exchange_rpc::WyvernExchangeApiServer;
use wyvern_exchange_runtime_api;

/// Full client dependencies.
pub struct FullDeps<C, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(deps: FullDeps<C, P>) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	C::Api: orderbook_runtime_api::OrderbookApi<Block, AccountId, Moment>,
	C::Api: wyvern_exchange_runtime_api::WyvernExchangeApi<
		Block,
		AccountId,
		Balance,
		Moment,
		Signature,
	>,
	C::Api: wyvern_exchange_core_runtime_api::WyvernExchangeCoreApi<
		Block,
		AccountId,
		Balance,
		Moment,
		Signature,
	>,
	P: TransactionPool + 'static,
{
	use pallet_transaction_payment_rpc::{TransactionPaymentRpc, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{SystemRpc, SystemApiServer};

	let mut io = RpcModule::new(());
	let FullDeps { client, pool, deny_unsafe } = deps;

	io.merge(SystemRpc::new(client.clone(), pool, deny_unsafe).into_rpc())?;

	io.merge(TransactionPaymentRpc::new(client.clone()).into_rpc())?;

	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `io.extend_with(YourRpcTrait::to_delegate(YourRpcStruct::new(ReferenceToClient, ...)));`

	// Contracts RPC API extension
	io.merge(ContractsRpc::new(client.clone()).into_rpc())?;
	io.merge(orderbook_rpc::Orderbook::new(
		client.clone()
	).into_rpc())?;

	io.merge(
		wyvern_exchange_rpc::WyvernExchange::new(client.clone()).into_rpc()
	)?;

	io.merge(
		wyvern_exchange_core_rpc::WyvernExchangeCore::new(client.clone()).into_rpc()
	)?;
	Ok(io)
}
