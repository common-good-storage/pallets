#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::{fmt::Debug, prelude::*};

// TODO: Do we define these as concrete types in the runtime and have these in Config::PeerId
// associative types?

/// Libp2p PeerId
pub type PeerId = Vec<u8>;

/// Libp2p Multiaddress
pub type Multiaddress = Vec<u8>;
