// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
//! Implementation for [`snowbridge_core::outbound::SendMessage`]
use super::*;
use bridge_hub_common::AggregateMessageOrigin;
use codec::Encode;
use frame_support::{
	ensure,
	traits::{EnqueueMessage, Get},
	CloneNoBound, RuntimeDebugNoBound,
};
use frame_system::unique;
use snowbridge_core::{
	outbound::{
		AgentExecuteCommand, Command, Fee, Message, QueuedMessage, SendError, SendMessage,
		SendMessageFeeProvider, VersionedQueuedMessage,
	},
	ChannelId, PayMaster, PayRewardError, PRIMARY_GOVERNANCE_CHANNEL,
};
use sp_core::{H160, H256};
use sp_runtime::BoundedVec;

/// The maximal length of an enqueued message, as determined by the MessageQueue pallet
pub type MaxEnqueuedMessageSizeOf<T> =
	<<T as Config>::MessageQueue as EnqueueMessage<AggregateMessageOrigin>>::MaxMessageLen;

#[derive(Encode, Decode, CloneNoBound, RuntimeDebugNoBound)]
#[cfg_attr(feature = "std", derive(PartialEq))]
pub struct Ticket {
	pub message_id: H256,
	pub channel_id: ChannelId,
	pub message: QueuedMessage,
}

impl<T> SendMessage for Pallet<T>
where
	T: Config,
{
	type Ticket = Ticket;

	fn validate(
		message: &Message,
	) -> Result<(Self::Ticket, Fee<<Self as SendMessageFeeProvider>::Balance>), SendError> {
		// The inner payload should not be too large
		let payload = message.command.abi_encode();
		ensure!(
			payload.len() < T::MaxMessagePayloadSize::get() as usize,
			SendError::MessageTooLarge
		);

		// Ensure there is a registered channel we can transmit this message on
		ensure!(T::Channels::contains(&message.channel_id), SendError::InvalidChannel);

		// Generate a unique message id unless one is provided
		let message_id: H256 = message
			.id
			.unwrap_or_else(|| unique((message.channel_id, &message.command)).into());

		let gas_used_at_most = T::GasMeter::maximum_gas_used_at_most(&message.command);
		let fee = Self::calculate_fee(gas_used_at_most, T::PricingParameters::get());

		// Todo: Do we need to check command{TransferToken}.fee_amount > fee.remote to avoid
		// spamming, then this PR does not require unordered messaging as prerequisite
		// Or just leave that to relayer to check if delivering the message is profitable and
		// bound this PR with unordered messaging, more changes required

		let ticket = Ticket {
			message_id,
			channel_id: message.channel_id,
			message: QueuedMessage {
				id: message_id,
				channel_id: message.channel_id,
				command: message.command.clone(),
			},
		};

		Ok((ticket, fee))
	}

	fn deliver(ticket: Self::Ticket) -> Result<H256, SendError> {
		let origin = AggregateMessageOrigin::Snowbridge(ticket.channel_id);

		if ticket.channel_id != PRIMARY_GOVERNANCE_CHANNEL {
			ensure!(!Self::operating_mode().is_halted(), SendError::Halted);
		}

		let _ = match ticket.clone().message.command {
			Command::AgentExecute { command, .. } => match command {
				AgentExecuteCommand::TransferToken { fee_amount, .. } =>
					Self::lock_fee(ticket.message_id, fee_amount)
						.map_err(|_| SendError::LockFeeFailed),
			},
			_ => Ok(()),
		}?;

		let queued_message: VersionedQueuedMessage = ticket.message.into();

		// The whole message should not be too large
		let bounded =
			BoundedVec::<u8, MaxEnqueuedMessageSizeOf<T>>::try_from(queued_message.encode())
				.map_err(|_| SendError::MessageTooLarge)?;
		let message = bounded.as_bounded_slice();

		T::MessageQueue::enqueue_message(message, origin);
		Self::deposit_event(Event::MessageQueued {
			channel_id: ticket.channel_id,
			id: ticket.message_id,
		});
		Ok(ticket.message_id)
	}
}

impl<T: Config> SendMessageFeeProvider for Pallet<T> {
	type Balance = T::Balance;

	/// The local component of the message processing fees in native currency
	fn local_fee() -> Self::Balance {
		Self::calculate_local_fee()
	}
}

impl<T: Config> PayMaster for Pallet<T>
where
	<T as frame_system::Config>::AccountId: From<[u8; 32]>,
{
	/// The local component of the message processing fees in native currency
	fn reward_relay(chain_id: u64, message_id: H256, relay: H160) -> Result<(), PayRewardError> {
		Self::unlock_fee(chain_id, message_id, relay)
	}
}
