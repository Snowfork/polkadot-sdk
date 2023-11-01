// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use super::{
	AccountId, AllPalletsWithSystem, Balances, BaseDeliveryFee, FeeAssetId, ParachainInfo,
	ParachainSystem, PolkadotXcm, Runtime, RuntimeCall, RuntimeEvent, RuntimeFlavor, RuntimeOrigin,
	TransactionByteFee, WeightToFee, XcmpQueue,
};
use crate::{
	bridge_common_config::{
		BridgeGrandpaRococoInstance, BridgeGrandpaWococoInstance, DeliveryRewardInBalance,
		RequiredStakeForStakeAndSlash,
	},
	bridge_hub_rococo_config::{
		AssetHubRococoParaId, BridgeHubWococoChainId, BridgeHubWococoMessagesLane,
		ToBridgeHubWococoHaulBlobExporter, WococoGlobalConsensusNetwork,
	},
	bridge_hub_wococo_config::{
		AssetHubWococoParaId, BridgeHubRococoChainId, BridgeHubRococoMessagesLane,
		RococoGlobalConsensusNetwork, ToBridgeHubRococoHaulBlobExporter,
	},
};
use bp_messages::LaneId;
use bp_relayers::{PayRewardFromAccount, RewardsAccountOwner, RewardsAccountParams};
use bp_runtime::ChainId;
use frame_support::{
	match_types, parameter_types,
	traits::{ConstU32, Contains, Equals, Everything, Nothing},
};
use frame_system::EnsureRoot;
use pallet_xcm::XcmPassthrough;
use parachains_common::{
	impls::ToStakingPot,
	xcm_config::{ConcreteAssetFromSystem, RelayOrOtherSystemParachains},
	TREASURY_PALLET_ID,
};
use polkadot_parachain_primitives::primitives::Sibling;
use polkadot_runtime_common::xcm_sender::ExponentialPrice;
use rococo_runtime_constants::system_parachain::SystemParachains;
use snowbridge_router_primitives::outbound::EthereumBlobExporter;
use sp_core::{Get, H256};
use sp_runtime::traits::AccountIdConversion;
use sp_std::marker::PhantomData;
use xcm::latest::prelude::*;
use xcm_builder::{
	deposit_or_burn_fee, AccountId32Aliases, AllowExplicitUnpaidExecutionFrom,
	AllowKnownQueryResponses, AllowSubscriptionsFrom, AllowTopLevelPaidExecutionFrom,
	CurrencyAdapter, DenyReserveTransferToRelayChain, DenyThenTry, DescribeAllTerminal,
	DescribeFamily, EnsureXcmOrigin, HandleFee, HashedDescription, IsConcrete,
	ParentAsSuperuser, ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative,
	SiblingParachainConvertsVia, SignedAccountId32AsNative, SignedToAccountId32,
	SovereignSignedViaLocation, TakeWeightCredit, TrailingSetTopicAsId, UsingComponents,
	WeightInfoBounds, WithComputedOrigin, WithUniqueTopic, XcmFeeManagerFromComponents,
	XcmFeeToAccount,
};
use xcm_executor::{
	traits::{ExportXcm, FeeReason, TransactAsset, WithOriginFilter},
	XcmExecutor,
};

parameter_types! {
	pub storage Flavor: RuntimeFlavor = RuntimeFlavor::default();
	pub const TokenLocation: MultiLocation = MultiLocation::parent();
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub UniversalLocation: InteriorMultiLocation =
		X2(GlobalConsensus(RelayNetwork::get()), Parachain(ParachainInfo::parachain_id().into()));
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsIntoHolding: u32 = 64;
	pub TreasuryAccount: AccountId = TREASURY_PALLET_ID.into_account_truncating();
	pub RelayTreasuryLocation: MultiLocation = (Parent, PalletInstance(rococo_runtime_constants::TREASURY_PALLET_ID)).into();

	// Network and location for the local Ethereum testnet.
	pub const EthereumNetwork: NetworkId = NetworkId::Ethereum { chain_id: 15 };
	pub EthereumLocation: MultiLocation = MultiLocation::new(2, X1(GlobalConsensus(EthereumNetwork::get())));

	pub const EthereumGatewayAddress: [u8; 20] = hex_literal::hex!("EDa338E4dC46038493b885327842fD3E301CaB39");
	// The Registry contract for the bridge which is also the origin for reserves and the prefix of all assets.
	pub EthereumGatewayLocation: MultiLocation = EthereumLocation::get()
		.pushed_with_interior(
			AccountKey20 {
				network: None,
				key: EthereumGatewayAddress::get(),
			}
		).unwrap();
}

/// Adapter for resolving `NetworkId` based on `pub storage Flavor: RuntimeFlavor`.
pub struct RelayNetwork;
impl Get<Option<NetworkId>> for RelayNetwork {
	fn get() -> Option<NetworkId> {
		Some(Self::get())
	}
}
impl Get<NetworkId> for RelayNetwork {
	fn get() -> NetworkId {
		match Flavor::get() {
			RuntimeFlavor::Rococo => NetworkId::Rococo,
			RuntimeFlavor::Wococo => NetworkId::Wococo,
		}
	}
}

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the parent `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
);

/// Means for transacting the native currency on this chain.
pub type CurrencyTransactor = CurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<TokenLocation>,
	// Do a simple punn to convert an AccountId32 MultiLocation into a native chain account ID:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports of `Balances`.
	(),
>;

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will convert to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Superuser converter for the Relay-chain (Parent) location. This will allow it to issue a
	// transaction from the Root origin.
	ParentAsSuperuser<RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `RuntimeOrigin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);

match_types! {
	pub type ParentOrParentsPlurality: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(Plurality { .. }) }
	};
	pub type ParentOrSiblings: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(_) }
	};
}

/// A call filter for the XCM Transact instruction. This is a temporary measure until we properly
/// account for proof size weights.
///
/// Calls that are allowed through this filter must:
/// 1. Have a fixed weight;
/// 2. Cannot lead to another call being made;
/// 3. Have a defined proof size weight, e.g. no unbounded vecs in call parameters.
pub struct SafeCallFilter;
impl Contains<RuntimeCall> for SafeCallFilter {
	fn contains(call: &RuntimeCall) -> bool {
		#[cfg(feature = "runtime-benchmarks")]
		{
			if matches!(call, RuntimeCall::System(frame_system::Call::remark_with_event { .. })) {
				return true
			}
		}

		// Allow to change dedicated storage items (called by governance-like)
		match call {
			RuntimeCall::System(frame_system::Call::set_storage { items })
				if items.iter().all(|(k, _)| {
					k.eq(&DeliveryRewardInBalance::key()) |
						k.eq(&RequiredStakeForStakeAndSlash::key()) |
						k.eq(&Flavor::key())
				}) =>
				return true,
			_ => (),
		};

		matches!(
			call,
			RuntimeCall::PolkadotXcm(pallet_xcm::Call::force_xcm_version { .. }) |
				RuntimeCall::System(
					frame_system::Call::set_heap_pages { .. } |
						frame_system::Call::set_code { .. } |
						frame_system::Call::set_code_without_checks { .. } |
						frame_system::Call::kill_prefix { .. },
				) | RuntimeCall::ParachainSystem(..) |
				RuntimeCall::Timestamp(..) |
				RuntimeCall::Balances(..) |
				RuntimeCall::CollatorSelection(
					pallet_collator_selection::Call::set_desired_candidates { .. } |
						pallet_collator_selection::Call::set_candidacy_bond { .. } |
						pallet_collator_selection::Call::register_as_candidate { .. } |
						pallet_collator_selection::Call::leave_intent { .. } |
						pallet_collator_selection::Call::set_invulnerables { .. } |
						pallet_collator_selection::Call::add_invulnerable { .. } |
						pallet_collator_selection::Call::remove_invulnerable { .. },
				) | RuntimeCall::Session(pallet_session::Call::purge_keys { .. }) |
				RuntimeCall::XcmpQueue(..) |
				RuntimeCall::DmpQueue(..) |
				RuntimeCall::BridgeRococoGrandpa(pallet_bridge_grandpa::Call::<
					Runtime,
					BridgeGrandpaRococoInstance,
				>::initialize { .. }) |
				RuntimeCall::BridgeWococoGrandpa(pallet_bridge_grandpa::Call::<
					Runtime,
					BridgeGrandpaWococoInstance,
				>::initialize { .. }) |
				RuntimeCall::EthereumBeaconClient(
					snowbridge_ethereum_beacon_client::Call::force_checkpoint { .. } |
						snowbridge_ethereum_beacon_client::Call::set_operating_mode { .. },
				) | RuntimeCall::EthereumInboundQueue(
					snowbridge_inbound_queue::Call::set_operating_mode { .. },
			) | RuntimeCall::EthereumOutboundQueue(
					snowbridge_outbound_queue::Call::set_operating_mode { .. },
			) | RuntimeCall::EthereumControl(..)
		)
	}
}

pub type Barrier = TrailingSetTopicAsId<
	DenyThenTry<
		DenyReserveTransferToRelayChain,
		(
			// Allow local users to buy weight credit.
			TakeWeightCredit,
			// Expected responses are OK.
			AllowKnownQueryResponses<PolkadotXcm>,
			WithComputedOrigin<
				(
					// If the message is one that immediately attempts to pay for execution, then
					// allow it.
					AllowTopLevelPaidExecutionFrom<Everything>,
					// Parent, its pluralities (i.e. governance bodies) and relay treasury pallet
					// get free execution.
					AllowExplicitUnpaidExecutionFrom<(
						ParentOrParentsPlurality,
						Equals<RelayTreasuryLocation>,
					)>,
					// Subscriptions for version tracking are OK.
					AllowSubscriptionsFrom<ParentOrSiblings>,
				),
				UniversalLocation,
				ConstU32<8>,
			>,
		),
	>,
>;

/// Locations that will not be charged fees in the executor,
/// either execution or delivery.
/// We only waive fees for system functions, which these locations represent.
pub type WaivedLocations =
	(RelayOrOtherSystemParachains<SystemParachains, Runtime>, Equals<RelayTreasuryLocation>);

/// Cases where a remote origin is accepted as trusted Teleporter for a given asset:
/// - NativeToken with the parent Relay Chain and sibling parachains.
pub type TrustedTeleporters = ConcreteAssetFromSystem<TokenLocation>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = CurrencyTransactor;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	// BridgeHub does not recognize a reserve location for any asset. Users must teleport Native
	// token where allowed (e.g. with the Relay Chain).
	type IsReserve = ();
	type IsTeleporter = TrustedTeleporters;
	type UniversalLocation = UniversalLocation;
	type Barrier = Barrier;
	type Weigher = WeightInfoBounds<
		crate::weights::xcm::BridgeHubRococoXcmWeight<RuntimeCall>,
		RuntimeCall,
		MaxInstructions,
	>;
	type Trader =
		UsingComponents<WeightToFee, TokenLocation, AccountId, Balances, ToStakingPot<Runtime>>;
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetLocker = ();
	type AssetExchanger = ();
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
	type FeeManager = XcmFeeManagerFromComponents<
		WaivedLocations,
		(
			XcmExportFeeToRelayerRewardAccounts<
				Self::AssetTransactor,
				WococoGlobalConsensusNetwork,
				AssetHubWococoParaId,
				BridgeHubWococoChainId,
				BridgeHubWococoMessagesLane,
			>,
			XcmExportFeeToRelayerRewardAccounts<
				Self::AssetTransactor,
				RococoGlobalConsensusNetwork,
				AssetHubRococoParaId,
				BridgeHubRococoChainId,
				BridgeHubRococoMessagesLane,
			>,
			XcmFeeToAccount<Self::AssetTransactor, AccountId, TreasuryAccount>,
		),
	>;
	type MessageExporter = BridgeHubRococoOrBridgeHubWococoSwitchExporter;
	type UniversalAliases = Nothing;
	type CallDispatcher = WithOriginFilter<SafeCallFilter>;
	type SafeCallFilter = SafeCallFilter;
	type Aliasers = Nothing;
}

pub type PriceForParentDelivery =
	ExponentialPrice<FeeAssetId, BaseDeliveryFee, TransactionByteFee, ParachainSystem>;

/// Converts a local signed origin into an XCM multilocation.
/// Forms the basis for local origins sending/executing XCMs.
pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = WithUniqueTopic<(
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm, PriceForParentDelivery>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
)>;

#[cfg(feature = "runtime-benchmarks")]
pub(crate) mod benchmark_helper {
	use crate::xcm_config::{
		MultiAssets, MultiLocation, SendError, SendResult, SendXcm, Xcm, XcmHash,
	};

	pub struct DoNothingRouter;
	impl SendXcm for DoNothingRouter {
		type Ticket = ();
		fn validate(
			_dest: &mut Option<MultiLocation>,
			_msg: &mut Option<Xcm<()>>,
		) -> SendResult<()> {
			Ok(((), MultiAssets::new()))
		}
		fn deliver(_: ()) -> Result<XcmHash, SendError> {
			Ok([0; 32])
		}
	}
}

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
	pub ReachableDest: Option<MultiLocation> = Some(Parent.into());
	// Relay/native token can be teleported between BH and Relay.
	pub TeleportableAssets: Option<(MultiAssets, MultiLocation)> = Some((
		MultiAsset { fun: Fungible(10), id: Concrete(Parent.into()) }.into(),
		Parent.into(),
	));
	// Reserve transfers are disabled on BH.
	pub ReserveTransferableAssets: Option<(MultiAssets, MultiLocation)> = None;
}

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmRouter = XcmRouter;
	// We want to disallow users sending (arbitrary) XCMs from this chain.
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, ()>;
	// We support local origins dispatching XCM executions in principle...
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Nothing; // This parachain is not meant as a reserve location.
	type Weigher = WeightInfoBounds<
		crate::weights::xcm::BridgeHubRococoXcmWeight<RuntimeCall>,
		RuntimeCall,
		MaxInstructions,
	>;
	type UniversalLocation = UniversalLocation;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type TrustedLockers = ();
	type SovereignAccountOf = LocationToAccountId;
	type MaxLockers = ConstU32<8>;
	type WeightInfo = crate::weights::pallet_xcm::WeightInfo<Runtime>;
	type AdminOrigin = EnsureRoot<AccountId>;
	type MaxRemoteLockConsumers = ConstU32<0>;
	type RemoteLockConsumerIdentifier = ();
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

pub type AgentIdOf = HashedDescription<H256, DescribeFamily<DescribeAllTerminal>>;

pub type SnowbridgeExporter = EthereumBlobExporter<
	UniversalLocation,
	EthereumGatewayLocation,
	snowbridge_outbound_queue::Pallet<Runtime>,
	AgentIdOf,
>;

/// Hacky switch implementation, because we have just one runtime for Rococo and Wococo BridgeHub,
/// so it means we have just one XcmConfig
pub struct BridgeHubRococoOrBridgeHubWococoSwitchExporter;
impl ExportXcm for BridgeHubRococoOrBridgeHubWococoSwitchExporter {
	type Ticket = (NetworkId, (sp_std::prelude::Vec<u8>, XcmHash));

	fn validate(
		network: NetworkId,
		channel: u32,
		universal_source: &mut Option<InteriorMultiLocation>,
		destination: &mut Option<InteriorMultiLocation>,
		message: &mut Option<Xcm<()>>,
	) -> SendResult<Self::Ticket> {
		let relay: NetworkId = RelayNetwork::get();
		match network {
			Rococo => ToBridgeHubRococoHaulBlobExporter::validate(
				network,
				channel,
				universal_source,
				destination,
				message,
			)
			.map(|result| ((Rococo, result.0), result.1)),
			Wococo => ToBridgeHubWococoHaulBlobExporter::validate(
				network,
				channel,
				universal_source,
				destination,
				message,
			)
			.map(|result| ((Wococo, result.0), result.1)),
			location if location == EthereumNetwork::get() && relay == Rococo => {
				SnowbridgeExporter::validate(
					network,
					channel,
					universal_source,
					destination,
					message,
				)
				.map(|result| ((Ethereum { chain_id: 15 }, result.0), result.1)) // TODO get network ID
			},
			_ => unimplemented!("Unsupported network: {:?}", network),
		}
	}

	fn deliver(ticket: Self::Ticket) -> Result<XcmHash, SendError> {
		let (network, ticket) = ticket;
		let relay: NetworkId = RelayNetwork::get();
		match network {
			Rococo => ToBridgeHubRococoHaulBlobExporter::deliver(ticket),
			Wococo => ToBridgeHubWococoHaulBlobExporter::deliver(ticket),
			location if location == EthereumNetwork::get() && relay == Rococo =>
				SnowbridgeExporter::deliver(ticket),
			_ => unimplemented!("Unsupported network: {:?}", network),
		}
	}
}

pub struct AllowSiblingsOnly;
impl Contains<MultiLocation> for AllowSiblingsOnly {
	fn contains(location: &MultiLocation) -> bool {
		if let MultiLocation { parents: 1, interior: X1(Parachain(_)) } = location {
			true
		} else {
			false
		}
	}
}

/// A `HandleFee` implementation that simply deposits the fees for `ExportMessage` XCM instructions
/// into the accounts that are used for paying the relayer rewards.
/// Burns the fees in case of a failure.
pub struct XcmExportFeeToRelayerRewardAccounts<
	AssetTransactor,
	DestNetwork,
	DestParaId,
	DestBridgeHubId,
	BridgeLaneId,
>(PhantomData<(AssetTransactor, DestNetwork, DestParaId, DestBridgeHubId, BridgeLaneId)>);

impl<
		AssetTransactor: TransactAsset,
		DestNetwork: Get<NetworkId>,
		DestParaId: Get<cumulus_primitives_core::ParaId>,
		DestBridgeHubId: Get<ChainId>,
		BridgeLaneId: Get<LaneId>,
	> HandleFee
	for XcmExportFeeToRelayerRewardAccounts<
		AssetTransactor,
		DestNetwork,
		DestParaId,
		DestBridgeHubId,
		BridgeLaneId,
	>
{
	fn handle_fee(
		fee: MultiAssets,
		maybe_context: Option<&XcmContext>,
		reason: FeeReason,
	) -> MultiAssets {
		if matches!(reason, FeeReason::Export { network: bridged_network, destination }
				if bridged_network == DestNetwork::get() &&
					destination == X1(Parachain(DestParaId::get().into())))
		{
			// We have 2 relayer rewards accounts:
			// - the SA of the source parachain on this BH: this pays the relayers for delivering
			//   Source para -> Target Para message delivery confirmations
			// - the SA of the destination parachain on this BH: this pays the relayers for
			//   delivering Target para -> Source Para messages
			// We split the `ExportMessage` fee between these 2 accounts.
			let source_para_account = PayRewardFromAccount::<
				pallet_balances::Pallet<Runtime>,
				AccountId,
			>::rewards_account(RewardsAccountParams::new(
				BridgeLaneId::get(),
				DestBridgeHubId::get(),
				RewardsAccountOwner::ThisChain,
			));

			let dest_para_account = PayRewardFromAccount::<
				pallet_balances::Pallet<Runtime>,
				AccountId,
			>::rewards_account(RewardsAccountParams::new(
				BridgeLaneId::get(),
				DestBridgeHubId::get(),
				RewardsAccountOwner::BridgedChain,
			));

			for asset in fee.into_inner() {
				match asset.fun {
					Fungible(total_fee) => {
						let source_fee = total_fee / 2;
						deposit_or_burn_fee::<AssetTransactor, _>(
							MultiAsset { id: asset.id, fun: Fungible(source_fee) }.into(),
							maybe_context,
							source_para_account.clone(),
						);

						let dest_fee = total_fee - source_fee;
						deposit_or_burn_fee::<AssetTransactor, _>(
							MultiAsset { id: asset.id, fun: Fungible(dest_fee) }.into(),
							maybe_context,
							dest_para_account.clone(),
						);
					},
					NonFungible(_) => {
						deposit_or_burn_fee::<AssetTransactor, _>(
							asset.into(),
							maybe_context,
							source_para_account.clone(),
						);
					},
				}
			}

			return MultiAssets::new()
		}

		fee
	}
}
