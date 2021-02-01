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
    #[pallet::getter(fn placeholder)]
    pub type Placeholder<T: Config> = StorageValue<_, u64>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId")]
    pub enum Event<T: Config> {
		// Placeholder
        MinerCreated(T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
		// Placeholder
        NoneValue,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
			unimplemented!()
		}
    }
}

