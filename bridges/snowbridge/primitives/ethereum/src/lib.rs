// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use core::marker::PhantomData;
use hash_db::Hasher;

pub mod header;
pub mod log;
pub mod mpt;
pub mod node_codec;
pub mod receipt;
pub mod storage_proof;

pub use ethereum_types::{Address, H160, H256, H64, U256};
pub use header::{Bloom, Header, HeaderId};
pub use log::Log;
pub use receipt::Receipt;
pub use storage_proof::StorageProof;
pub use trie_db::TrieLayout;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum DecodeError {
	// Unexpected RLP data
	InvalidRLP(rlp::DecoderError),
	// Data does not match expected ABI
	InvalidABI(ethabi::Error),
	// Invalid message payload
	InvalidPayload,
}

impl From<rlp::DecoderError> for DecodeError {
	fn from(err: rlp::DecoderError) -> Self {
		DecodeError::InvalidRLP(err)
	}
}

impl From<ethabi::Error> for DecodeError {
	fn from(err: ethabi::Error) -> Self {
		DecodeError::InvalidABI(err)
	}
}

/// Trie layout for EIP-1186 state proof nodes.
#[derive(Default, Clone)]
pub struct EIP1186Layout<H>(PhantomData<H>);

impl<H: Hasher<Out = H256>> TrieLayout for EIP1186Layout<H> {
	const USE_EXTENSION: bool = true;
	const ALLOW_EMPTY: bool = false;
	const MAX_INLINE_VALUE: Option<u32> = None;
	type Hash = H;
	type Codec = node_codec::RlpNodeCodec<H>;
}
