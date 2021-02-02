#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use pallet_common::{Multiaddress, PeerId};

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
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub(super) fn create_miner(
            origin: OriginFor<T>,
            owner: T::AccountId,
            worker: T::AccountId,
            peer_id: PeerId,
            multiaddrs: Vec<Multiaddress>,
        ) -> DispatchResultWithPostInfo {
            // currently a signed origin, any signed
            // Miner::new()
            // Set Claim with compact encoding
            // UpdateStates - MinerCount
            // Return Miner address
            // Emit an event.
            // Return a successful DispatchResult
            unimplemented!()
        }
    }
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
pub struct Claim {
    /// Raw Bytes Stored by the miner
    raw_bytes_power: u32,
    /// Quality Adjusted Power
    /// This is the raw bytes * Sector Quality Multiplier (when committing storage)
    /// It is equal to raw_bytes_power for now
    quality_adjusted_power: u32,
}
