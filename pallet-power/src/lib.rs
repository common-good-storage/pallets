#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use sp_std::{fmt::Debug, prelude::*};

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::storage]
    #[pallet::getter(fn claims)]
    pub type Claims<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Claim>;

    #[pallet::storage]
    #[pallet::getter(fn miner_count)]
    pub type MinerCount<T: Config> = StorageValue<_, u64>;

    #[pallet::storage]
    #[pallet::getter(fn total_raw_bytes_power)]
    pub type TotalRawBytesPower<T: Config> = StorageValue<_, u64>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId")]
    pub enum Event<T: Config> {
        MinerCreated(T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1000000)]
        pub(super) fn create_miner(
            origin: OriginFor<T>,
            params: CreateMinerParams<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            // currently a signed origin, any signed
            // Miner::new()
            // Set Claim with compact encoding
            // UpdateStates - MinerCount, MinerAboveMinPower
            // Return Miner address
            // Emit an event.
            // Return a successful DispatchResult
            unimplemented!()
        }
    }
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct CreateMinerParams<AccountId: Encode + Decode + Clone + Debug + Eq + PartialEq> {
    /// Owner of the Miner Account
    owner: AccountId,
    /// Worker of the Miner Account
    worker: AccountId,
    ///  Placeholder for window PoSt proof type
    window_post_proof_type: u64,
    ///  PeerID of the miner
    peer: Vec<u8>,
    /// Multiaddress of the miner to connect to it
    multiaddrs: Vec<u8>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct Claim {
    /// Placeholder: Window PoSt Proof Type
    window_post_proof_type: u64,
    /// Raw Bytes Stored by the miner
    raw_bytes_power: u32,
    /// Quality Adjusted Power
    ///
    /// This is the raw bytes * Sector Quality Multiplier
    ///
    /// https://spec.filecoin.io/#section-glossary.quality-adjusted-power
    quality_adjusted_power: u32,
}
