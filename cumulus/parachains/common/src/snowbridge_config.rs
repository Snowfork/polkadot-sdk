frame_support::parameter_types! {
	/// User fee for ERC20 token transfer back to Ethereum.
	/// (initially was calculated by test `OutboundQueue::calculate_fees` - ETH/ROC 1/400 and fee_per_gas 20 GWEI = 2200698000000 + *25%)
	/// Needs to be more than fee calculated from DefaultFeeConfig FeeConfigRecord in snowbridge:parachain/pallets/outbound-queue/src/lib.rs
	/// Polkadot uses 12 decimals, Kusama and Rococo 10 decimals.
	pub const BridgeHubEthereumBaseFeeInDOT: u128 = 27_508_725_000;
	pub const BridgeHubEthereumBaseFeeInKSM: u128 = 2_750_872_500_000;
	pub const BridgeHubEthereumBaseFeeInRocs: u128 = 2_750_872_500_000;
}
