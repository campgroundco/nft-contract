pub mod context;

use ito_contract::admin::AdminBridge;
use near_sdk::testing_env;

use context::{alice, new_treasury, owner, setup_contract, STORAGE_FOR_CREATE_SERIES};

#[test]
fn it_should_change_campground_fee() {
    let (mut context, mut contract) = setup_contract();
    assert_eq!(contract.campground_fee, 5);
    testing_env!(context
        .predecessor_account_id(owner())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    contract.change_campground_fee(10);
    assert_eq!(contract.campground_fee, 10);
}

#[test]
#[should_panic(expected = "Campground: Only contract owner can execute")]
fn it_should_panic_when_non_owner_changes_campground_fee() {
    let (mut context, mut contract) = setup_contract();
    assert_eq!(contract.campground_fee, 5);
    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    contract.change_campground_fee(10);
}

#[test]
fn it_should_change_campground_treasury_address() {
    let (mut context, mut contract) = setup_contract();
    assert_eq!(contract.campground_fee, 5);
    testing_env!(context
        .predecessor_account_id(owner())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    contract.change_campground_treasury_address(new_treasury());
    assert_eq!(contract.campground_treasury_address, new_treasury());
}

#[test]
#[should_panic(expected = "Campground: Only contract owner can execute")]
fn it_should_panic_when_non_owner_changes_campground_treasury_address() {
    let (mut context, mut contract) = setup_contract();
    assert_eq!(contract.campground_fee, 5);
    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    contract.change_campground_treasury_address(new_treasury());
}

#[test]
fn it_should_change_campground_minimum_fee() {
    let (mut context, mut contract) = setup_contract();
    assert_eq!(contract.campground_fee, 5);
    testing_env!(context
        .predecessor_account_id(owner())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    contract.change_campground_minimum_fee(50000000);
    assert_eq!(contract.campground_minimum_fee_yocto_near, 50000000);
}

#[test]
#[should_panic(expected = "Campground: Only contract owner can execute")]
fn it_should_panic_when_non_owner_changes_campground_minimum_fee() {
    let (mut context, mut contract) = setup_contract();
    assert_eq!(contract.campground_fee, 5);
    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    contract.change_campground_minimum_fee(50000000);
}
