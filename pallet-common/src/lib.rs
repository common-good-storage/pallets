#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, HasCompact};
use frame_support::{Parameter, RuntimeDebug};
use sp_runtime::traits::Member;

pub trait Power {
    /// AccountId type for owner, worker, contorller and miner
    type AccountId: Parameter + Clone + Eq + PartialEq;
    /// TODO compact type - bigInt
    type StoragePower: Parameter + Member + Clone;
    /// Libp2p PeerId
    type PeerId: AsRef<[u8]> + Clone + Send + 'static;

    /// Register a miner - used by miner
    fn register_new_miner(
        owner: Self::AccountId,
        worker: Self::AccountId,
        peer_id: Self::PeerId,
    ) -> Option<Claim<Self::StoragePower>>;

    /// Updates the claimed power for a miner, requested by miners
    /// Example: Worker recovers faulty sector and adds power back
    fn update_claim(
        miner: Self::AccountId,
        raw_bytes_delta: Self::StoragePower,
        quality_adjusted_delta: Self::StoragePower,
    ) -> Option<Claim<Self::StoragePower>>;
}

/// Struct that stores the claimed storage from a miner, used when submitting PoRep to ensure miner has claims
/// Claims are updated by miners as they update their storage and deals to update their Storage Power
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct Claim<StoragePower> {
    /// Raw Bytes Stored by the miner
    raw_bytes_power: StoragePower,
    /// Quality Adjusted Power
    /// This is the raw bytes * Sector Quality Multiplier (when committing storage)
    /// It is equal to raw_bytes_power for now
    quality_adjusted_power: StoragePower,
}
