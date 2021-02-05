use crate::mock::*;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Read pallet storage and assert an expected result.
        assert_eq!(Power::miner_count(), None);
    });
}
