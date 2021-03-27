#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use pallet_grandpa::fg_primitives;
use pallet_grandpa::{AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::traits::{
    BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, Saturating, Verify,
};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// A few exports that help ease life for downstream crates.
pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{KeyOwnerProofSystem, Randomness},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        IdentityFee, Weight,
    },
    StorageValue,
};
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{MultiSigner, Perbill, Permill};

use orderbook::{AssetQuery, JSONType, OrderJSONType, OrderQuery};

use wyvern_exchange_core::{FeeMethod, HowToCall, OrderType, SaleKind, Side};

/// Import the template pallet.
// pub use pallet_template;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

/// A timestamp: milliseconds since the unix epoch.
pub type Moment = u64;
/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub aura: Aura,
            pub grandpa: Grandpa,
        }
    }
}

pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("pacific-store"),
    impl_name: create_runtime_str!("pacific-store"),
    authoring_version: 1,
    spec_version: 100,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
};

pub const MILLISECS_PER_BLOCK: u64 = 6000;

pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    /// We allow for 2 seconds of compute with a 6 second average block time.
    pub const MaximumBlockWeight: Weight = 2 * WEIGHT_PER_SECOND;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    /// Assume 10% of weight for average on_initialize calls.
    pub MaximumExtrinsicWeight: Weight = AvailableBlockRatio::get()
        .saturating_sub(Perbill::from_percent(10)) * MaximumBlockWeight::get();
    pub const MaximumBlockLength: u32 = 5 * 1024 * 1024;
    pub const Version: RuntimeVersion = VERSION;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Trait for Runtime {
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = ();
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type Call = Call;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = multiaddress::AccountIdLookup<AccountId, ()>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The ubiquitous event type.
    type Event = Event;
    /// The ubiquitous origin type.
    type Origin = Origin;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// Maximum weight of each block.
    type MaximumBlockWeight = MaximumBlockWeight;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// The weight of the overhead invoked on the block import process, independent of the
    /// extrinsics included in that block.
    type BlockExecutionWeight = BlockExecutionWeight;
    /// The base weight of any extrinsic processed by the runtime, independent of the
    /// logic of that extrinsic. (Signature verification, nonce increment, fee, etc...)
    type ExtrinsicBaseWeight = ExtrinsicBaseWeight;
    /// The maximum weight that a single extrinsic of `Normal` dispatch class can have,
    /// idependent of the logic of that extrinsics. (Roughly max block weight - average on
    /// initialize cost).
    type MaximumExtrinsicWeight = MaximumExtrinsicWeight;
    /// Maximum size of all encoded transactions (in bytes) that are allowed in one block.
    type MaximumBlockLength = MaximumBlockLength;
    /// Portion of the block weight that is available to all normal transactions.
    type AvailableBlockRatio = AvailableBlockRatio;
    /// Version of the runtime.
    type Version = Version;
    /// Converts a module to the index of the module in `construct_runtime!`.
    /// This type is being generated by `construct_runtime!`.
    type PalletInfo = PalletInfo;
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
}

impl pallet_aura::Trait for Runtime {
    type AuthorityId = AuraId;
}

impl pallet_grandpa::Trait for Runtime {
    type Event = Event;
    type Call = Call;

    type KeyOwnerProofSystem = ();

    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        GrandpaId,
    )>>::IdentificationTuple;

    type HandleEquivocation = ();

    type WeightInfo = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Trait for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 500;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Trait for Runtime {
    type MaxLocks = MaxLocks;
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

parameter_types! {
    pub const TransactionByteFee: Balance = 1;
}

impl pallet_transaction_payment::Trait for Runtime {
    type Currency = Balances;
    type OnTransactionPayment = ();
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
}

impl pallet_sudo::Trait for Runtime {
    type Event = Event;
    type Call = Call;
}

impl orderbook::Trait for Runtime {
    type Event = Event;
}

impl wyvern_exchange::Trait for Runtime {}

impl wyvern_exchange_core::sale_kind_interface::Trait for Runtime {}

impl wyvern_exchange_core::exchange_common::Trait for Runtime {
    type Currency = Balances;
}

impl wyvern_exchange_core::Trait for Runtime {
    type Event = Event;
    type Public = MultiSigner;
    type Signature = Signature;
}
// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
        Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
        Aura: pallet_aura::{Module, Config<T>, Inherent},
        Grandpa: pallet_grandpa::{Module, Call, Storage, Config, Event},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Module, Storage},
        Sudo: pallet_sudo::{Module, Call, Config<T>, Storage, Event<T>},
        // Include the custom logic from the template pallet in the runtime.
        Orderbook: orderbook::{Module, Call, Storage, Event<T>},
        WyvernExchange: wyvern_exchange::{Module, Call, Storage},
        WyvernExchangeCore: wyvern_exchange_core::{Module, Call, Storage, Event<T>},
        // TemplateModule: pallet_template::{Module, Call, Storage, Event<T>},
    }
);

/// The address format for describing accounts.
mod multiaddress;
pub type Address = multiaddress::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllModules,
>;

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            Runtime::metadata().into()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) ->
            Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }

        fn random_seed() -> <Block as BlockT>::Hash {
            RandomnessCollectiveFlip::random_seed()
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> u64 {
            Aura::slot_duration()
        }

        fn authorities() -> Vec<AuraId> {
            Aura::authorities()
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl fg_primitives::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> GrandpaAuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: fg_primitives::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            _key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: fg_primitives::SetId,
            _authority_id: GrandpaId,
        ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
            // NOTE: this is the only implementation possible since we've
            // defined our key owner proof type as a bottom type (i.e. a type
            // with no values).
            None
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
        for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
    }

    impl orderbook_runtime_api::OrderbookApi<Block,AccountId, Moment> for Runtime {
        fn get_order(
            order_query: Option<OrderQuery<AccountId>>,
        ) -> Option<OrderJSONType<AccountId, Moment>> {
            Orderbook::get_order(order_query)
        }
        fn get_orders(
            order_query: Option<OrderQuery<AccountId>>, 
            page: Option<u64>,
        ) -> Option<Vec<OrderJSONType<AccountId, Moment>>> {
            Orderbook::get_orders(order_query,page)
        }
        fn get_asset(
            token_address: Option<Vec<u8>>,
            token_id: Option<Vec<u8>>,
        ) -> Option<JSONType> {
            Orderbook::get_asset(token_address,token_id)
        }
        fn get_assets(
            asset_query: Option<AssetQuery<AccountId>>, 
            page: Option<u64>,
        ) -> Option<Vec<JSONType>> {
            Orderbook::get_assets(asset_query,page)
        }
    }

    impl wyvern_exchange_runtime_api::WyvernExchangeApi<Block,AccountId, Balance,Moment,Signature> 
        for Runtime {
        fn calculate_final_price_ex(
            side: Side,
            sale_kind: SaleKind,
            base_price: u64,
            extra: Moment,
            listing_time: Moment,
            expiration_time: Moment,
        ) -> u64{
            WyvernExchange::calculate_final_price_ex( 
                side,
                sale_kind,
                base_price,
                extra,
                listing_time,
                expiration_time,
            )
        }
        fn hash_order_ex(
            addrs: Vec<AccountId>,
            uints: Vec<u64>,
            fee_method: FeeMethod,
            side: Side,
            sale_kind: SaleKind,
            how_to_call: HowToCall,
            calldata: Vec<u8>,
            replacement_pattern: Vec<u8>,
            static_extradata: Vec<u8>,
        ) -> Vec<u8>{
            WyvernExchange::hash_order_ex(
            addrs,
            uints,
            fee_method,
            side,
            sale_kind,
            how_to_call,
            calldata,
            replacement_pattern,
            static_extradata,
            )
        }
        fn hash_to_sign_ex(
            addrs: Vec<AccountId>,
            uints: Vec<u64>,
            fee_method: FeeMethod,
            side: Side,
            sale_kind: SaleKind,
            how_to_call: HowToCall,
            calldata: Vec<u8>,
            replacement_pattern: Vec<u8>,
            static_extradata: Vec<u8>,
        ) -> Vec<u8>{
            WyvernExchange::hash_to_sign_ex(
                addrs,
                uints,
                fee_method,
                side,
                sale_kind,
                how_to_call,
                calldata,
                replacement_pattern,
                static_extradata,
            )
        }
    fn validate_order_parameters_ex(
            addrs: Vec<AccountId>,
            uints: Vec<u64>,
            fee_method: FeeMethod,
            side: Side,
            sale_kind: SaleKind,
            how_to_call: HowToCall,
            calldata: Vec<u8>,
            replacement_pattern: Vec<u8>,
            static_extradata: Vec<u8>,
        ) -> bool{
            WyvernExchange::validate_order_parameters_ex(
                addrs,
                uints,
                fee_method,
                side,
                sale_kind,
                how_to_call,
                calldata,
                replacement_pattern,
                static_extradata,
            )
        }
    fn validate_order_ex(
            addrs: Vec<AccountId>,
            uints: Vec<u64>,
            fee_method: FeeMethod,
            side: Side,
            sale_kind: SaleKind,
            how_to_call: HowToCall,
            calldata: Vec<u8>,
            replacement_pattern: Vec<u8>,
            static_extradata: Vec<u8>,
            sig: Vec<u8>,
        ) -> bool {
            WyvernExchange::validate_order_ex(
                addrs,
                uints,
                fee_method,
                side,
                sale_kind,
                how_to_call,
                calldata,
                replacement_pattern,
                static_extradata,
                sig,
            )
        }
        fn require_valid_order_ex(
            addrs: Vec<AccountId>,
            uints: Vec<u64>,
            fee_method: FeeMethod,
            side: Side,
            sale_kind: SaleKind,
            how_to_call: HowToCall,
            calldata: Vec<u8>,
            replacement_pattern: Vec<u8>,
            static_extradata: Vec<u8>,
            sig: Vec<u8>,
        ) -> Vec<u8> {
            WyvernExchange::require_valid_order_ex(
                addrs,
                uints,
                fee_method,
                side,
                sale_kind,
                how_to_call,
                calldata,
                replacement_pattern,
                static_extradata,
                sig,
            )
        }
        fn calculate_current_price_ex(
            addrs: Vec<AccountId>,
            uints: Vec<u64>,
            fee_method: FeeMethod,
            side: Side,
            sale_kind: SaleKind,
            how_to_call: HowToCall,
            calldata: Vec<u8>,
            replacement_pattern: Vec<u8>,
            static_extradata: Vec<u8>,
        ) -> u64{
            WyvernExchange::calculate_current_price_ex(
                addrs,
                uints,
                fee_method,
                side,
                sale_kind,
                how_to_call,
                calldata,
                replacement_pattern,
                static_extradata,
            )
        }
        fn orders_can_match_ex(
            addrs: Vec<AccountId>,
            uints: Vec<u64>,
            fee_methods_sides_kinds_how_to_calls: Vec<u8>,
            calldata_buy: Vec<u8>,
            calldata_sell: Vec<u8>,
            replacement_pattern_buy: Vec<u8>,
            replacement_pattern_sell: Vec<u8>,
            static_extradata_buy: Vec<u8>,
            static_extradata_sell: Vec<u8>,
        ) -> bool {
            WyvernExchange::orders_can_match_ex(
                addrs,
                uints,
                fee_methods_sides_kinds_how_to_calls,
                calldata_buy,
                calldata_sell,
                replacement_pattern_buy,
                replacement_pattern_sell,
                static_extradata_buy,
                static_extradata_sell,
            )
        }
        fn calculate_match_price_ex(
            addrs: Vec<AccountId>,
            uints: Vec<u64>,
            fee_methods_sides_kinds_how_to_calls: Vec<u8>,
            calldata_buy: Vec<u8>,
            calldata_sell: Vec<u8>,
            replacement_pattern_buy: Vec<u8>,
            replacement_pattern_sell: Vec<u8>,
            static_extradata_buy: Vec<u8>,
            static_extradata_sell: Vec<u8>,
        ) -> u64{
            WyvernExchange::calculate_match_price_ex(
                addrs,
                uints,
                fee_methods_sides_kinds_how_to_calls,
                calldata_buy,
                calldata_sell,
                replacement_pattern_buy,
                replacement_pattern_sell,
                static_extradata_buy,
                static_extradata_sell,
            )
        }
    }
    impl wyvern_exchange_core_runtime_api::WyvernExchangeCoreApi<
        Block,
        AccountId,
        Balance,
        Moment,
        Signature,
    >
        for Runtime {
        fn hash_order(
            order: OrderType<AccountId, Moment, Balance>,
        ) -> Vec<u8>{
            WyvernExchangeCore::hash_order(&order).unwrap()
        }

        fn hash_to_sign(
            order: OrderType<AccountId, Moment, Balance>,
        ) -> Vec<u8>{
            WyvernExchangeCore::hash_to_sign(&order).unwrap()
        }
        fn validate_order_parameters(
            order: OrderType<AccountId, Moment, Balance>,
        ) -> bool{
            WyvernExchangeCore::validate_order_parameters(&order)
        }
        fn validate_order(
            hash:  Vec<u8>,
            order: OrderType<AccountId, Moment, Balance>,
            sig: Vec<u8>,
        ) -> bool {
            WyvernExchangeCore::validate_order(&hash,&order,&sig).unwrap()
        }
        fn require_valid_order(
            order: OrderType<AccountId, Moment, Balance>,
            sig: Vec<u8>,
        ) -> Vec<u8> {
            WyvernExchangeCore::require_valid_order(&order,&sig).unwrap()
        }
        fn calculate_current_price(
            order: OrderType<AccountId, Moment, Balance>,
        ) -> Balance{
            WyvernExchangeCore::calculate_current_price(&order).unwrap()
        }
        fn orders_can_match(
            buy: OrderType<AccountId, Moment, Balance>,
            sell: OrderType<AccountId, Moment, Balance>,
        ) -> bool {
            WyvernExchangeCore::orders_can_match(&buy,&sell)
        }
        fn calculate_match_price(
            buy: OrderType<AccountId, Moment, Balance>,
            sell: OrderType<AccountId, Moment, Balance>,
        ) -> Balance{
            WyvernExchangeCore::calculate_match_price(&buy,&sell).unwrap()
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

            use frame_system_benchmarking::Module as SystemBench;
            impl frame_system_benchmarking::Trait for Runtime {}

            let whitelist: Vec<TrackedStorageKey> = vec![
                // Block Number
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac")
                    .to_vec().into(),
                // Total Issuance
                hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80")
                    .to_vec().into(),
                // Execution Phase
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a")
                    .to_vec().into(),
                // Event Count
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850")
                    .to_vec().into(),
                // System Events
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7")
                    .to_vec().into(),
            ];

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);

            add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
            add_benchmark!(params, batches, pallet_balances, Balances);
            add_benchmark!(params, batches, pallet_timestamp, Timestamp);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}
