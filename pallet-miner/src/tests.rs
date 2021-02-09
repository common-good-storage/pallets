use crate::mock::{new_test_ext, Miner, Origin, System, Test};
use crate::{AccountIdConversion, MinerId};
use frame_support::assert_ok;

#[test]
fn it_creates_miner() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 0;
        let worker: u64 = 1;
        let peer_id = vec![1, 32];
        let expected_miner_index = 1;

        // this needs to be set in order to read back the System::events later on, unsure why
        System::set_block_number(1);

        assert_ok!(Miner::create(
            Origin::signed(1),
            owner,
            worker,
            peer_id.clone()
        ));

        let miner_index = Miner::miner_index();
        let new_miner_addr: <Test as frame_system::Config>::AccountId =
            MinerId(miner_index.unwrap()).into_account();
        let new_miner_info = Miner::miners(new_miner_addr).unwrap();

        assert_eq!(Miner::miner_index(), Some(expected_miner_index));
        assert_eq!(new_miner_info.owner, owner);
        assert_eq!(new_miner_info.worker, worker);
        assert_eq!(new_miner_info.peer_id, peer_id);
        assert_eq!(System::event_count(), 1);
        /*assert_eq!(
            System::events(),
            vec![frame_system::EventRecord {
                phase: Phase::Initialization,
                event: crate::Event::pallet_miner(crate::Event::MinerCreated(1590839634285)),
                topics: []
            }]
        );*/
    });
}
