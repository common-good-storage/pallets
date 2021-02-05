use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn it_creates_miner() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 0;
        let worker: u64 = 1;
        let peer_id = vec![1, 32];
        assert_ok!(Miner::create(Origin::signed(1), owner, worker, peer_id));
        assert_eq!(Miner::miner_index(), Some(1));
    });
}
