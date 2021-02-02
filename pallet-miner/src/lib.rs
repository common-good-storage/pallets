#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

use codec::{Decode, Encode};
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
        type BlockNumber: Parameter + Member + Clone + Eq + PartialEq;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::storage]
    #[pallet::getter(fn miners)]
    pub type Miners<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        MinerInfo<T::AccountId, <T as pallet::Config>::BlockNumber>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn miner_index)]
    pub type MinerIndex<T: Config> = StorageValue<_, u128>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId")]
    pub enum Event<T: Config> {
        PlaceholderEvent(T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
		// Benchmark not accurate
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn change_worker_address(
            origin: OriginFor<T>,
            miner: T::AccountId,
            new_worker: T::AccountId,
            new_controllers: Option<Vec<T::AccountId>>,
        ) -> DispatchResultWithPostInfo {
            // ChangeWorkerAddress will ALWAYS overwrite the existing control addresses with the control addresses passed in the params.
            // If a None is passed, the control addresses will be cleared.
            // A worker change will be scheduled if the worker passed in the params is different from the existing worker.
            unimplemented!()
        }

		// Benchmark not accurate
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn change_peer_id(
            origin: OriginFor<T>,
            miner: T::AccountId,
            new_peer_id: PeerId,
        ) -> DispatchResultWithPostInfo {
            unimplemented!()
        }

		// Benchmark not accurate
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn change_multiaddress(
            origin: OriginFor<T>,
            miner: T::AccountId,
            new_multiaddresses: Vec<Multiaddress>,
        ) -> DispatchResultWithPostInfo {
            unimplemented!()
        }

		// Benchmark not accurate
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn confirm_update_worker_key(
            origin: OriginFor<T>,
            miner: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            // triggers a change in new worker key if it was previously set and the activation time
            // has arrived
            unimplemented!()
        }

		// Benchmark not accurate
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn change_owner_address(
            origin: OriginFor<T>,
            miner: T::AccountId,
            new_owner: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            // Proposes or confirms a change of owner address.
            // If invoked by the current owner, proposes a new owner address for confirmation. If the proposed address is the
            // current owner address, revokes any existing proposal.
            // If invoked by the previously proposed address, with the same proposal, changes the current owner address to be
            // that proposed address.
            unimplemented!()
        }
    }
}

impl<T: Config> Pallet<T> {
    pub fn new(
        owner: T::AccountId,
        worker: T::AccountId,
        peer_id: PeerId,
        multiaddrs: Vec<Multiaddress>,
    ) -> Result<T::AccountId, Error<T>> {
        // checkadd MinerIndex
        // Assign new Miner addr
        // Add miner to Miners
        // Return Miner address
        unimplemented!()
    }
}

#[derive(Encode, Decode)]
pub struct MinerInfo<
    AccountId: Encode + Decode + Eq + PartialEq,
    BlockNumber: Encode + Decode + Eq + PartialEq,
> {
    /// Owner of this Miner
    owner: AccountId,
    /// Worker of this Miner
    /// Used to sign messages (and in the future blocks) on behalf of the miner
    worker: AccountId,
    /// Other addresses that can sign messages on behalf of the miner
    controllers: Option<Vec<AccountId>>,
    /// Miner's libp2p PeerId
    peer_id: PeerId,
    /// Multiaddresses to establish connections with the miner
    multiaddrs: Vec<Multiaddress>,
    /// Update to this worker address to at defined time  
    pending_worker: WorkerKeyChange<AccountId, BlockNumber>,
    /// Update to this owner address when it confirms
    pending_owner: AccountId,
}

#[derive(Encode, Decode)]
pub struct WorkerKeyChange<
    AccountId: Encode + Decode + Eq + PartialEq,
    BlockNumber: Encode + Decode + Eq + PartialEq,
> {
	/// New Worker Address to be updated
    new_worker: AccountId,
	/// Time after which confirm_update_worker_key will trigger updates to MinerInfo
    effective_at: BlockNumber,
}
