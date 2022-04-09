//! # WyvernExchangeCore Pallet
//!
//!
//! ## Overview
//!
//!
//!  Decentralized digital asset exchange. Supports any digital asset that can be represented
//!  on the Ethereum blockchain (i.e. - transferred in an Polkadot extrinsic or sequence of
//! transactions).
//!
//!  Let us suppose two agents interacting with a distributed ledger have utility functions
//! preferencing  certain states of that ledger over others.
//!  Aiming to maximize their utility, these agents may construct with their utility functions
//!  along with the present ledger state a mapping of state transitions (transactions) to marginal
//! utilities.  Any composite state transition with positive marginal utility for and enactable
//!  by the combined permissions of both agents thus is a mutually desirable trade, and the
//! trustless  code execution provided by a distributed ledger renders the requisite atomicity
//! trivial.  The WyvernExchangeCore pallet provides functionality for WyvernExchanges management.
//!
//! * Approve Order
//! * Cancel Order
//! * Hash Order
//! * Validate Order
//! * AtomicMatch Order
//!
//! ### Goals
//!
//! The WyvernExchange system in Substrate is designed to make the following possible:
//!
//! *Autonomously governed decentralized digital asset exchange.
//!
//! ### Dispatchable Functions
//!
//! * `change_minimum_maker_protocol_fee` - Change the minimum maker fee paid to the protocol (only
//!   -owner)
//! * `change_minimum_taker_protocol_fee` - Change the minimum taker fee paid to the protocol (only
//!   -owner)
//! * `change_protocol_fee_recipient` - Change the protocol fee recipient (only -owner)
//! * `approve_order ` - Approve an order and optionally mark it for orderbook inclusion. Must be
//!   called by the maker of the order
//! * `cancel_order` - Cancel an order, preventing it from being matched. Must be called by the
//!   maker of the order
//! * `atomic_match` -Atomically match two orders, ensuring validity of the match, and execute all
//!   associated state transitions. Protected against reentrancy by a contract-global lock.

//! ### Public  Functions
//!
//! * `hash_order` - Hash an order, returning the canonical order hash, without the message prefix
//! * `hash_to_sign` - Hash an order, returning the hash that a client must sign.
//! * `require_valid_order ` - Assert an order is valid and return its hash order OrderType to
//!   validate sig  signature.
//! * `validate_order ` - Validate a provided previously approved / signed order, hash, and
//!   signature.
//! * `validate_order_parameters` - Validate order parameters (doesnot check validity-signature)
//! * `calculate_current_price` - Calculate the current price of an order (fn -convenience)
//! * `calculate_match_price` - Calculate the price two orders would match at, if in fact they would
//!   match (fail -otherwise).
//! * `orders_can_match` - Return whether or not two orders can be matched with each other by basic
//!   parameters (does not check order signatures / calldata or perform calls -static).
//! * `calculate_final_price ` - Calculate the settlement price of an order;   Precondition:
//!   parameters have passed validate_parameters.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod types;

pub mod exchange_common;

pub mod sale_kind_interface;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use pallet_contracts::chain_extension::UncheckedFrom;
	use frame_support::{
		dispatch::DispatchResult,
		ensure,
		sp_io::hashing::keccak_256,
		sp_runtime::traits::{IdentifyAccount, Member, Verify, Zero},
		sp_std::prelude::*,
		traits::Currency,
	};
   use frame_system::Config as SysConfig;
    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	pub type BalanceOfC<T> = <<T as pallet_contracts::Config>::Currency as Currency<
		<T as SysConfig>::AccountId,
	>>::Balance;

	pub use crate::types::*;
	use crate::{exchange_common, exchange_common::BalanceOf, sale_kind_interface};
	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ sale_kind_interface::Config
		+ exchange_common::Config
		+ pallet_contracts::Config
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Public: IdentifyAccount<AccountId = <Self as frame_system::Config>::AccountId> + Clone;
		type Signature: Verify<Signer = Self::Public> + Member + Decode + Encode;
	}
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(super) type NextOrderIndex<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	pub(super) type Owner<T: Config> = StorageValue<Value = AccountIdOf<T>, QueryKind = ValueQuery, OnEmpty = MyAccountIdDefault<T>>;

	#[pallet::storage]
	pub(super) type ContractSelf<T: Config> = StorageValue<Value = AccountIdOf<T>, QueryKind = ValueQuery, OnEmpty = MyAccountIdDefault<T>>;

	#[pallet::storage]
	pub(super) type ProxyRegistry<T: Config> = StorageValue<Value = AccountIdOf<T>, QueryKind = ValueQuery, OnEmpty = MyAccountIdDefault<T>>;

	#[pallet::storage]
	pub(super) type TokenTransferProxy<T: Config> = StorageValue<Value = AccountIdOf<T>, QueryKind = ValueQuery, OnEmpty = MyAccountIdDefault<T>>;

	//The token used to pay exchange fees.
	#[pallet::storage]
	pub(super) type ExchangeToken<T: Config> = StorageValue<Value = AccountIdOf<T>, QueryKind = ValueQuery, OnEmpty = MyAccountIdDefault<T>>;

	//Cancelled / finalized orders, by hash.
	#[pallet::storage]
	#[pallet::getter(fn cancelled_or_finalized)]
	pub(super) type CancelledOrFinalized<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, bool, ValueQuery>;
	//Orders verified by on-chain approval (alternative to  signatures
	// so that smart contracts can place orders directly).
	#[pallet::storage]
	#[pallet::getter(fn approved_orders)]
	pub(super) type ApprovedOrders<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, bool, ValueQuery>;
	//For split fee orders, minimum required protocol maker fee, in basis points.
	//Paid to owner (who can change it).

	#[pallet::storage]
	pub(super) type MinimumMakerProtocolFee<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;
	//For split fee orders, minimum required protocol taker fee, in basis points.
	//Paid to owner (who can change it).
	#[pallet::storage]
	pub(super) type MinimumTakerProtocolFee<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;
	//Recipient of protocol fees.
	#[pallet::storage]
	pub(super) type ProtocolFeeRecipient<T: Config> = StorageValue<Value = AccountIdOf<T>, QueryKind = ValueQuery, OnEmpty = MyAccountIdDefault<T>>;

	#[pallet::type_value]
	pub(super) fn GasLimitDefault<T: Config>() -> Weight {
		20000000000
	}
	#[pallet::storage]
	pub(super) type GasLimit<T> =
		StorageValue<Value = Weight, QueryKind = ValueQuery, OnEmpty = GasLimitDefault<T>>;

	#[pallet::type_value]
	pub(super) fn ProxySelectorDefault<T: Config>() -> Vec<u8> {
		vec![0x55, 0x7e, 0xfb, 0x0c]
	}
	#[pallet::storage]
	pub(super) type ProxySelector<T> =
		StorageValue<Value = Vec<u8>, QueryKind = ValueQuery, OnEmpty = ProxySelectorDefault<T>>;

	#[pallet::type_value]
	pub(super) fn TokenSelectorDefault<T: Config>() -> Vec<u8> {
		vec![0x0b, 0x39, 0x6f, 0x18]
	}
	#[pallet::storage]
	pub(super) type TokenSelector<T> =
		StorageValue<Value = Vec<u8>, QueryKind = ValueQuery, OnEmpty = TokenSelectorDefault<T>>;
#[pallet::type_value]
	pub(super) fn MyAccountIdDefault<T: Config>() ->  AccountIdOf<T> {
		AccountIdOf::<T>::decode(&mut &vec![1u8;32][..]).expect("32 bytes can always construct an AccountId32")
	}
	#[pallet::storage]
	pub(super) type MyAccountId<T> =
		StorageValue<Value = AccountIdOf<T>, QueryKind = ValueQuery, OnEmpty = MyAccountIdDefault<T>>;
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		OrderApprovedPartOne(
			Vec<u8>,
			AccountIdOf<T>,
			AccountIdOf<T>,
			AccountIdOf<T>,
			BalanceOf<T>,
			BalanceOf<T>,
			BalanceOf<T>,
			BalanceOf<T>,
			AccountIdOf<T>,
			FeeMethod,
			Side,
			SaleKind,
			AccountIdOf<T>,
		),
		OrderApprovedPartTwo(
			Vec<u8>,
			HowToCall,
			Vec<u8>,
			Vec<u8>,
			AccountIdOf<T>,
			Vec<u8>,
			AccountIdOf<T>,
			BalanceOf<T>,
			T::Moment,
			T::Moment,
			T::Moment,
			u64,
			bool,
		),
		OrderCancelled(Vec<u8>),
		OrdersMatched(Vec<u8>, Vec<u8>, AccountIdOf<T>, AccountIdOf<T>, BalanceOf<T>, Vec<u8>),
		MinimumMakerProtocolFeeChanged(BalanceOf<T>),
		MinimumTakerProtocolFeeChanged(BalanceOf<T>),
		ProtocolFeeRecipientChanged(AccountIdOf<T>, AccountIdOf<T>),
		OwnerChanged(AccountIdOf<T>, AccountIdOf<T>),
		ContractSelfChanged(AccountIdOf<T>, AccountIdOf<T>),
		ProxyRegistryChanged(AccountIdOf<T>, AccountIdOf<T>),
		TokenTransferProxyChanged(AccountIdOf<T>, AccountIdOf<T>),
		GasLimitChanged(AccountIdOf<T>, Weight),
		ProxySelectorChanged(AccountIdOf<T>, Vec<u8>),
		TokenSelectorChanged(AccountIdOf<T>, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		MsgVerifyFailed,
		InvalidBuyOrderParameters,
		InvalidSellOrderParameters,
		OrdersCannotMatch,
		ListingTimeExpired,
		ArrayNotEqual,
		BuyArrayNotEqual,
		SellArrayNotEqual,
		BuyTakerProtocolFeeGreaterThanSellTakerProtocolFee,
		BuyTakerRelayerFeeGreaterThanSellTakerRelayerFee,
		SellPaymentTokenEqualPaymentToken,
		SellTakerProtocolFeeGreaterThanBuyTakerProtocolFee,
		SellTakerRelayerFeeGreaterThanBuyTakerRelayerFee,
		ValueLessThanRequiredAmount,
		ValueNotZero,
		BuyPriceLessThanSellPrice,
		OrderHashExists,
		OnlyMaker,
		InvalidOrderHash,
		InvalidSignature,
		OnlyOwner,
		OnlyContractSelf,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
  <T as SysConfig>::AccountId: pallet_contracts::chain_extension::UncheckedFrom<<T as SysConfig>::Hash>,
        <T as SysConfig>::AccountId: AsRef<[u8]>,
		AccountIdOf<T>: UncheckedFrom<<T as SysConfig>::Hash>,
		AccountIdOf<T>: AsRef<[u8]>,
 <T as pallet_contracts::Config>::Currency: Currency<<T as frame_system::Config>::AccountId>,
	{
		// Change the minimum maker fee paid to the protocol (only:owner)
		// newMinimumMakerProtocolFee New fee to set in basis points
		#[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
		pub fn change_minimum_maker_protocol_fee(
			origin: OriginFor<T>,
			new_minimum_maker_protocol_fee: BalanceOf<T>,
		) -> DispatchResult {
			let _user = ensure_signed(origin)?;
			Self::only_owner(&_user)?;
			MinimumMakerProtocolFee::<T>::put(new_minimum_maker_protocol_fee);
			Self::deposit_event(Event::MinimumMakerProtocolFeeChanged(
				new_minimum_maker_protocol_fee,
			));

			Ok(())
		}

		// Change the minimum taker fee paid to the protocol (only:owner)
		// new_minimum_taker_protocol_fee New fee to set in basis points
		#[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
		pub fn change_minimum_taker_protocol_fee(
			origin: OriginFor<T>,
			new_minimum_taker_protocol_fee: BalanceOf<T>,
		) -> DispatchResult {
			// onlyOwner
			let _user = ensure_signed(origin)?;
			Self::only_owner(&_user)?;
			MinimumTakerProtocolFee::<T>::put(new_minimum_taker_protocol_fee);
			Self::deposit_event(Event::MinimumTakerProtocolFeeChanged(
				new_minimum_taker_protocol_fee,
			));

			Ok(())
		}

		// Change the protocol fee recipient (only:owner)
		// new_protocol_fee_recipient New protocol fee recipient AccountId
		#[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
		pub fn change_protocol_fee_recipient(
			origin: OriginFor<T>,
			new_protocol_fee_recipient: AccountIdOf<T>,
		) -> DispatchResult {
			let _user = ensure_signed(origin)?;
			Self::only_owner(&_user)?;
			ProtocolFeeRecipient::<T>::put(new_protocol_fee_recipient.clone());
			Self::deposit_event(Event::ProtocolFeeRecipientChanged(
				_user,
				new_protocol_fee_recipient.clone(),
			));
			Ok(())
		}

		#[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
		pub fn change_owner(origin: OriginFor<T>, new_owner: AccountIdOf<T>) -> DispatchResult {
			let _user = ensure_signed(origin)?;
			sp_runtime::runtime_logger::RuntimeLogger::init();

			ensure!(
				MyAccountIdDefault::<T>::get() == Owner::<T>::get() || _user == Owner::<T>::get(),
				Error::<T>::OnlyOwner,
			);
			Owner::<T>::put(new_owner.clone());
			Self::deposit_event(Event::OwnerChanged(_user, new_owner.clone()));
			Ok(())
		}

		#[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
		pub fn set_contract_self(origin: OriginFor<T>, contract: AccountIdOf<T>) -> DispatchResult {
			let _user = ensure_signed(origin)?;
			Self::only_owner(&_user)?;
			ContractSelf::<T>::put(contract.clone());
			Self::deposit_event(Event::ContractSelfChanged(_user, contract.clone()));
			Ok(())
		}

		#[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
		pub fn set_proxy_registry(
			origin: OriginFor<T>,
			registry_address: AccountIdOf<T>,
		) -> DispatchResult {
			let _user = ensure_signed(origin)?;
			Self::only_owner(&_user)?;
			ProxyRegistry::<T>::put(registry_address.clone());
			Self::deposit_event(Event::ProxyRegistryChanged(_user, registry_address.clone()));
			Ok(())
		}
		#[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
		pub fn set_token_transfer_proxy(
			origin: OriginFor<T>,
			proxy_address: AccountIdOf<T>,
		) -> DispatchResult {
			let _user = ensure_signed(origin)?;
			Self::only_owner(&_user)?;
			TokenTransferProxy::<T>::put(proxy_address.clone());
			Self::deposit_event(Event::TokenTransferProxyChanged(_user, proxy_address.clone()));
			Ok(())
		}

		#[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
		pub fn set_gas_limit(origin: OriginFor<T>, gas_limit: Weight) -> DispatchResult {
			let _user = ensure_signed(origin)?;
			Self::only_owner(&_user)?;
			GasLimit::<T>::put(gas_limit.clone());
			Self::deposit_event(Event::GasLimitChanged(_user, gas_limit.clone()));
			Ok(())
		}
		#[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
		pub fn set_proxy_selector(origin: OriginFor<T>, selectror: Vec<u8>) -> DispatchResult {
			let _user = ensure_signed(origin)?;
			Self::only_owner(&_user)?;
			ProxySelector::<T>::put(selectror.clone());
			Self::deposit_event(Event::ProxySelectorChanged(_user, selectror.clone()));
			Ok(())
		}

		#[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
		pub fn set_token_selector(origin: OriginFor<T>, selectror: Vec<u8>) -> DispatchResult {
			let _user = ensure_signed(origin)?;
			Self::only_owner(&_user)?;
			TokenSelector::<T>::put(selectror.clone());
			Self::deposit_event(Event::TokenSelectorChanged(_user, selectror.clone()));
			Ok(())
		}

		#[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
		/// A generic extrinsic to wrap
		/// [pallet_contracts::bare_call](https://github.com/paritytech/substrate/blob/352c46a648a5f2d4526e790a184daa4a1ffdb3bf/frame/contracts/src/lib.rs#L545-L562)
		///
		/// * `dest` - A destination account id for the contract being targeted
		/// * `selector` - The 'selector' of the ink! smart contract function.
		/// This can be retrived from the compiled `metadata.json`. It's possible to
		/// [specify a selector](https://paritytech.github.io/ink-docs/macros-attributes/selector/) in
		/// the smart contract itself.
		/// * `arg` - An argument to be passed to the smart contract.
		/// * `gas_limit` - The gas limit passed to the contract bare_call. This example should work
		///   when given a value of around 10000000000
		pub fn call_smart_contract(
			origin: OriginFor<T>,
			dest: AccountIdOf<T>,
			mut selector: Vec<u8>,
			mut selectors: Vec<u8>,
			callees: Vec<AccountIdOf<T>>,
			from: AccountIdOf<T>,
			to: AccountIdOf<T>,
			values: Vec<BalanceOfC<T>>,
			#[pallet::compact] gas_limit: Weight,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// Check against unbounded input
			// ensure!(selector.len() < 4, Error::<T>::InputTooLarge);
			// Amount to transfer
			let value: BalanceOfC<T> = Default::default();
			let mut callees_enc: Vec<u8> = callees.encode();
			let mut from_enc: Vec<u8> = from.encode();
			let mut to_enc: Vec<u8> = to.encode();
			let mut values_enc: Vec<u8> = values.encode();
			let mut data = Vec::new();
			data.append(&mut selector);
			data.append(&mut selectors);
			data.append(&mut callees_enc);
			data.append(&mut from_enc);
			data.append(&mut to_enc);
			data.append(&mut values_enc);
			use sp_std::if_std;
			if_std! {
				println!("The data_encode. is: {:?}",data);
			}
			// Do the actual call to the smart contract function
			let _r = pallet_contracts::Pallet::<T>::bare_call(
				who,
				dest.clone(),
				value,
				gas_limit,
				None,
				data,
				true,
			)
			.result;
			if_std! {
				println!("The call_smart_contracts. is: {:?}",_r);
			}
			// Self::deposit_event(Event::CalledContractFromPallet(dest));
			Ok(())
		}
		#[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
		pub fn call_smart_contracts(
			origin: OriginFor<T>,
			dest: AccountIdOf<T>,
			mut selector: Vec<u8>,
			from: AccountIdOf<T>,
			to: AccountIdOf<T>,
			values: BalanceOfC<T>,
			#[pallet::compact] gas_limit: Weight,
		) -> DispatchResult {
			use sp_std::if_std;

			let who = ensure_signed(origin)?;
			// Check against unbounded input
			// ensure!(selector.len() < 4, Error::<T>::InputTooLarge);
			// Amount to transfer
			let value: BalanceOfC<T> = Default::default();
			let mut from_enc: Vec<u8> = from.encode();
			let mut to_enc: Vec<u8> = to.encode();
			let mut values_enc: Vec<u8> = values.encode();
			let mut data = Vec::new();
			data.append(&mut selector);
			data.append(&mut from_enc);
			data.append(&mut to_enc);
			data.append(&mut values_enc);
			if_std! {
				println!(" dest={:?}=The call_smart_contracts data=. is: {:?}",dest.clone(),data.clone());
			}
			// Do the actual call to the smart contract function
			let _r = pallet_contracts::Pallet::<T>::bare_call(
				who,
				dest.clone(),
				value,
				gas_limit,
				None,
				data,
				true,
			)
			.result;
			if_std! {
				println!("The call_smart_contracts. is: {:?}",_r);
			}
			Ok(())
		}
	}
	impl<T: Config> Pallet<T>
	where
  <T as SysConfig>::AccountId: pallet_contracts::chain_extension::UncheckedFrom<<T as SysConfig>::Hash>,
        <T as SysConfig>::AccountId: AsRef<[u8]>,
		AccountIdOf<T>: UncheckedFrom<<T as frame_system::Config>::Hash>,
		AccountIdOf<T>: AsRef<[u8]>,
 <T as pallet_contracts::Config>::Currency: Currency<<T as frame_system::Config>::AccountId>,
	{
		pub fn only_owner(owner: &AccountIdOf<T>) -> DispatchResult {
			ensure!(Owner::<T>::get() == *owner, Error::<T>::OnlyOwner);
			Ok(())
		}
		// Transfer tokens
		// token Token to transfer
		// from AccountId to charge fees
		// to AccountId to receive fees
		// amount Amount of protocol tokens to charge
		pub fn transfer_tokens(
			msg_sender: &AccountIdOf<T>,
			_token: &AccountIdOf<T>,
			_from: &AccountIdOf<T>,
			_to: &AccountIdOf<T>,
			_amount: BalanceOf<T>,
		) -> Result<(), Error<T>> where <T as pallet_contracts::Config>::Currency: Currency<<T as frame_system::Config>::AccountId>  {
			if _amount > Zero::zero() {
				return Ok(())
			}
			if *_token == MyAccountIdDefault::<T>::get() {
				let _ = <T as exchange_common::pallet::Config>::Currency::transfer(
					&_from,
					&_to,
					_amount,
					frame_support::traits::ExistenceRequirement::AllowDeath,
				);
				return Self::transfer_native_tokens(&_from, _to, _amount)
			}
			let value: BalanceOfC<T> = Default::default();
			let gas_limit: Weight = 20000000000;
			let mut selector: Vec<u8> = vec![0x55, 0x7e, 0xfb, 0x0c];
			let mut selectors: Vec<u8> = vec![0x0b, 0x39, 0x6f, 0x18];
			let mut callees_enc: Vec<u8> = vec![_token].encode();
			let mut from_enc: Vec<u8> = _from.encode();
			let mut to_enc: Vec<u8> = _to.encode();
			let mut values_enc: Vec<u8> = vec![_amount].encode();
			let mut data = Vec::new();
			data.append(&mut selector);
			data.append(&mut selectors);
			data.append(&mut callees_enc);
			data.append(&mut from_enc);
			data.append(&mut to_enc);
			data.append(&mut values_enc);
			use sp_std::if_std;
			if_std! {
				println!("The data_encode. is: {:?}",data);
			}
			// Do the actual call to the smart contract function
			let _r = pallet_contracts::Pallet::<T>::bare_call(
				msg_sender.clone(),
				TokenTransferProxy::<T>::get().clone(),
				value,
				gas_limit,
				None,
				data,
				true,
			)
			.result;
			if_std! {
				println!("The call_smart_contracts. is: {:?}",_r);
			}

			Ok(())
		}

		// Transfer tokens
		// token Token to transfer
		// from AccountId to charge fees
		// to AccountId to receive fees
		// amount Amount of protocol tokens to charge
		pub fn transfer_native_tokens(
			_from: &AccountIdOf<T>,
			_to: &AccountIdOf<T>,
			_amount: BalanceOf<T>,
		) -> Result<(), Error<T>> {
			if _amount > Zero::zero() {
				let _ = <T as exchange_common::pallet::Config>::Currency::transfer(
					&_from,
					&_to,
					_amount,
					frame_support::traits::ExistenceRequirement::AllowDeath,
				);
			}
			Ok(())
		}

		pub fn transfer_tokens_fee(
			msg_sender: &AccountIdOf<T>,
			_token: &AccountIdOf<T>,
			_from: &AccountIdOf<T>,
			_to: &AccountIdOf<T>,
			_amount: BalanceOf<T>,
			_price: &BalanceOf<T>,
		) -> Result<(), Error<T>> {
			if _amount > Zero::zero() {
				let _amount = _amount * *_price / INVERSE_BASIS_POINT.into();
				Self::transfer_tokens(&msg_sender, _token, _from, _to, _amount)?;
			}
			Ok(())
		}

		pub fn transfer_tokens_fee_sell(
			msg_sender: &AccountIdOf<T>,
			_token: &AccountIdOf<T>,
			_from: &AccountIdOf<T>,
			_to: &AccountIdOf<T>,
			_amount: BalanceOf<T>,
			_price: &BalanceOf<T>,
			receive_or_required_amount: &mut BalanceOf<T>,
			is_maker: bool,
		) -> Result<(), Error<T>> {
			if _amount > Zero::zero() {
				let _fee = _amount * *_price / INVERSE_BASIS_POINT.into();
				let mut _from_ = (*_from).clone();
				if *_token == MyAccountIdDefault::<T>::get() {
					if is_maker {
						*receive_or_required_amount -= _fee;
					} else {
						*receive_or_required_amount += _fee;
					};
					_from_ = ContractSelf::<T>::get();
					Self::transfer_native_tokens(&_from_, _to, _amount)?;
				} else {
					Self::transfer_tokens(&msg_sender, _token, &_from_, _to, _amount)?;
				}
			}
			Ok(())
		}

		// Charge a fee in protocol tokens
		// from AccountId to charge fees
		// to AccountId to receive fees
		// amount Amount of protocol tokens to charge
		pub fn charge_protocol_fee(
			msg_sender: &AccountIdOf<T>,
			from: &AccountIdOf<T>,
			to: &AccountIdOf<T>,
			amount: BalanceOf<T>,
		) -> Result<(), Error<T>> {
			Self::transfer_tokens(&msg_sender, &ExchangeToken::<T>::get(), &from, &to, amount)
		}

		// Hash an order, returning the canonical order hash, without the message prefix
		// order OrderType to hash
		// Hash of order
		pub fn hash_order(
			order: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
		) -> Result<Vec<u8>, Error<T>> {
			Ok(keccak_256(&order.encode()).into())
		}

		// Hash an order, returning the hash that a client must sign,
		// including the standard message prefix
		// order OrderType to hash
		// Hash of  order hash per Polkadot format
		pub fn hash_to_sign(
			order: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
		) -> Result<Vec<u8>, Error<T>> {
			Ok(keccak_256(&Self::hash_order(&order)?).to_vec())
		}

		// Assert an order is valid and return its hash
		// order OrderType to validate
		// sig  signature
		pub fn require_valid_order(
			order: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
			sig: &[u8],
		) -> Result<Vec<u8>, Error<T>> {
			let hash: Vec<u8> = Self::hash_to_sign(&order)?;
			ensure!(Self::validate_order(&hash, order, sig)?, Error::<T>::InvalidOrderHash);
			Ok(hash)
		}

		// Validate order parameters (does *not* check validity:signature)
		// order OrderType to validate
		pub fn validate_order_parameters(
			order: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
		) -> bool {
			use sp_std::if_std;
			// OrderType must be targeted at this protocol version (this contract:Exchange).
			if order.exchange != ContractSelf::<T>::get() {
				if_std! {
					println!("The buy.order.exchange != ContractSelf::<T>::get()  is: {:?},=={:?}", order.exchange,ContractSelf::<T>::get());
				}
				return false
			}

			// OrderType must possess valid sale kind parameter combination.
			if !<sale_kind_interface::Pallet<T>>::validate_parameters(
				&order.sale_kind,
				order.expiration_time,
			) {
				if_std! {
					println!("The order.sale_kind,order.expiration_time is: {:?},=={:?}", order.sale_kind,order.expiration_time);

				}
				return false
			}

			// If using the split fee method, order must have sufficient protocol fees.
			if order.fee_method == FeeMethod::SplitFee &&
				(order.maker_protocol_fee < MinimumMakerProtocolFee::<T>::get() ||
					order.taker_protocol_fee < MinimumTakerProtocolFee::<T>::get())
			{
				if_std! {
					println!("The order.fee_method,FeeMethod::SplitFeeis: {},{:?},=={:?}", order.fee_method == FeeMethod::SplitFee,order.fee_method,FeeMethod::SplitFee);
					println!("The order.maker_protocol_fee,MinimumMakerProtocolFee::<T>::get() is:{}, {:?},=={:?}",order.maker_protocol_fee < MinimumMakerProtocolFee::<T>::get(), order.maker_protocol_fee,MinimumMakerProtocolFee::<T>::get());
					println!("Theorder.taker_protocol_fee,MinimumTakerProtocolFee::<T>::get()is:{}, {:?},=={:?}",order.taker_protocol_fee < MinimumTakerProtocolFee::<T>::get(), order.taker_protocol_fee,MinimumTakerProtocolFee::<T>::get());
				}
				return false
			}

			true
		}

		// Validate a provided previously approved / signed order, hash, and signature.
		// hash OrderType hash (calculated:already, passed to recalculation:avoid)
		// order OrderType to validate
		// sig  signature
		pub fn validate_order(
			hash: &[u8],
			order: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
			sig: &[u8],
		) -> Result<bool, Error<T>> {
			// OrderType must have valid parameters.
			if !Self::validate_order_parameters(&order) {
				return Ok(false)
			}

			// OrderType must have not been canceled or already filled.
			if <CancelledOrFinalized<T>>::get(hash) {
				return Ok(false)
			}

			// OrderType authentication. OrderType must be either:
			// (a) previously approved
			if <ApprovedOrders<T>>::get(hash) {
				return Ok(true)
			}

			if Self::check_signature_bytes(&sig, &hash, &order.maker).is_ok() {
				return Ok(true)
			}

			Ok(false)
		}

		// An alterantive way to validate a signature is:
		// function to verify the signature.
		pub fn check_signature_bytes(
			_signature: &[u8],
			_msg: &[u8],
			_signer: &AccountIdOf<T>,
		) -> Result<(), Error<T>> {
			// sr25519 always expects a 64 byte signature.
			ensure!(_signature.len() == 64, Error::<T>::InvalidSignature);
			let signature: Signature =sp_core::sr25519::Signature::from_slice(_signature).unwrap().into();

			// In Polkadot, the AccountId is always the same as the 32 byte public key.
			let account_bytes: [u8; 32] = account_to_bytes(_signer)?;
			// let public_key = sr25519::Public::from_raw(account_bytes);

			// Check if everything is good or not.
			match signature.verify(_msg, &account_bytes.into()) {
				true => Ok(()),
				false => Err(Error::<T>::MsgVerifyFailed)?,
			}
		}

		// Approve an order and optionally mark it for orderbook inclusion.
		// Must be called by the maker of the order
		// order OrderType to approve
		// orderbook_inclusion_desired Whether orderbook providers should include the order
		// in their orderbooks
		pub fn approve_order(
			origin: <T as frame_system::Config>::Origin,
			order: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
			orderbook_inclusion_desired: bool,
		) -> DispatchResult {
			// CHECKS
			let _user = ensure_signed(origin)?;
			// Assert sender is authorized to approve order.
			ensure!(_user == order.maker, Error::<T>::OnlyMaker);

			// Calculate order hash.
			let hash: Vec<u8> = Self::hash_to_sign(&order)?;

			// Assert order has not already been approved.
			ensure!(!<ApprovedOrders<T>>::get(hash.clone()), Error::<T>::OrderHashExists);

			// EFFECTS
			// Mark order as approved.
			<ApprovedOrders<T>>::insert(hash.clone(), true);

			// Log approval event. Must be split in two due to Solidity stack size limitations.
			Self::deposit_event(Event::OrderApprovedPartOne(
				hash.clone(),
				order.exchange.clone(),
				order.maker.clone(),
				order.taker.clone(),
				order.maker_relayer_fee,
				order.taker_relayer_fee,
				order.maker_protocol_fee,
				order.taker_protocol_fee,
				order.fee_recipient.clone(),
				order.fee_method.clone(),
				order.side.clone(),
				order.sale_kind.clone(),
				order.target.clone(),
			));

			Self::deposit_event(Event::OrderApprovedPartTwo(
				hash.clone(),
				order.how_to_call.clone(),
				order.calldata.clone(),
				order.replacement_pattern.clone(),
				order.static_target.clone(),
				order.static_extradata.clone(),
				order.payment_token.clone(),
				order.base_price.clone(),
				order.extra.clone(),
				order.listing_time.clone(),
				order.expiration_time.clone(),
				order.salt.clone(),
				orderbook_inclusion_desired,
			));
			Ok(())
		}

		// Cancel an order, preventing it from being matched. Must be called by the maker of the
		// order order OrderType to cancel
		// sig  signature
		pub fn cancel_order(
			origin: <T as frame_system::Config>::Origin,
			order: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
			sig: &[u8],
		) -> DispatchResult {
			// CHECKS
			let _user = ensure_signed(origin)?;

			// Assert sender is authorized to cancel order.
			ensure!(_user == order.maker, Error::<T>::OnlyMaker);

			// Calculate order hash.
			let hash = Self::require_valid_order(order, sig)?;
			// EFFECTS
			// Mark order as cancelled, preventing it from being matched.
			<CancelledOrFinalized<T>>::insert(hash.clone(), true);

			// Log cancel event.
			Self::deposit_event(Event::OrderCancelled(hash.clone()));

			Ok(())
		}

		// Calculate the current price of an order (fn:convenience)
		// order OrderType to calculate the price of
		// The current price of the order
		pub fn calculate_current_price(
			order: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
		) -> Result<BalanceOf<T>, Error<T>> {
			Ok(<sale_kind_interface::Pallet<T>>::calculate_final_price(
				&order.side,
				&order.sale_kind,
				order.base_price,
				order.extra,
				order.listing_time,
				order.expiration_time,
			))
		}

		// Calculate the price two orders would match at, if in fact they would match
		// (fail:otherwise) buy Buy-side order
		// sell Sell-side order
		// Match price
		pub fn calculate_match_price(
			buy: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
			sell: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
		) -> Result<BalanceOf<T>, Error<T>> {
			// Calculate sell price.
			let sell_price: BalanceOf<T> = <sale_kind_interface::Pallet<T>>::calculate_final_price(
				&sell.side,
				&sell.sale_kind,
				sell.base_price,
				sell.extra,
				sell.listing_time,
				sell.expiration_time,
			);

			// Calculate buy price.
			let buy_price: BalanceOf<T> = <sale_kind_interface::Pallet<T>>::calculate_final_price(
				&buy.side,
				&buy.sale_kind,
				buy.base_price,
				buy.extra,
				buy.listing_time,
				buy.expiration_time,
			);

			// Require price cross.
			ensure!(buy_price >= sell_price, Error::<T>::BuyPriceLessThanSellPrice);

			// Maker/taker priority.
			let price: BalanceOf<T> =
				if sell.fee_recipient != MyAccountIdDefault::<T>::get() { sell_price } else { buy_price };

			Ok(price)
		}

		// Execute all  token / DOT transfers associated with an order match
		// (fees and buyer => transfer:seller)
		// buy Buy-side order
		// sell Sell-side order
		pub fn execute_funds_transfer(
			msg_sender: &AccountIdOf<T>,
			msg_value: BalanceOf<T>,
			buy: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
			sell: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
		) -> Result<BalanceOf<T>, Error<T>> {
			// let originprotocol_fee_recipient = ProtocolFeeRecipient::<T>::get();
			// Only payable in the special case of unwrapped DOT.
			if sell.payment_token != MyAccountIdDefault::<T>::get() {
				ensure!(msg_value == Zero::zero(), Error::<T>::ValueNotZero);
			}

			// Calculate match price.
			let price: BalanceOf<T> = Self::calculate_match_price(&buy, &sell)?;

			// If paying using a token (DOT:not), transfer tokens. This is done prior to
			// fee payments to that a seller will have tokens before being charged fees.
			if price > Zero::zero() && sell.payment_token != MyAccountIdDefault::<T>::get() {
				Self::transfer_tokens(
					&msg_sender,
					&sell.payment_token,
					&buy.maker,
					&sell.maker,
					price,
				)?;
			}

			// Amount that will be received by seller (DOT:for).
			let mut receive_amount: BalanceOf<T> = price;

			// Amount that must be sent by buyer (DOT:for).
			let mut required_amount: BalanceOf<T> = price;

			// Determine maker/taker and charge fees accordingly.
			if sell.fee_recipient != MyAccountIdDefault::<T>::get() {
				// Sell-side order is maker.
				Self::execute_funds_transfer_sell_side(
					&msg_sender,
					buy,
					sell,
					&price,
					&mut receive_amount,
					&mut required_amount,
				)?;
			} else {
				// Buy-side order is maker.
				Self::execute_funds_transfer_buy_side(&msg_sender, buy, sell, &price)?;
			}

			if sell.payment_token == MyAccountIdDefault::<T>::get() {
				// Special-case DOT, order must be matched by buyer.
				ensure!(msg_value >= required_amount, Error::<T>::ValueLessThanRequiredAmount);
				// sell.maker.transfer(receive_amount);
				Self::transfer_native_tokens(
					&ContractSelf::<T>::get(),
					&sell.maker,
					receive_amount,
				)?;
				// Allow overshoot for variable-price auctions, refund difference.
				let diff: BalanceOf<T> = msg_value - required_amount;
				if diff > Zero::zero() {
					// buy.maker.transfer(diff);
					Self::transfer_native_tokens(&ContractSelf::<T>::get(), &buy.maker, diff)?;
				}
			}

			// This contract should never hold DOT, however, we cannot assert this,
			// since it is impossible to prevent anyone from sending DOT e.g. with selfdestruct.
			Ok(price)
		}

		// Execute all  token / DOT transfers associated with an order match
		// (fees and buyer => transfer:seller)
		// buy Buy-side order
		// sell Sell-side order
		pub fn execute_funds_transfer_sell_side(
			msg_sender: &AccountIdOf<T>,
			buy: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
			sell: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
			price: &BalanceOf<T>,
			receive_amount: &mut BalanceOf<T>,
			required_amount: &mut BalanceOf<T>,
		) -> Result<BalanceOf<T>, Error<T>> {
			let originprotocol_fee_recipient = ProtocolFeeRecipient::<T>::get();

			// Determine maker/taker and charge fees accordingly.
			// Sell-side order is maker.
			// Assert taker fee is less than or equal to maximum fee specified by buyer.
			ensure!(
				sell.taker_relayer_fee <= buy.taker_relayer_fee,
				Error::<T>::SellTakerRelayerFeeGreaterThanBuyTakerRelayerFee
			);

			if sell.fee_method == FeeMethod::SplitFee {
				// Assert taker fee is less than or equal to maximum fee specified by buyer.
				ensure!(
					sell.taker_protocol_fee <= buy.taker_protocol_fee,
					Error::<T>::SellTakerProtocolFeeGreaterThanBuyTakerProtocolFee
				);

				// Maker fees are deducted from the token amount that the maker receives.
				// Taker fees are extra tokens that must be paid by the taker.
				Self::transfer_tokens_fee_sell(
					&msg_sender,
					&sell.payment_token,
					&sell.maker,
					&sell.fee_recipient,
					sell.maker_relayer_fee,
					price,
					receive_amount,
					true,
				)?;

				Self::transfer_tokens_fee_sell(
					&msg_sender,
					&sell.payment_token,
					&buy.maker,
					&sell.fee_recipient,
					sell.taker_relayer_fee,
					price,
					required_amount,
					false,
				)?;

				Self::transfer_tokens_fee_sell(
					&msg_sender,
					&sell.payment_token,
					&sell.maker,
					&originprotocol_fee_recipient,
					sell.maker_protocol_fee,
					price,
					receive_amount,
					true,
				)?;

				Self::transfer_tokens_fee_sell(
					&msg_sender,
					&sell.payment_token,
					&buy.maker,
					&originprotocol_fee_recipient,
					sell.taker_protocol_fee,
					price,
					required_amount,
					false,
				)?;
			} else {
				// Charge maker fee to seller.
				Self::charge_protocol_fee(
					&msg_sender,
					&sell.maker,
					&sell.fee_recipient,
					sell.maker_relayer_fee,
				)?;

				// Charge taker fee to buyer.
				Self::charge_protocol_fee(
					&msg_sender,
					&buy.maker,
					&sell.fee_recipient,
					sell.taker_relayer_fee,
				)?;
			}

			// This contract should never hold token, however, we cannot assert this,
			// since it is impossible to prevent anyone from sending DOT e.g. with selfdestruct.
			Ok(*price)
		}

		// Execute all ERC20 token / DOT transfers associated with an order match
		// (fees and buyer => transfer:seller)
		// buy Buy-side order
		// sell Sell-side order
		pub fn execute_funds_transfer_buy_side(
			msg_sender: &AccountIdOf<T>,
			buy: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
			sell: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
			price: &BalanceOf<T>,
		) -> Result<BalanceOf<T>, Error<T>> {
			let originprotocol_fee_recipient = ProtocolFeeRecipient::<T>::get();

			// Determine maker/taker and charge fees accordingly.
			// Buy-side order is maker.

			// Assert taker fee is less than or equal to maximum fee specified by seller.
			ensure!(
				buy.taker_relayer_fee <= sell.taker_relayer_fee,
				Error::<T>::BuyTakerRelayerFeeGreaterThanSellTakerRelayerFee
			);

			if sell.fee_method == FeeMethod::SplitFee {
				// The Exchange does not escrow DOT, so direct DOT can only be used to with
				// sell-side maker / buy-side taker orders.
				ensure!(
					sell.payment_token != MyAccountIdDefault::<T>::get(),
					Error::<T>::SellPaymentTokenEqualPaymentToken
				);

				// Assert taker fee is less than or equal to maximum fee specified by seller.
				ensure!(
					buy.taker_protocol_fee <= sell.taker_protocol_fee,
					Error::<T>::BuyTakerProtocolFeeGreaterThanSellTakerProtocolFee
				);

				Self::transfer_tokens_fee(
					&msg_sender,
					&sell.payment_token,
					&buy.maker,
					&buy.fee_recipient,
					buy.maker_relayer_fee,
					price,
				)?;

				Self::transfer_tokens_fee(
					&msg_sender,
					&sell.payment_token,
					&sell.maker,
					&buy.fee_recipient,
					buy.taker_relayer_fee,
					price,
				)?;

				Self::transfer_tokens_fee(
					&msg_sender,
					&sell.payment_token,
					&buy.maker,
					&originprotocol_fee_recipient,
					buy.maker_protocol_fee,
					price,
				)?;

				Self::transfer_tokens_fee(
					&msg_sender,
					&sell.payment_token,
					&sell.maker,
					&originprotocol_fee_recipient,
					buy.taker_protocol_fee,
					price,
				)?;
			} else {
				// Charge maker fee to buyer.
				Self::charge_protocol_fee(
					&msg_sender,
					&buy.maker,
					&buy.fee_recipient,
					buy.maker_relayer_fee,
				)?;

				// Charge taker fee to seller.
				Self::charge_protocol_fee(
					&msg_sender,
					&sell.maker,
					&buy.fee_recipient,
					buy.taker_relayer_fee,
				)?;
			}

			// This contract should never hold DOT, however, we cannot assert this,
			// since it is impossible to prevent anyone from sending DOT e.g. with selfdestruct.
			Ok(*price)
		}

		// Return whether or not two orders can be matched with each other by basic parameters
		// (does not check order signatures / calldata or perform calls:static)
		// buy Buy-side order
		// sell Sell-side order
		// Whether or not the two orders can be matched
		pub fn orders_can_match(
			buy: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
			sell: &OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
		) -> bool {
			//  Must be opposite-side.
			(buy.side == Side::Buy && sell.side == Side::Sell) &&
            // Must use same fee method.
            (buy.fee_method == sell.fee_method) &&
            // Must use same payment token. 
            (buy.payment_token == sell.payment_token) &&
            // Must match maker/taker addresses. 
            (sell.taker == MyAccountIdDefault::<T>::get() || sell.taker == buy.maker) &&
            (buy.taker == MyAccountIdDefault::<T>::get() || buy.taker == sell.maker) &&
            // One must be maker and the other must be taker (no bool XOR Solidity:in). 
            ((sell.fee_recipient == MyAccountIdDefault::<T>::get() &&
            buy.fee_recipient != MyAccountIdDefault::<T>::get()) ||
            (sell.fee_recipient != MyAccountIdDefault::<T>::get() &&
            buy.fee_recipient == MyAccountIdDefault::<T>::get())) &&
            // Must match target. 
            (buy.target == sell.target) &&
            // Must match how_to_call. 
            (buy.how_to_call == sell.how_to_call) &&
            // Buy-side order must be settleable. 
            <sale_kind_interface::Pallet<T>>::can_settle_order(
                buy.listing_time,
                buy.expiration_time,
            ) &&
            // Sell-side order must be settleable. 
            <sale_kind_interface::Pallet<T>>::can_settle_order(
                sell.listing_time,
                sell.expiration_time,
            )
		}

		// Atomically match two orders, ensuring validity of the match,
		// and execute all associated state transitions.
		// buy Buy-side order
		// buy_sig Buy-side order signature
		// sell Sell-side order
		// sell_sig Sell-side order signature
		pub fn atomic_match(
			msg_sender: AccountIdOf<T>,
			msg_value: BalanceOf<T>,
			buy: OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
			buy_sig: Vec<u8>,
			sell: OrderType<AccountIdOf<T>, T::Moment, BalanceOf<T>>,
			sell_sig: Vec<u8>,
			metadata: &[u8],
		) -> DispatchResult where <T as pallet_contracts::Config>::Currency: Currency<<T as frame_system::Config>::AccountId> {
			use sp_std::if_std;
			if_std! {
				println!("The atomic_match is: {:?}", buy.calldata);
			}

			// Ensure buy order validity and calculate hash if necessary.
			let mut buy_hash: Vec<u8> = vec![];
			if buy.maker == msg_sender {
				if !Self::validate_order_parameters(&buy) {
					return Err(Error::<T>::InvalidBuyOrderParameters.into())
				}
			} else {
				buy_hash = Self::require_valid_order(&buy, &buy_sig)?;
			}

			// Ensure sell order validity and calculate hash if necessary.
			let mut sell_hash: Vec<u8> = vec![];
			if sell.maker == msg_sender {
				if !Self::validate_order_parameters(&sell) {
					return Err(Error::<T>::InvalidSellOrderParameters.into())
				}
			} else {
				sell_hash = Self::require_valid_order(&sell, &sell_sig)?;
			}

			// Must be matchable.
			if !Self::orders_can_match(&buy, &sell) {
				return Err(Error::<T>::OrdersCannotMatch.into())
			}

			// Must match calldata after replacement, if specified.
			let mut buycalldata = buy.calldata.clone();
			let mut sellcalldata = sell.calldata.clone();
			if buy.replacement_pattern.len() > 0 {
				if !<exchange_common::Pallet<T>>::guarded_array_replace(
					&mut buycalldata,
					&sell.calldata,
					&buy.replacement_pattern,
				) {
					return Err(Error::<T>::BuyArrayNotEqual.into())
				}
			}

			if sell.replacement_pattern.len() > 0 {
				if !<exchange_common::Pallet<T>>::guarded_array_replace(
					&mut sellcalldata,
					&buy.calldata,
					&sell.replacement_pattern,
				) {
					return Err(Error::<T>::SellArrayNotEqual.into())
				}
			}

			if !<exchange_common::Pallet<T>>::array_eq(&buycalldata, &sellcalldata) {
				return Err(Error::<T>::ArrayNotEqual.into())
			}

			// Mark previously signed or approved orders as finalized.
			let buymaker: AccountIdOf<T> = buy.maker.clone();
			if msg_sender != buymaker {
				<CancelledOrFinalized<T>>::insert(buy_hash.clone(), true);
			}
			let sellmaker: AccountIdOf<T> = sell.maker.clone();
			if msg_sender != sellmaker {
				<CancelledOrFinalized<T>>::insert(sell_hash.clone(), true);
			}

			// INTERACTIONS
			// Execute funds transfer and pay fees.
			let price: BalanceOf<T> =
				Self::execute_funds_transfer(&msg_sender, msg_value, &buy, &sell)?;

			// use sp_std::if_std;
			if_std! {
				println!("atomic_matchx={:?} ,{:?} ,The sellcalldata is: {:?}",msg_sender.clone(),sell.target.clone(), &sellcalldata);
			}
			// Check against unbounded input
			// ensure!(selector.len() < 4, Error::<T>::InputTooLarge);
			// Amount to transfer
			let value: BalanceOfC<T> = Default::default();
			let gas_limit: Weight = 20000000000;
			// Do the actual call to the smart contract function
			let _r = pallet_contracts::Pallet::<T>::bare_call(
				msg_sender.clone(),
				sell.target.clone(),
				value,
				gas_limit,
				None,
				sellcalldata.clone(),
				true,
			)
			.result;
			if_std! {
				println!("The bare_call result. is: {:?}",_r);
			}
			// let message_call = pallet_contracts::Call::decode(&mut &buy.calldata[..]).map_err(|_|
			// {});

			//  if_std! {
			//             println!("My message_call is: {:#?}", message_call);
			//         }

			// Log match event.
			Self::deposit_event(Event::OrdersMatched(
				buy_hash.clone(),
				sell_hash.clone(),
				if sell.fee_recipient != MyAccountIdDefault::<T>::get() {
					sell.maker.clone()
				} else {
					buy.maker.clone()
				},
				if sell.fee_recipient != MyAccountIdDefault::<T>::get() {
					buy.maker.clone()
				} else {
					sell.maker.clone()
				},
				price,
				metadata.to_vec(),
			));

			Ok(())
		}
		// }
	}
	// This function converts a 32 byte AccountId to its byte-array equivalent form.
	fn account_to_bytes<AccountId, T: Config>(account: &AccountId) -> Result<[u8; 32], Error<T>>
	where
		AccountId: Encode,
	{
		let account_vec = account.encode();

		ensure!(account_vec.len() == 32, Error::<T>::InvalidSignature);
		let mut bytes = [0u8; 32];
		bytes.copy_from_slice(&account_vec);
		Ok(bytes)
	}
}
