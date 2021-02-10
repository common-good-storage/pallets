#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

use codec::{Decode, Encode};
use pallet_common::{AccountIdConversion, MinerId, Power};

// `pallet::Module` is created by `pallet` macro
pub use pallet::{Config, Error, Event, MinerIndex, Miners, Module, Pallet};

pub type AccountIdOf<T> = <<T as Config>::Power as Power>::AccountId;
pub type PeerId<T> = <<T as Config>::Power as Power>::PeerId;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type BlockNumber: Parameter + Member + Clone + Eq + PartialEq;
        type Power: Power;
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
        AccountIdOf<T>,
        MinerInfo<AccountIdOf<T>, <T as pallet::Config>::BlockNumber, PeerId<T>>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn miner_index)]
    pub type MinerIndex<T: Config> = StorageValue<_, u32>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(AccountIdOf<T> = "AccountId")]
    pub enum Event<T: Config> {
        /// Emits new miner address
        MinerCreated(AccountIdOf<T>),
        /// Emits miner address and requested change in worker address
        WorkerChangeRequested(AccountIdOf<T>, AccountIdOf<T>),
        /// Emits miner address and new worker address
        WorkerChanged(AccountIdOf<T>, AccountIdOf<T>),
        /// Emits miner address and new worker address to update to
        PeerIdChanged(AccountIdOf<T>, AccountIdOf<T>),
        /// Emits miner address and new owner address to update to
        OwnerChangeRequested(AccountIdOf<T>, AccountIdOf<T>),
        /// Emits miner address and new owner address
        OwnerChanged(AccountIdOf<T>, AccountIdOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        Overflow,
        ClaimsNotSet,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Benchmark not accurate
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create(
            origin: OriginFor<T>,
            owner: AccountIdOf<T>,
            worker: AccountIdOf<T>,
            peer_id: PeerId<T>,
        ) -> DispatchResultWithPostInfo {
            // following https://github.com/filecoin-project/specs-actors/blob/57195d8909b1c366fd1af41de9e92e11d7876177/actors/builtin/miner/miner_actor.go#L97
            // Note: This replaces the external call to the power actor and register the miner
            // claims with `Power::register_new_miner` method
            ensure_signed(origin)?;

            let mut miner_index = MinerIndex::<T>::get().unwrap_or_default();
            miner_index = miner_index.checked_add(1).ok_or(Error::<T>::Overflow)?;
            let miner: AccountIdOf<T> = MinerId(miner_index).into_account();
            MinerIndex::<T>::put(miner_index);

            T::Power::register_new_miner(&miner).ok_or(Error::<T>::ClaimsNotSet)?;

            let miner_info = MinerInfo {
                owner,
                worker,
                controllers: None,
                peer_id,
                pending_worker: None,
                pending_owner: None,
            };

            Miners::<T>::insert(miner.clone(), miner_info);
            Self::deposit_event(Event::MinerCreated(miner));

            Ok(().into())
        }

        // Benchmark not accurate
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn change_worker_address(
            _origin: OriginFor<T>,
            _miner: AccountIdOf<T>,
            _new_worker: AccountIdOf<T>,
            _new_controllers: Option<Vec<AccountIdOf<T>>>,
        ) -> DispatchResultWithPostInfo {
            // ChangeWorkerAddress will ALWAYS overwrite the existing control addresses with the control addresses passed in the params.
            // If a None is passed, the control addresses will be cleared.
            // A worker change will be scheduled if the worker passed in the params is different from the existing worker.
            // following https://github.com/filecoin-project/specs-actors/blob/57195d8909b1c366fd1af41de9e92e11d7876177/actors/builtin/miner/miner_actor.go#L225
            unimplemented!()
        }

        // Benchmark not accurate
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn change_peer_id(
            _origin: OriginFor<T>,
            _miner: AccountIdOf<T>,
            _new_peer_id: PeerId<T>,
        ) -> DispatchResultWithPostInfo {
            // following https://github.com/filecoin-project/specs-actors/blob/57195d8909b1c366fd1af41de9e92e11d7876177/actors/builtin/miner/miner_actor.go#L266
            unimplemented!()
        }

        // Benchmark not accurate
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn confirm_update_worker_key(
            _origin: OriginFor<T>,
            _miner: AccountIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            // triggers a change in new worker key if it was previously set and the activation time
            // has arrived
            // following https://github.com/filecoin-project/specs-actors/blob/57195d8909b1c366fd1af41de9e92e11d7876177/actors/builtin/miner/miner_actor.go#L205
            unimplemented!()
        }

        // Benchmark not accurate
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn change_owner_address(
            _origin: OriginFor<T>,
            _miner: AccountIdOf<T>,
            _new_owner: AccountIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            // Proposes or confirms a change of owner address.
            // If invoked by the current owner, proposes a new owner address for confirmation. If the proposed address is the
            // current owner address, revokes any existing proposal.
            // If invoked by the previously proposed address, with the same proposal, changes the current owner address to be
            // that proposed address.
            // following https://github.com/filecoin-project/specs-actors/blob/57195d8909b1c366fd1af41de9e92e11d7876177/actors/builtin/miner/miner_actor.go#L224
            unimplemented!()
        }
    }
}

#[derive(Encode, Decode)]
pub struct MinerInfo<
    AccountId: Encode + Decode + Eq + PartialEq,
    BlockNumber: Encode + Decode + Eq + PartialEq,
    PeerId: Encode + Decode + Eq + PartialEq,
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
    /// Update to this worker address to at defined time
    pending_worker: Option<WorkerKeyChange<AccountId, BlockNumber>>,
    /// Update to this owner address when it confirms
    pending_owner: Option<AccountId>,
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
