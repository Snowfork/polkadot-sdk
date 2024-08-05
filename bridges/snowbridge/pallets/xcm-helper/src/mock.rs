// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>

use codec::Encode;
use frame_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{
		AsEnsureOriginWithArg, ConstU128, ConstU32, Contains, Equals, Everything, EverythingBut,
		Nothing,
	},
	weights::Weight,
};
use frame_system::EnsureRoot;
use polkadot_parachain_primitives::primitives::Id as ParaId;
use polkadot_runtime_parachains::origin;
use sp_core::H256;
use sp_runtime::{traits::IdentityLookup, AccountId32, BuildStorage};
pub use sp_std::cell::RefCell;
use xcm::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowSubscriptionsFrom, AllowTopLevelPaidExecutionFrom, Case,
	ChildParachainAsNative, ChildParachainConvertsVia, ChildSystemParachainAsSuperuser,
	DescribeAllTerminal, FixedRateOfFungible, FixedWeightBounds, FrameTransactionalProcessor,
	FungibleAdapter, FungiblesAdapter, HashedDescription, IsConcrete, MatchedConvertedConcreteId,
	NoChecking, SendXcmFeeToAccount, SignedAccountId32AsNative, SignedToAccountId32,
	SovereignSignedViaLocation, TakeWeightCredit, XcmFeeManagerFromComponents,
};
use xcm_executor::{
	traits::{Identity, JustTry},
	XcmExecutor,
};

use crate::Config;

pub type AccountId = AccountId32;
pub type Balance = u128;
type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		AssetsPallet: pallet_assets,
		ParasOrigin: origin,
		XcmHelper: crate,
	}
);

thread_local! {
	pub static SENT_XCM: RefCell<Vec<(Location, Xcm<()>)>> = RefCell::new(Vec::new());
	pub static FAIL_SEND_XCM: RefCell<bool> = RefCell::new(false);
}
pub(crate) fn sent_xcm() -> Vec<(Location, Xcm<()>)> {
	SENT_XCM.with(|q| (*q.borrow()).clone())
}
pub(crate) fn take_sent_xcm() -> Vec<(Location, Xcm<()>)> {
	SENT_XCM.with(|q| {
		let mut r = Vec::new();
		std::mem::swap(&mut r, &mut *q.borrow_mut());
		r
	})
}
pub(crate) fn set_send_xcm_artificial_failure(should_fail: bool) {
	FAIL_SEND_XCM.with(|q| *q.borrow_mut() = should_fail);
}
/// Sender that never returns error.
pub struct TestSendXcm;
impl SendXcm for TestSendXcm {
	type Ticket = (Location, Xcm<()>);
	fn validate(
		dest: &mut Option<Location>,
		msg: &mut Option<Xcm<()>>,
	) -> SendResult<(Location, Xcm<()>)> {
		if FAIL_SEND_XCM.with(|q| *q.borrow()) {
			return Err(SendError::Transport("Intentional send failure used in tests"));
		}
		let pair = (dest.take().unwrap(), msg.take().unwrap());
		Ok((pair, Assets::new()))
	}
	fn deliver(pair: (Location, Xcm<()>)) -> Result<XcmHash, SendError> {
		let hash = fake_message_hash(&pair.1);
		SENT_XCM.with(|q| q.borrow_mut().push(pair));
		Ok(hash)
	}
}
/// Sender that returns error if `X8` junction and stops routing
pub struct TestSendXcmErrX8;
impl SendXcm for TestSendXcmErrX8 {
	type Ticket = (Location, Xcm<()>);
	fn validate(
		dest: &mut Option<Location>,
		_: &mut Option<Xcm<()>>,
	) -> SendResult<(Location, Xcm<()>)> {
		if dest.as_ref().unwrap().len() == 8 {
			dest.take();
			Err(SendError::Transport("Destination location full"))
		} else {
			Err(SendError::NotApplicable)
		}
	}
	fn deliver(pair: (Location, Xcm<()>)) -> Result<XcmHash, SendError> {
		let hash = fake_message_hash(&pair.1);
		SENT_XCM.with(|q| q.borrow_mut().push(pair));
		Ok(hash)
	}
}

parameter_types! {
	pub Para3000: u32 = 3000;
	pub Para3000Location: Location = Parachain(Para3000::get()).into();
	pub Para3000PaymentAmount: u128 = 1;
	pub Para3000PaymentAssets: Assets = Assets::from(Asset::from((Here, Para3000PaymentAmount::get())));
}
/// Sender only sends to `Parachain(3000)` destination requiring payment.
pub struct TestPaidForPara3000SendXcm;
impl SendXcm for TestPaidForPara3000SendXcm {
	type Ticket = (Location, Xcm<()>);
	fn validate(
		dest: &mut Option<Location>,
		msg: &mut Option<Xcm<()>>,
	) -> SendResult<(Location, Xcm<()>)> {
		if let Some(dest) = dest.as_ref() {
			if !dest.eq(&Para3000Location::get()) {
				return Err(SendError::NotApplicable)
			}
		} else {
			return Err(SendError::NotApplicable)
		}

		let pair = (dest.take().unwrap(), msg.take().unwrap());
		Ok((pair, Para3000PaymentAssets::get()))
	}
	fn deliver(pair: (Location, Xcm<()>)) -> Result<XcmHash, SendError> {
		let hash = fake_message_hash(&pair.1);
		SENT_XCM.with(|q| q.borrow_mut().push(pair));
		Ok(hash)
	}
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = Everything;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub ExistentialDeposit: Balance = 1;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type FreezeIdentifier = ();
	type MaxFreezes = ConstU32<0>;
}

#[cfg(feature = "runtime-benchmarks")]
/// Simple conversion of `u32` into an `AssetId` for use in benchmarking.
pub struct XcmBenchmarkHelper;
#[cfg(feature = "runtime-benchmarks")]
impl pallet_assets::BenchmarkHelper<Location> for XcmBenchmarkHelper {
	fn create_asset_id_parameter(id: u32) -> Location {
		Location::new(1, [Parachain(id)])
	}
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = Location;
	type AssetIdParameter = Location;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = ConstU128<1>;
	type AssetAccountDeposit = ConstU128<10>;
	type MetadataDepositBase = ConstU128<1>;
	type MetadataDepositPerByte = ConstU128<1>;
	type ApprovalDeposit = ConstU128<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type WeightInfo = ();
	type CallbackHandle = ();
	type Extra = ();
	type RemoveItemsLimit = ConstU32<5>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = XcmBenchmarkHelper;
}

// This child parachain is a system parachain trusted to teleport native token.
pub const SOME_SYSTEM_PARA: u32 = 1001;

// This child parachain acts as trusted reserve for its assets in tests.
// USDT allowed to teleport to/from here.
pub const FOREIGN_ASSET_RESERVE_PARA_ID: u32 = 2001;
// Inner junction of reserve asset on `FOREIGN_ASSET_RESERVE_PARA_ID`.
pub const FOREIGN_ASSET_INNER_JUNCTION: Junction = GeneralIndex(1234567);

// This child parachain acts as trusted reserve for say.. USDC that can be used for fees.
pub const USDC_RESERVE_PARA_ID: u32 = 2002;
// Inner junction of reserve asset on `USDC_RESERVE_PARA_ID`.
pub const USDC_INNER_JUNCTION: Junction = PalletInstance(42);

// This child parachain is a trusted teleporter for say.. USDT (T from Teleport :)).
// We'll use USDT in tests that teleport fees.
pub const USDT_PARA_ID: u32 = 2003;

// This child parachain is not configured as trusted reserve or teleport location for any assets.
pub const OTHER_PARA_ID: u32 = 2009;

// This child parachain is used for filtered/disallowed assets.
pub const FILTERED_PARA_ID: u32 = 2010;

parameter_types! {
	pub const RelayLocation: Location = Here.into_location();
	pub const NativeAsset: Asset = Asset {
		fun: Fungible(10),
		id: AssetId(Here.into_location()),
	};
	pub SystemParachainLocation: Location = Location::new(
		0,
		[Parachain(SOME_SYSTEM_PARA)]
	);
	pub ForeignReserveLocation: Location = Location::new(
		0,
		[Parachain(FOREIGN_ASSET_RESERVE_PARA_ID)]
	);
	pub PaidParaForeignReserveLocation: Location = Location::new(
		0,
		[Parachain(Para3000::get())]
	);
	pub ForeignAsset: Asset = Asset {
		fun: Fungible(10),
		id: AssetId(Location::new(
			0,
			[Parachain(FOREIGN_ASSET_RESERVE_PARA_ID), FOREIGN_ASSET_INNER_JUNCTION],
		)),
	};
	pub PaidParaForeignAsset: Asset = Asset {
		fun: Fungible(10),
		id: AssetId(Location::new(
			0,
			[Parachain(Para3000::get())],
		)),
	};
	pub UsdcReserveLocation: Location = Location::new(
		0,
		[Parachain(USDC_RESERVE_PARA_ID)]
	);
	pub Usdc: Asset = Asset {
		fun: Fungible(10),
		id: AssetId(Location::new(
			0,
			[Parachain(USDC_RESERVE_PARA_ID), USDC_INNER_JUNCTION],
		)),
	};
	pub UsdtTeleportLocation: Location = Location::new(
		0,
		[Parachain(USDT_PARA_ID)]
	);
	pub Usdt: Asset = Asset {
		fun: Fungible(10),
		id: AssetId(Location::new(
			0,
			[Parachain(USDT_PARA_ID)],
		)),
	};
	pub FilteredTeleportLocation: Location = Location::new(
		0,
		[Parachain(FILTERED_PARA_ID)]
	);
	pub FilteredTeleportAsset: Asset = Asset {
		fun: Fungible(10),
		id: AssetId(Location::new(
			0,
			[Parachain(FILTERED_PARA_ID)],
		)),
	};
	pub const AnyNetwork: Option<NetworkId> = None;
	pub UniversalLocation: InteriorLocation = Here;
	pub UnitWeightCost: u64 = 1_000;
	pub CheckingAccount: AccountId = AccountId::from([1u8; 32]);
}

pub type SovereignAccountOf = (
	ChildParachainConvertsVia<ParaId, AccountId>,
	AccountId32Aliases<AnyNetwork, AccountId>,
	HashedDescription<AccountId, DescribeAllTerminal>,
);

pub type ForeignAssetsConvertedConcreteId = MatchedConvertedConcreteId<
	Location,
	Balance,
	// Excludes relay/parent chain currency
	EverythingBut<(Equals<RelayLocation>,)>,
	Identity,
	JustTry,
>;

pub type AssetTransactors = (
	FungibleAdapter<Balances, IsConcrete<RelayLocation>, SovereignAccountOf, AccountId, ()>,
	FungiblesAdapter<
		AssetsPallet,
		ForeignAssetsConvertedConcreteId,
		SovereignAccountOf,
		AccountId,
		NoChecking,
		CheckingAccount,
	>,
);

type LocalOriginConverter = (
	SovereignSignedViaLocation<SovereignAccountOf, RuntimeOrigin>,
	ChildParachainAsNative<origin::Origin, RuntimeOrigin>,
	SignedAccountId32AsNative<AnyNetwork, RuntimeOrigin>,
	ChildSystemParachainAsSuperuser<ParaId, RuntimeOrigin>,
);

parameter_types! {
	pub const BaseXcmWeight: Weight = Weight::from_parts(1_000, 1_000);
	pub CurrencyPerSecondPerByte: (AssetId, u128, u128) = (AssetId(RelayLocation::get()), 1, 1);
	pub TrustedLocal: (AssetFilter, Location) = (All.into(), Here.into());
	pub TrustedSystemPara: (AssetFilter, Location) = (NativeAsset::get().into(), SystemParachainLocation::get());
	pub TrustedUsdt: (AssetFilter, Location) = (Usdt::get().into(), UsdtTeleportLocation::get());
	pub TrustedFilteredTeleport: (AssetFilter, Location) = (FilteredTeleportAsset::get().into(), FilteredTeleportLocation::get());
	pub TeleportUsdtToForeign: (AssetFilter, Location) = (Usdt::get().into(), ForeignReserveLocation::get());
	pub TrustedForeign: (AssetFilter, Location) = (ForeignAsset::get().into(), ForeignReserveLocation::get());
	pub TrustedPaidParaForeign: (AssetFilter, Location) = (PaidParaForeignAsset::get().into(), PaidParaForeignReserveLocation::get());

	pub TrustedUsdc: (AssetFilter, Location) = (Usdc::get().into(), UsdcReserveLocation::get());
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsIntoHolding: u32 = 64;
	pub TreasuryAccount: AccountId = AccountId::new([167u8; 32]);
}

pub const XCM_FEES_NOT_WAIVED_USER_ACCOUNT: [u8; 32] = [37u8; 32];

pub struct XcmFeesNotWaivedLocations;
impl Contains<Location> for XcmFeesNotWaivedLocations {
	fn contains(location: &Location) -> bool {
		matches!(
			location.unpack(),
			(0, [Junction::AccountId32 { network: None, id: XCM_FEES_NOT_WAIVED_USER_ACCOUNT }])
		)
	}
}

pub type Barrier = (
	TakeWeightCredit,
	AllowTopLevelPaidExecutionFrom<Everything>,
	AllowSubscriptionsFrom<Everything>,
);

pub type XcmRouter = (TestPaidForPara3000SendXcm, TestSendXcmErrX8, TestSendXcm);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = AssetTransactors;
	type OriginConverter = LocalOriginConverter;
	type IsReserve = (Case<TrustedForeign>, Case<TrustedUsdc>, Case<TrustedPaidParaForeign>);
	type IsTeleporter = (
		Case<TrustedLocal>,
		Case<TrustedSystemPara>,
		Case<TrustedUsdt>,
		Case<TeleportUsdtToForeign>,
		Case<TrustedFilteredTeleport>,
	);
	type UniversalLocation = UniversalLocation;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
	type Trader = FixedRateOfFungible<CurrencyPerSecondPerByte, ()>;
	type ResponseHandler = ();
	type AssetTrap = ();
	type AssetLocker = ();
	type AssetExchanger = ();
	type AssetClaims = ();
	type SubscriptionService = ();
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
	type FeeManager = XcmFeeManagerFromComponents<
		EverythingBut<XcmFeesNotWaivedLocations>,
		SendXcmFeeToAccount<Self::AssetTransactor, TreasuryAccount>,
	>;
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = Everything;
	type Aliasers = Nothing;
	type TransactionalProcessor = FrameTransactionalProcessor;
	type HrmpNewChannelOpenRequestHandler = ();
	type HrmpChannelAcceptedHandler = ();
	type HrmpChannelClosingHandler = ();
	type XcmRecorder = ();
}

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, AnyNetwork>;

parameter_types! {
	pub EthereumLocation: Location = Location {
				parents: 2,
				interior: Junctions::from([GlobalConsensus(Ethereum { chain_id: 11155111 })]),
	};
	pub BridgeHub: Location = Location { parents:1, interior: Junctions::from([Parachain(1013)])};
	pub DeliveryFee: Asset = Asset::from((Location::parent(),48_000_000));
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
	type UniversalLocation = UniversalLocation;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Destination = EthereumLocation;
	type DeliveryFee = DeliveryFee;
	type Forwarder = BridgeHub;
}

impl origin::Config for Test {}

#[cfg(feature = "runtime-benchmarks")]
pub struct TestDeliveryHelper;
#[cfg(feature = "runtime-benchmarks")]
impl xcm_builder::EnsureDelivery for TestDeliveryHelper {
	fn ensure_successful_delivery(
		origin_ref: &Location,
		_dest: &Location,
		_fee_reason: xcm_executor::traits::FeeReason,
	) -> (Option<xcm_executor::FeesMode>, Option<Assets>) {
		use xcm_executor::traits::ConvertLocation;
		let account = SovereignAccountOf::convert_location(origin_ref).expect("Valid location");
		// Give the existential deposit at least
		let balance = ExistentialDeposit::get();
		let _ = <Balances as frame_support::traits::Currency<_>>::make_free_balance_be(
			&account, balance,
		);
		(None, None)
	}
}

pub(crate) fn last_event() -> RuntimeEvent {
	System::events().pop().expect("RuntimeEvent expected").event
}

pub(crate) fn last_events(n: usize) -> Vec<RuntimeEvent> {
	System::events().into_iter().map(|e| e.event).rev().take(n).rev().collect()
}

pub(crate) fn new_test_ext_with_balances(
	balances: Vec<(AccountId, Balance)>,
) -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	pallet_balances::GenesisConfig::<Test> { balances }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub(crate) fn fake_message_hash<T>(message: &Xcm<T>) -> XcmHash {
	message.using_encoded(sp_io::hashing::blake2_256)
}
