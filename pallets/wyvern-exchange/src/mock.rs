// Creating mock runtime here

pub use crate as wyvern_exchange;
pub use crate::{exchange_common::*, exchange_core::*, Config, Module};
use core::marker::PhantomData;
use frame_support::{
	impl_outer_event, impl_outer_origin, parameter_types,
	traits::{Currency, EnsureOrigin, OnFinalize, OnInitialize},
	weights::Weight,
};
pub use frame_system as system;
use frame_system::RawOrigin;
use sp_core::{sr25519, Pair, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};

impl_outer_origin! {
	pub enum Origin for Test {}
}

impl_outer_event! {
	pub enum TestEvent for Test {
		balances<T>,
		system<T>,
		exchange_core<T>,
	}
}

impl balances::Config for Test {
	type Balance = u64;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;

	type MaxLocks = ();
	type Event = TestEvent;
	type AccountStore = System;
	type WeightInfo = ();
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
pub const ExistentialDeposit: u64 = 500;
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Config for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = sr25519::Public;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

impl timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ();
	type WeightInfo = ();
}

impl Config for Test {}

impl wyvern_exchange::exchange_common::Config for Test {
	type Currency = Balances;
}

impl wyvern_exchange::sale_kind_interface::Config for Test {}
impl exchange_core::Config for Test {
	type Event = TestEvent;
	type Public = sr25519::Public;
	type Signature = sr25519::Signature;
}

pub type WyvernExchange = Module<Test>;
pub type ExchangeCore = exchange_core::Module<Test>;
pub type System = system::Module<Test>;
pub type Timestamp = timestamp::Pallet<Test>;
pub type Balances = balances::Module<Test>;

/// Run until a particular block.
pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		WyvernExchange::on_initialize(System::block_number());
	}
}

pub struct MockOrigin<T>(PhantomData<T>);

impl<T: Config> EnsureOrigin<T::Origin> for MockOrigin<T> {
	type Success = T::AccountId;
	fn try_origin(o: T::Origin) -> Result<Self::Success, T::Origin> {
		o.into().and_then(|o| match o {
			RawOrigin::Signed(ref who) => Ok(who.clone()),
			r => Err(T::Origin::from(r)),
		})
	}
}

pub fn create_account_test(account_id: sr25519::Public) {
	let _ = Balances::deposit_creating(&account_id, 100_000_000_000_000_000);
}
// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let mut ext = sp_io::TestExternalities::from(storage);
	// Events are not emitted on block 0 -> advance to block 1.
	// Any dispatchable calls made during genesis block will have no events emitted.
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn account_pair(s: &str) -> sr25519::Pair {
	sr25519::Pair::from_string(&format!("//{}", s), None).expect("static values are valid; qed")
}

pub fn account_key(s: &str) -> sr25519::Public {
	sr25519::Pair::from_string(&format!("//{}", s), None)
		.expect("static values are valid; qed")
		.public()
}
