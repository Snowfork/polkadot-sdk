use crate::{
	xcm_config::{AgentIdOf, EthereumNetwork, UniversalLocation},
	Runtime,
};
use snowbridge_router_primitives::outbound::EthereumBlobExporter;

pub type SnowbridgeExporter = EthereumBlobExporter<
	UniversalLocation,
	EthereumNetwork,
	snowbridge_outbound_queue::Pallet<Runtime>,
	AgentIdOf,
>;
