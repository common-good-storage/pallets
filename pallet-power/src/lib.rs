#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

use pallet_common::{Claim, Power};

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Libp2p Peer Identifier, usually array of bytes  
        type PeerId: Parameter + Member + AsRef<[u8]> + Clone + Send + 'static;
        /// Unit used for recoding raw bytes and quality adjusted power
        type StoragePower: Parameter + Member + Clone;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    /// Miners address mapped to their Claims on storage power
    #[pallet::storage]
    #[pallet::getter(fn claims)]
    pub type Claims<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Claim<T::StoragePower>>;

    /// Total Miner registered in the system
    #[pallet::storage]
    #[pallet::getter(fn miner_count)]
    pub type MinerCount<T: Config> = StorageValue<_, u64>;

    /// Total Power in Raw bytes declared in the system
    #[pallet::storage]
    #[pallet::getter(fn total_raw_bytes_power)]
    pub type TotalRawBytesPower<T: Config> = StorageValue<_, u64>;

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// This is a placeholder
        /// `frame_support::pallet` macro require pallet::call but no call in this phase
        #[pallet::weight(10_000)]
        fn do_something(_: OriginFor<T>) -> DispatchResultWithPostInfo {
            unimplemented!()
        }
    }
}

impl<T: Config> Power for Pallet<T> {
    type AccountId = T::AccountId;
    type StoragePower = T::StoragePower;
    type PeerId = T::PeerId;

    fn register_new_miner(
        owner: T::AccountId,
        worker: T::AccountId,
        peer_id: Self::PeerId,
    ) -> Option<Claim<Self::StoragePower>> {
        unimplemented!()
    }

    fn update_claim(
        miner: <T as frame_system::Config>::AccountId,
        raw_bytes_delta: Self::StoragePower,
        quality_adjusted_delta: Self::StoragePower,
    ) -> Option<Claim<Self::StoragePower>> {
        unimplemented!()
    }
}
