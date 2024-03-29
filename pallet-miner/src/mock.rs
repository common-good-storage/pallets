use crate as pallet_miner;
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system::{Module, Call, Config, Storage, Event<T>},
            Power: pallet_power::{Module, Storage},
            Miner: pallet_miner::{Module, Call, Storage, Event<T>},
        }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

// not sure where the Call, Event, PalletInfo comes from
impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
}

impl pallet_power::Config for Test {
    type PeerId = Vec<u8>;
    type StoragePower = u128;
}

parameter_types! {
    pub BlockDelay: u64 = 5;
}

impl pallet_miner::Config for Test {
    type Event = Event;
    type Power = Power;
    type BlockDelay = BlockDelay;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}
