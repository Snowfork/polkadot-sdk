// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>
use codec::{Decode, Encode};
use frame_support::PalletError;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

#[derive(Copy, Clone, Encode, Decode, Eq, PartialEq, TypeInfo, RuntimeDebug, PalletError)]
pub enum BlsError {
	InvalidSignature,
	InvalidPublicKey,
	InvalidAggregatePublicKeys,
	SignatureVerificationFailed,
	HashToCurveFailed,
}

pub mod milagro {
	use crate::{BlsError, PublicKey, Signature, SyncCommittee};
	use frame_support::{
		ensure,
		pallet_prelude::{Decode, Encode, MaxEncodedLen, TypeInfo},
	};
	pub use milagro_bls::{
		AggregatePublicKey, AggregateSignature, PublicKey as PublicKeyPrepared,
		Signature as SignaturePrepared,
	};
	use snowbridge_ethereum::H256;
	use sp_std::{boxed::Box, iter::repeat, prelude::*};

	/// fast_aggregate_verify optimized with aggregate key subtracting absent ones.
	pub fn fast_aggregate_verify(
		aggregate_pubkey: &PublicKeyPrepared,
		absent_pubkeys: &Vec<PublicKeyPrepared>,
		message: H256,
		signature: &Signature,
	) -> Result<(), BlsError> {
		let agg_sig = prepare_aggregate_signature(signature)?;
		let agg_key = prepare_aggregate_pubkey_from_absent(aggregate_pubkey, absent_pubkeys)?;
		fast_aggregate_verify_pre_aggregated(agg_sig, agg_key, message)
	}

	/// Decompress one public key into a point in G1.
	pub fn prepare_milagro_pubkey(pubkey: &PublicKey) -> Result<PublicKeyPrepared, BlsError> {
		PublicKeyPrepared::from_bytes_unchecked(&pubkey.0).map_err(|_| BlsError::InvalidPublicKey)
	}

	/// Prepare for G1 public keys.
	pub fn prepare_g1_pubkeys(pubkeys: &[PublicKey]) -> Result<Vec<PublicKeyPrepared>, BlsError> {
		pubkeys
			.iter()
			// Deserialize one public key from compressed bytes
			.map(prepare_milagro_pubkey)
			.collect::<Result<Vec<PublicKeyPrepared>, BlsError>>()
	}

	/// Prepare for G1 AggregatePublicKey.
	pub fn prepare_aggregate_pubkey(
		pubkeys: &[PublicKeyPrepared],
	) -> Result<AggregatePublicKey, BlsError> {
		AggregatePublicKey::into_aggregate(pubkeys).map_err(|_| BlsError::InvalidPublicKey)
	}

	/// Prepare for G1 AggregatePublicKey.
	pub fn prepare_aggregate_pubkey_from_absent(
		aggregate_key: &PublicKeyPrepared,
		absent_pubkeys: &Vec<PublicKeyPrepared>,
	) -> Result<AggregatePublicKey, BlsError> {
		let mut aggregate_pubkey = AggregatePublicKey::from_public_key(aggregate_key);
		if !absent_pubkeys.is_empty() {
			let absent_aggregate_key = prepare_aggregate_pubkey(absent_pubkeys)?;
			aggregate_pubkey.point.sub(&absent_aggregate_key.point);
		}
		Ok(AggregatePublicKey { point: aggregate_pubkey.point })
	}

	/// Prepare for G2 AggregateSignature, normally more expensive than G1 operation.
	pub fn prepare_aggregate_signature(
		signature: &Signature,
	) -> Result<AggregateSignature, BlsError> {
		Ok(AggregateSignature::from_signature(
			&SignaturePrepared::from_bytes(&signature.0).map_err(|_| BlsError::InvalidSignature)?,
		))
	}

	/// fast_aggregate_verify_pre_aggregated which is the most expensive call in beacon light
	/// client.
	pub fn fast_aggregate_verify_pre_aggregated(
		agg_sig: AggregateSignature,
		aggregate_key: AggregatePublicKey,
		message: H256,
	) -> Result<(), BlsError> {
		ensure!(
			agg_sig.fast_aggregate_verify_pre_aggregated(&message[..], &aggregate_key),
			BlsError::SignatureVerificationFailed
		);
		Ok(())
	}

	/// Prepared G1 public key of sync committee as it is stored in the runtime storage.
	#[derive(Clone, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen)]
	pub struct SyncCommitteePrepared<const COMMITTEE_SIZE: usize> {
		pub root: H256,
		pub pubkeys: Box<[PublicKeyPrepared; COMMITTEE_SIZE]>,
		pub aggregate_pubkey: PublicKeyPrepared,
	}

	impl<const COMMITTEE_SIZE: usize> Default for SyncCommitteePrepared<COMMITTEE_SIZE> {
		fn default() -> Self {
			let pubkeys: Vec<PublicKeyPrepared> =
				repeat(PublicKeyPrepared::default()).take(COMMITTEE_SIZE).collect();
			let pubkeys: Box<[PublicKeyPrepared; COMMITTEE_SIZE]> =
				Box::new(pubkeys.try_into().map_err(|_| ()).expect("checked statically; qed"));

			SyncCommitteePrepared {
				root: H256::default(),
				pubkeys,
				aggregate_pubkey: PublicKeyPrepared::default(),
			}
		}
	}

	impl<const COMMITTEE_SIZE: usize> TryFrom<&SyncCommittee<COMMITTEE_SIZE>>
		for SyncCommitteePrepared<COMMITTEE_SIZE>
	{
		type Error = BlsError;

		fn try_from(sync_committee: &SyncCommittee<COMMITTEE_SIZE>) -> Result<Self, Self::Error> {
			let g1_pubkeys = prepare_g1_pubkeys(&sync_committee.pubkeys)?;
			let sync_committee_root =
				sync_committee.hash_tree_root().expect("checked statically; qed");

			Ok(SyncCommitteePrepared::<COMMITTEE_SIZE> {
				pubkeys: g1_pubkeys.try_into().map_err(|_| ()).expect("checked statically; qed"),
				aggregate_pubkey: prepare_milagro_pubkey(&sync_committee.aggregate_pubkey)?,
				root: sync_committee_root,
			})
		}
	}
}

pub mod arkworks {
	use crate::{BlsError, SyncCommittee};
	use ark_bls12_381::{
		g2::Config as G2Config, Bls12_381, G1Affine, G1Projective, G2Affine, G2Projective,
	};
	use ark_ec::{
		hashing::{
			curve_maps::wb::WBMap, map_to_curve_hasher::MapToCurveBasedHasher, HashToCurve,
			HashToCurveError,
		},
		models::short_weierstrass::Projective,
		pairing::Pairing,
		AffineRepr, CurveGroup,
	};
	use ark_ff::{field_hashers::DefaultFieldHasher, Zero};
	pub use ark_scale::hazmat::ArkScaleProjective;
	use ark_serialize::*;
	use core::{borrow::Borrow, ops::Neg};
	use frame_support::pallet_prelude::{Decode, Encode};
	use sha2::Sha256;
	use snowbridge_ethereum::H256;
	use sp_std::{boxed::Box, prelude::Vec};

	/// Domain Separation Tag for signatures on G2
	pub const DST_G2: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_POP_";

	pub type PublicKeyPrepared = ArkScaleProjective<G1Projective>;

	#[derive(Clone, Debug)]
	pub struct Signature(G2Projective);
	impl From<G2Projective> for Signature {
		fn from(sig: G2Projective) -> Signature {
			Signature(sig)
		}
	}
	impl AsRef<G2Projective> for Signature {
		fn as_ref(&self) -> &G2Projective {
			&self.0
		}
	}
	impl Signature {
		pub fn from_bytes(bytes: &[u8]) -> Result<Signature, SerializationError> {
			let p = G2Affine::deserialize_compressed_unchecked(bytes)?;
			Ok(Self(p.into()))
		}

		#[allow(dead_code)]
		pub fn aggregate<S: Borrow<Signature>>(
			signatures: impl IntoIterator<Item = S>,
		) -> Signature {
			signatures.into_iter().map(|s| s.borrow().0).sum::<G2Projective>().into()
		}
	}

	#[derive(Clone, Debug, Eq, PartialEq, Hash, CanonicalSerialize, CanonicalDeserialize)]
	pub struct PublicKey(pub G1Projective);
	impl From<G1Projective> for PublicKey {
		fn from(pk: G1Projective) -> PublicKey {
			PublicKey(pk)
		}
	}
	impl PublicKey {
		pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, SerializationError> {
			let p = G1Affine::deserialize_compressed_unchecked(bytes)?;
			Ok(Self(p.into()))
		}

		pub fn encode_from_bytes(bytes: &[u8]) -> Result<PublicKeyPrepared, SerializationError> {
			let pubkey = PublicKey::from_bytes(bytes)?;
			Ok(ArkScaleProjective::from(pubkey.0))
		}

		pub fn aggregate<P: Borrow<PublicKey>>(
			public_keys: impl IntoIterator<Item = P>,
		) -> PublicKey {
			public_keys.into_iter().map(|s| s.borrow().0).sum::<G1Projective>().into()
		}

		pub fn verify(&self, signature: &Signature, message: &G2Projective) -> bool {
			Bls12_381::multi_pairing(
				[G1Affine::generator().neg(), self.0.into_affine()],
				[signature.as_ref().into_affine(), message.into_affine()],
			)
			.is_zero()
		}
	}

	pub fn prepare_pubkeys(
		pubkeys: &[crate::PublicKey],
	) -> Result<Vec<PublicKeyPrepared>, SerializationError> {
		pubkeys
			.iter()
			// Deserialize one public key from compressed bytes
			.map(|pk| PublicKey::encode_from_bytes(pk.0.as_ref()))
			.collect::<Result<Vec<PublicKeyPrepared>, SerializationError>>()
	}

	#[derive(Encode, Decode)]
	pub struct SyncCommitteePrepared<const COMMITTEE_SIZE: usize> {
		pub root: H256,
		pub pubkeys: Box<[PublicKeyPrepared; COMMITTEE_SIZE]>,
		pub aggregate_pubkey: PublicKeyPrepared,
	}

	impl<const COMMITTEE_SIZE: usize> TryFrom<&SyncCommittee<COMMITTEE_SIZE>>
		for SyncCommitteePrepared<COMMITTEE_SIZE>
	{
		type Error = SerializationError;

		fn try_from(sync_committee: &SyncCommittee<COMMITTEE_SIZE>) -> Result<Self, Self::Error> {
			let aggregate_pubkey =
				PublicKey::encode_from_bytes(sync_committee.aggregate_pubkey.0.as_ref())?;
			let g1_pubkeys = prepare_pubkeys(&sync_committee.pubkeys)?;
			let sync_committee_root =
				sync_committee.hash_tree_root().expect("checked statically; qed");
			Ok(SyncCommitteePrepared::<COMMITTEE_SIZE> {
				pubkeys: g1_pubkeys.try_into().map_err(|_| ()).expect("checked statically; qed"),
				aggregate_pubkey,
				root: sync_committee_root,
			})
		}
	}

	pub fn hash_to_curve_g2(message: &[u8]) -> Result<G2Projective, HashToCurveError> {
		let wb_to_curve_hasher = MapToCurveBasedHasher::<
			Projective<G2Config>,
			DefaultFieldHasher<Sha256, 128>,
			WBMap<G2Config>,
		>::new(DST_G2)?;
		Ok(wb_to_curve_hasher.hash(message)?.into())
	}

	pub fn fast_aggregate_verify(
		pub_keys: Vec<PublicKeyPrepared>,
		message: H256,
		signature: crate::Signature,
	) -> Result<bool, BlsError> {
		let sig = Signature::from_bytes(&signature.0).map_err(|_| BlsError::InvalidSignature)?;
		let pub_keys: Vec<PublicKey> =
			pub_keys.into_iter().map(|pk| PublicKey::from(pk.0)).collect();
		let agg_pk = PublicKey::aggregate(pub_keys);
		let msg = hash_to_curve_g2(&message.0).map_err(|_| BlsError::HashToCurveFailed)?;
		Ok(agg_pk.verify(&sig, &msg))
	}
}
