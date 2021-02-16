// Imports created by construct_runtime macros are unresolved by rust analyzer
use crate as pallet_miner;
use crate::mock::{new_test_ext, Event, Miner, Origin, System, Test};
use crate::{Error, MinerControllers};
use frame_support::{assert_noop, assert_ok, dispatch::DispatchResultWithPostInfo};
use pallet_common::{AccountIdConversion, MinerId};

const WORKER: u64 = 33;
const PEERID_BYTE: u8 = 9;
const FIRST_MINER_ADDR: u64 = 1590839634285;

// Utility functions
//
// TODO: add genesis config in pallet and build test_ext with it
fn create_miner_for(
    owner: <Test as frame_system::Config>::AccountId,
) -> DispatchResultWithPostInfo {
    Miner::create(Origin::signed(1), owner, WORKER, vec![PEERID_BYTE])
}

#[test]
fn first_miner_addr_is_correct() {
    new_test_ext().execute_with(|| {
        let new_miner_addr: <Test as frame_system::Config>::AccountId = MinerId(1).into_account();
        assert_eq!(new_miner_addr, FIRST_MINER_ADDR);
    })
}

#[test]
fn create_miner() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 0;
        let worker: u64 = 1;
        let peer_id = vec![1, 32];
        let expected_miner_index = 1;

        // this needs to be set in order to read back the System::events later on, Events are not
        // populated on genesis - unsure why
        // https://github.com/paritytech/substrate/blob/master/frame/system/src/lib.rs#L1122
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
        assert_eq!(new_miner_info.controllers.len(), 0);
        assert_eq!(System::event_count(), 1);

        assert_eq!(
            System::events()
                .pop()
                .map(|e| e.event)
                .expect("EventRecord should have event field"),
            Event::pallet_miner(pallet_miner::Event::MinerCreated(new_miner_addr))
        );
    });
}

#[test]
fn it_creates_worker_change_request_with_valid_signer_and_new_worker() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        // set initial block for events and calculation for effective_at
        let block = 1;
        System::set_block_number(block);

        let new_worker: u64 = 99;
        let new_controllers = MinerControllers::Override(vec![1, 2, 3]);
        assert_ok!(Miner::change_worker_address(
            Origin::signed(owner),
            FIRST_MINER_ADDR,
            new_worker,
            new_controllers.clone()
        ));

        let miner_key_change = Miner::miners(FIRST_MINER_ADDR)
            .unwrap()
            .pending_worker
            .unwrap();

        assert_eq!(miner_key_change.new_worker, new_worker);
        assert_eq!(
            miner_key_change.effective_at,
            block + <Test as pallet_miner::Config>::BlockDelay::get()
        );
        assert_eq!(
            System::events()
                .pop()
                .map(|e| e.event)
                .expect("EventRecord should have event field"),
            Event::pallet_miner(pallet_miner::Event::WorkerChangeRequested(
                FIRST_MINER_ADDR,
                new_worker,
                new_controllers
            ))
        )
    });
}

#[test]
fn it_clears_worker_change_request_with_valid_signer_and_old_worker() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        // set initial pending_worker request
        let new_worker: u64 = 99;
        assert_ok!(Miner::change_worker_address(
            Origin::signed(owner),
            FIRST_MINER_ADDR,
            new_worker,
            MinerControllers::NoChange
        ));

        // clears existing pending_worker request with existing worker
        assert_ok!(Miner::change_worker_address(
            Origin::signed(owner),
            FIRST_MINER_ADDR,
            WORKER,
            MinerControllers::NoChange
        ));

        assert!(Miner::miners(FIRST_MINER_ADDR)
            .unwrap()
            .pending_worker
            .is_none());
    })
}

#[test]
fn no_change_to_controllers_without_override() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        // set initial pending_worker request
        let new_worker: u64 = 99;
        let new_controllers = vec![1, 2, 3];
        assert_ok!(Miner::change_worker_address(
            Origin::signed(owner),
            FIRST_MINER_ADDR,
            new_worker,
            MinerControllers::Override(new_controllers.clone())
        ));

        // clears existing pending_worker request with existing worker
        assert_ok!(Miner::change_worker_address(
            Origin::signed(owner),
            FIRST_MINER_ADDR,
            WORKER,
            MinerControllers::NoChange
        ));

        assert_eq!(
            Miner::miners(FIRST_MINER_ADDR).unwrap().controllers,
            new_controllers
        );
    })
}

#[test]
fn it_rejects_worker_change_request_with_invalid_signer() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        // set initial block for events
        System::set_block_number(1);

        // set initial pending_worker request
        let invalid_signer: u64 = 456;
        assert_noop!(
            Miner::change_worker_address(
                Origin::signed(invalid_signer),
                FIRST_MINER_ADDR,
                WORKER,
                MinerControllers::NoChange
            ),
            Error::<Test>::InvalidSigner
        );
    })
}

#[test]
fn it_accepts_effective_worker_change_trigger() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        // set initial block for events
        System::set_block_number(1);

        // set initial pending_worker request
        let new_worker: u64 = 99;
        assert_ok!(Miner::change_worker_address(
            Origin::signed(owner),
            FIRST_MINER_ADDR,
            new_worker,
            MinerControllers::NoChange
        ));

        System::set_block_number(10);

        assert_ok!(Miner::confirm_update_worker_key(
            Origin::signed(owner),
            FIRST_MINER_ADDR,
        ));

        let new_miner_info = Miner::miners(FIRST_MINER_ADDR).unwrap();

        assert_eq!(new_miner_info.worker, new_worker);
        assert!(new_miner_info.pending_worker.is_none());
        assert_eq!(
            System::events()
                .pop()
                .map(|e| e.event)
                .expect("EventRecord should have event field"),
            Event::pallet_miner(pallet_miner::Event::WorkerChanged(
                FIRST_MINER_ADDR,
                new_worker,
            ))
        )
    })
}

#[test]
fn it_rejects_trigger_before_effective_at() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        // set initial block for events
        System::set_block_number(1);

        // set initial pending_worker request
        let new_worker: u64 = 99;
        assert_ok!(Miner::change_worker_address(
            Origin::signed(owner),
            FIRST_MINER_ADDR,
            new_worker,
            MinerControllers::NoChange
        ));

        System::set_block_number(<Test as pallet_miner::Config>::BlockDelay::get());

        assert_noop!(
            Miner::confirm_update_worker_key(Origin::signed(owner), FIRST_MINER_ADDR,),
            Error::<Test>::IneffectiveRequest
        );
    })
}

#[test]
fn it_rejects_trigger_without_request() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        assert_noop!(
            Miner::confirm_update_worker_key(Origin::signed(owner), FIRST_MINER_ADDR,),
            Error::<Test>::NoRequest
        );
    })
}

#[test]
fn owner_can_create_owner_change_request() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        let new_owner: u64 = 234;
        assert_ok!(create_miner_for(owner));

        System::set_block_number(1);
        assert_ok!(Miner::change_owner_address(
            Origin::signed(owner),
            FIRST_MINER_ADDR,
            new_owner
        ));

        assert_eq!(
            Miner::miners(FIRST_MINER_ADDR).unwrap().pending_owner,
            Some(new_owner)
        );
        assert_eq!(
            System::events()
                .pop()
                .map(|e| e.event)
                .expect("EventRecord should have event field"),
            Event::pallet_miner(pallet_miner::Event::OwnerChangeRequested(
                FIRST_MINER_ADDR,
                new_owner,
            ))
        )
    })
}

#[test]
fn owner_cannot_create_owner_change_request_with_own_account() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        assert_ok!(create_miner_for(owner));

        assert_noop!(
            Miner::change_owner_address(Origin::signed(owner), FIRST_MINER_ADDR, owner,),
            Error::<Test>::IneffectiveRequest
        );
    })
}

#[test]
fn it_cannot_create_owner_change_request_with_invalid_signer() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        let new_owner: u64 = 234;
        let random_account: u64 = 789;
        assert_ok!(create_miner_for(owner));

        assert_noop!(
            Miner::change_owner_address(
                Origin::signed(random_account),
                FIRST_MINER_ADDR,
                new_owner
            ),
            Error::<Test>::InvalidSigner
        );
    })
}

#[test]
fn proposed_owner_can_confirm_change_request() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        let new_owner: u64 = 234;
        assert_ok!(create_miner_for(owner));

        assert_ok!(Miner::change_owner_address(
            Origin::signed(owner),
            FIRST_MINER_ADDR,
            new_owner
        ));

        System::set_block_number(1);
        assert_ok!(Miner::change_owner_address(
            Origin::signed(new_owner),
            FIRST_MINER_ADDR,
            new_owner
        ));

        assert_eq!(Miner::miners(FIRST_MINER_ADDR).unwrap().owner, new_owner);
        assert_eq!(Miner::miners(FIRST_MINER_ADDR).unwrap().pending_owner, None);
        assert_eq!(
            System::events()
                .pop()
                .map(|e| e.event)
                .expect("EventRecord should have event field"),
            Event::pallet_miner(pallet_miner::Event::OwnerChanged(
                FIRST_MINER_ADDR,
                new_owner,
            ))
        )
    })
}

#[test]
fn owner_can_revoke_existing_owner_change_request() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        let new_owner: u64 = 234;
        assert_ok!(create_miner_for(owner));

        assert_ok!(Miner::change_owner_address(
            Origin::signed(owner),
            FIRST_MINER_ADDR,
            new_owner
        ));

        assert_ok!(Miner::change_owner_address(
            Origin::signed(owner),
            FIRST_MINER_ADDR,
            new_owner
        ));

        assert_eq!(
            Miner::miners(FIRST_MINER_ADDR).unwrap().pending_owner,
            Some(new_owner)
        );

        System::set_block_number(1);

        assert_ok!(Miner::change_owner_address(
            Origin::signed(owner),
            FIRST_MINER_ADDR,
            owner
        ));

        assert_eq!(Miner::miners(FIRST_MINER_ADDR).unwrap().owner, owner);
        assert_eq!(Miner::miners(FIRST_MINER_ADDR).unwrap().pending_owner, None);
        assert_eq!(
            System::events()
                .pop()
                .map(|e| e.event)
                .expect("EventRecord should have event field"),
            Event::pallet_miner(pallet_miner::Event::OwnerChangeRequested(
                FIRST_MINER_ADDR,
                owner,
            ))
        )
    })
}

#[test]
fn it_changes_peer_id_with_valid_signer() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        let new_peer_id = vec![88];
        assert_ok!(create_miner_for(owner));

        // set initial block for events and calculation for effective_at
        let block = 1;
        System::set_block_number(block);

        assert_ok!(Miner::change_peer_id(
            Origin::signed(owner),
            FIRST_MINER_ADDR,
            new_peer_id.clone()
        ));

        let peer_id = Miner::miners(FIRST_MINER_ADDR).unwrap().peer_id;

        assert_eq!(peer_id, new_peer_id);
        assert_eq!(
            System::events()
                .pop()
                .map(|e| e.event)
                .expect("EventRecord should have event field"),
            Event::pallet_miner(pallet_miner::Event::PeerIdChanged(
                FIRST_MINER_ADDR,
                new_peer_id
            ))
        )
    });
}

#[test]
fn it_rejects_change_to_peer_id_with_invalid_signer() {
    new_test_ext().execute_with(|| {
        let owner: u64 = 123;
        let invalid_signer: u64 = 234;
        let new_peer_id = vec![88];
        assert_ok!(create_miner_for(owner));

        // set initial block for events and calculation for effective_at
        let block = 1;
        System::set_block_number(block);

        assert_noop!(
            Miner::change_peer_id(
                Origin::signed(invalid_signer),
                FIRST_MINER_ADDR,
                new_peer_id
            ),
            Error::<Test>::InvalidSigner
        );
    });
}
