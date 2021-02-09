use crate::mock::{new_test_ext, Power};
use frame_support::assert_ok;
use pallet_common::{Claim, Power as PowerTrait};

#[test]
fn it_register_new_miner() {
    new_test_ext().execute_with(|| {
        let miner_account: u64 = 1;
        let expected_claim = Claim::<u128>::default();

        assert_ok! {Power::register_new_miner(&miner_account).ok_or("Registration failed")};

        let claim = Power::claims(miner_account);
        assert_eq!(claim.is_some(), true);
        assert_eq!(claim.unwrap(), expected_claim);
    });
}
