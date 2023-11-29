use snowbridge_router_primitives::outbound::EthereumBlobExporter;
use xcm_config::{AgentIdOf, EthereumNetwork, UniversalLocation};

pub type SnowbridgeExporter = EthereumBlobExporter<
	UniversalLocation,
	EthereumNetwork,
	snowbridge_outbound_queue::Pallet<Runtime>,
	AgentIdOf,
>;
