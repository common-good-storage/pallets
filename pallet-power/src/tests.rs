use crate::mock::{new_test_ext, Power};
use pallet_common::{Claim, Power as PowerTrait};

#[test]
fn register_new_miner() {
    new_test_ext().execute_with(|| {
        let miner_account: u64 = 1;
        let expected_claim = Claim::<u128>::default();

        Power::register_new_miner(&miner_account).expect("Registration failed");

        let claim = Power::claims(miner_account);
        assert_eq!(claim.is_some(), true);
        assert_eq!(claim.unwrap(), expected_claim);
    });
}
