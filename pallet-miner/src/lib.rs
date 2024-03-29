#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

use codec::{Decode, Encode};
pub use sp_std::vec::Vec;

// `pallet::Module` is created by `pallet` macro
pub use pallet::{Config, Error, Event, MinerIndex, Miners, Module, Pallet};

#[frame_support::pallet]
pub mod pallet {
    use super::{MinerControllers, MinerInfo, Vec, WorkerKeyChange};
    use frame_support::pallet_prelude::{
        ensure, Blake2_128Concat, DispatchResultWithPostInfo, Get, Hooks, IsType, PhantomData,
        StorageMap, StorageValue,
    };
    use frame_system::pallet_prelude::{ensure_signed, BlockNumberFor, OriginFor};

    use pallet_common::{AccountIdConversion, MinerId, Power};
    pub type MinerAccountId<T> = <<T as Config>::Power as Power>::AccountId;
    pub type PeerId<T> = <<T as Config>::Power as Power>::PeerId;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Power: Power;
        type BlockDelay: Get<BlockNumberFor<Self>>;
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
        MinerAccountId<T>,
        MinerInfo<T::AccountId, BlockNumberFor<T>, PeerId<T>>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn miner_index)]
    pub type MinerIndex<T: Config> = StorageValue<_, u32>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(MinerAccountId<T> = "MinerAccountId", T::AccountId = "AccountID", PeerId<T> = "PeerId")]
    pub enum Event<T: Config> {
        /// Emits new miner address
        MinerCreated(MinerAccountId<T>),
        /// Emits miner address, requested change in worker address and controllers address update
        WorkerChangeRequested(
            MinerAccountId<T>,
            T::AccountId,
            MinerControllers<T::AccountId>,
        ),
        /// Emits miner address and new worker address
        WorkerChanged(MinerAccountId<T>, T::AccountId),
        /// Emits miner address and new PeerId to update to
        PeerIdChanged(MinerAccountId<T>, PeerId<T>),
        /// Emits miner address and new owner address to update to
        OwnerChangeRequested(MinerAccountId<T>, T::AccountId),
        /// Emits miner address and new owner address
        OwnerChanged(MinerAccountId<T>, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        Overflow,
        ClaimsNotSet,
        NoSuchMiner,
        InvalidSigner,
        NoRequest,
        IneffectiveRequest,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Benchmark not accurate
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create(
            origin: OriginFor<T>,
            owner: T::AccountId,
            worker: T::AccountId,
            peer_id: PeerId<T>,
        ) -> DispatchResultWithPostInfo {
            // following https://github.com/filecoin-project/specs-actors/blob/57195d8909b1c366fd1af41de9e92e11d7876177/actors/builtin/miner/miner_actor.go#L97
            // Note: This replaces the external call to the power actor and register the miner
            // claims with `Power::register_new_miner` method
            //
            // This allows signer to be accounts other than owner so potential services can be
            // built to create miners for owners. Signer pays for the transaction costs and not
            // value is staked by creating miner.
            ensure_signed(origin)?;

            let mut miner_index = MinerIndex::<T>::get().unwrap_or_default();
            miner_index = miner_index.checked_add(1).ok_or(Error::<T>::Overflow)?;
            let miner: MinerAccountId<T> = MinerId(miner_index).into_account();
            MinerIndex::<T>::put(miner_index);

            T::Power::register_new_miner(&miner).ok_or(Error::<T>::ClaimsNotSet)?;

            let miner_info = MinerInfo {
                owner,
                worker,
                controllers: Vec::new(),
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
            origin: OriginFor<T>,
            miner: MinerAccountId<T>,
            new_worker: T::AccountId,
            new_controllers: MinerControllers<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            // following https://github.com/filecoin-project/specs-actors/blob/57195d8909b1c366fd1af41de9e92e11d7876177/actors/builtin/miner/miner_actor.go#L225

            let signer = ensure_signed(origin)?;
            let mut miner_info =
                Miners::<T>::try_get(&miner).map_err(|_| Error::<T>::NoSuchMiner)?;

            // Ensure that the caller is the owner of the miner to make any updates
            ensure!(signer == miner_info.owner, Error::<T>::InvalidSigner);

            // This is different from filecoin miner_actor impl where ChangeWorkerAddress will ALWAYS overwrite the existing control addresses
            // with the control addresses passed in the params. Instead we match MinerControllers
            // Variant here
            if let MinerControllers::Override(controllers) = new_controllers.clone() {
                miner_info.controllers = controllers;
            }

            // A worker change will be scheduled if the worker passed in the params is different from the existing worker.
            if miner_info.worker != new_worker {
                miner_info.pending_worker = Some(WorkerKeyChange {
                    new_worker: new_worker.clone(),
                    effective_at: <frame_system::Module<T>>::block_number() + T::BlockDelay::get(),
                });
            } else {
                miner_info.pending_worker = None;
            }

            Miners::<T>::insert(&miner, miner_info);
            Self::deposit_event(Event::WorkerChangeRequested(
                miner,
                new_worker,
                new_controllers,
            ));
            Ok(().into())
        }

        // Benchmark not accurate
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn change_peer_id(
            origin: OriginFor<T>,
            miner: MinerAccountId<T>,
            new_peer_id: PeerId<T>,
        ) -> DispatchResultWithPostInfo {
            // following https://github.com/filecoin-project/specs-actors/blob/57195d8909b1c366fd1af41de9e92e11d7876177/actors/builtin/miner/miner_actor.go#L266

            let signer = ensure_signed(origin)?;
            Miners::<T>::try_mutate(&miner, |maybe_miner_info| -> DispatchResultWithPostInfo {
                let miner_info = maybe_miner_info.as_mut().ok_or(Error::<T>::NoSuchMiner)?;
                ensure!(
                    signer == miner_info.owner
                        || signer == miner_info.worker
                        || miner_info
                            .controllers
                            .iter()
                            .any(|account| account == &signer),
                    Error::<T>::InvalidSigner
                );
                miner_info.peer_id = new_peer_id.clone();
                Self::deposit_event(Event::PeerIdChanged(miner.clone(), new_peer_id));
                Ok(().into())
            })
        }

        // Benchmark not accurate
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn confirm_update_worker_key(
            origin: OriginFor<T>,
            miner: MinerAccountId<T>,
        ) -> DispatchResultWithPostInfo {
            // following https://github.com/filecoin-project/specs-actors/blob/57195d8909b1c366fd1af41de9e92e11d7876177/actors/builtin/miner/miner_actor.go#L205

            // Allow any paying accounts to trigger the change set by owner
            ensure_signed(origin)?;

            Miners::<T>::try_mutate(&miner, |maybe_miner_info| -> DispatchResultWithPostInfo {
                let miner_info = maybe_miner_info.as_mut().ok_or(Error::<T>::NoSuchMiner)?;
                if let Some(key_change) = &miner_info.pending_worker {
                    // Can only change to new_worker addr after effective_at block number
                    if key_change.effective_at <= <frame_system::Module<T>>::block_number() {
                        let new_worker = key_change.new_worker.clone();
                        miner_info.worker = new_worker.clone();
                        miner_info.pending_worker = None;
                        Self::deposit_event(Event::WorkerChanged(miner.clone(), new_worker));
                        Ok(().into())
                    } else {
                        Err(Error::<T>::IneffectiveRequest.into())
                    }
                } else {
                    Err(Error::<T>::NoRequest.into())
                }
            })
        }

        // Benchmark not accurate
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn change_owner_address(
            origin: OriginFor<T>,
            miner: MinerAccountId<T>,
            new_owner: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            // If invoked by the current owner, proposes a new owner address for confirmation. If the proposed address is the
            // current owner address, revokes any existing proposal.
            // If invoked by the previously proposed address, with the same proposal, changes the current owner address to be
            // that proposed address.
            // following https://github.com/filecoin-project/specs-actors/blob/57195d8909b1c366fd1af41de9e92e11d7876177/actors/builtin/miner/miner_actor.go#L224
            let signer = ensure_signed(origin)?;

            let mut miner_info =
                Miners::<T>::try_get(&miner).map_err(|_| Error::<T>::NoSuchMiner)?;

            match miner_info.pending_owner {
                Some(proposed_owner) if new_owner == proposed_owner && signer == proposed_owner => {
                    // New owner confirms proposed
                    miner_info.owner = signer;
                    miner_info.pending_owner = None;
                    Miners::<T>::insert(miner.clone(), miner_info);
                    Self::deposit_event(Event::<T>::OwnerChanged(miner, new_owner));
                }
                Some(_) if signer == miner_info.owner && signer == new_owner => {
                    // Existing owner cancels the ownership change
                    miner_info.pending_owner = None;
                    Miners::<T>::insert(miner.clone(), miner_info);
                    Self::deposit_event(Event::<T>::OwnerChangeRequested(miner, new_owner));
                }
                Some(_) if signer == miner_info.owner => {
                    // Override existing proposal
                    miner_info.pending_owner = Some(new_owner.clone());
                    Miners::<T>::insert(miner.clone(), miner_info);
                    Self::deposit_event(Event::<T>::OwnerChangeRequested(miner, new_owner));
                }
                None if signer == miner_info.owner && new_owner == miner_info.owner => {
                    // Attempted to change ownership to themselves
                    return Err(Error::<T>::IneffectiveRequest.into());
                }
                None if signer == miner_info.owner => {
                    // Initiate ownership transfer of the miner
                    miner_info.pending_owner = Some(new_owner.clone());
                    Miners::<T>::insert(miner.clone(), miner_info);
                    Self::deposit_event(Event::<T>::OwnerChangeRequested(miner, new_owner));
                }
                Some(_) | None => {
                    let is_current_owner = signer == miner_info.owner;
                    let is_proposed_owner = miner_info
                        .pending_owner
                        .map(|po| po == signer)
                        .unwrap_or(false);

                    assert!(!is_current_owner && !is_proposed_owner);
                    return Err(Error::<T>::InvalidSigner.into());
                }
            }

            Ok(().into())
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
    /// Other addresses that can sign messages on behalf of the miner,
    /// a limit for max number of controllers to be added
    controllers: Vec<AccountId>,
    /// Miner's libp2p PeerId
    peer_id: PeerId,
    /// Update to this worker address to at defined time
    pending_worker: Option<WorkerKeyChange<AccountId, BlockNumber>>,
    /// Update to this owner address when it confirms
    pending_owner: Option<AccountId>,
}

#[derive(Encode, Decode, Debug)]
pub struct WorkerKeyChange<
    AccountId: Encode + Decode + Eq + PartialEq,
    BlockNumber: Encode + Decode + Eq + PartialEq,
> {
    /// New Worker Address to be updated
    new_worker: AccountId,
    /// Time after which confirm_update_worker_key will trigger updates to MinerInfo
    effective_at: BlockNumber,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
pub enum MinerControllers<AccountId> {
    /// Miner controller accounts should be set as follows
    Override(Vec<AccountId>),
    /// Miner controller accounts should not change
    NoChange,
}
