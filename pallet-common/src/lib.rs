#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{Parameter, RuntimeDebug};
use sp_runtime::traits::Member;

pub trait Power {
    /// AccountId type for owner, worker, contorller and miner
    type AccountId: Parameter + Member + Clone + Eq + PartialEq + Default;
    /// TODO compact type - bigInt?
    type StoragePower: Parameter + Member + Clone;
    /// Libp2p PeerId
    type PeerId: Parameter + Member + AsRef<[u8]> + Clone + Send + 'static;

    /// Register a miner - used by miner
    fn register_new_miner(
        miner: &Self::AccountId,
        owner: &Self::AccountId,
        worker: &Self::AccountId,
        peer_id: &Self::PeerId,
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
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct Claim<StoragePower> {
    /// Raw Bytes Stored by the miner
    raw_bytes_power: StoragePower,
    /// Quality Adjusted Power
    /// This is the raw bytes * Sector Quality Multiplier (when committing storage)
    /// It is equal to raw_bytes_power for now
    quality_adjusted_power: StoragePower,
}

#[derive(Encode, Decode, Default)]
pub struct MinerId(pub u32);

// Code from https://github.com/paritytech/polkadot/blob/rococo-v1/parachain/src/primitives.rs
/// This type can be converted into and possibly from an AccountId (which itself is generic).
pub trait AccountIdConversion<AccountId>: Sized {
    /// Convert into an account ID. This is infallible.
    fn into_account(&self) -> AccountId;

    /// Try to convert an account ID into this type. Might not succeed.
    fn try_from_account(a: &AccountId) -> Option<Self>;
}

// Code from https://github.com/paritytech/polkadot/blob/rococo-v1/parachain/src/primitives.rs
// This will be moved to own crate and can remove
struct TrailingZeroInput<'a>(&'a [u8]);
impl<'a> codec::Input for TrailingZeroInput<'a> {
    fn remaining_len(&mut self) -> Result<Option<usize>, codec::Error> {
        Ok(None)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<(), codec::Error> {
        let len = into.len().min(self.0.len());
        into[..len].copy_from_slice(&self.0[..len]);
        for i in &mut into[len..] {
            *i = 0;
        }
        self.0 = &self.0[len..];
        Ok(())
    }
}

// Code modified from https://github.com/paritytech/polkadot/blob/rococo-v1/parachain/src/primitives.rs
/// Format is b"miner" ++ encode(parachain ID) ++ 00.... where 00... is indefinite trailing
/// zeroes to fill AccountId.
impl<T: Encode + Decode + Default> AccountIdConversion<T> for MinerId {
    fn into_account(&self) -> T {
        (b"miner", self)
            .using_encoded(|b| T::decode(&mut TrailingZeroInput(b)))
            .unwrap_or_default()
    }

    fn try_from_account(x: &T) -> Option<Self> {
        x.using_encoded(|d| {
            if &d[0..5] != b"miner" {
                return None;
            }
            let mut cursor = &d[5..];
            let result = Decode::decode(&mut cursor).ok()?;
            if cursor.iter().all(|x| *x == 0) {
                Some(result)
            } else {
                None
            }
        })
    }
}
