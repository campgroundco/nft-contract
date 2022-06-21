pub mod context;

use ito_contract::admin::AdminBridge;
use near_sdk::testing_env;

use context::{alice, new_treasury, owner, setup_contract, STORAGE_FOR_CREATE_SERIES};
use ito_contract::sub_admin::SubAdminBridge;
use ito_contract::vars::SUB_ADMIN_ADDRESS;

#[test]
fn contract_should_change_campground_fee() {
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
fn contract_should_reject_non_owner_changing_campground_fee() {
    let (mut context, mut contract) = setup_contract();
    assert_eq!(contract.campground_fee, 5);
    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    contract.change_campground_fee(10);
}

#[test]
fn contract_should_change_campground_treasury_address() {
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
fn contract_should_reject_when_non_owner_changing_campground_treasury_address() {
    let (mut context, mut contract) = setup_contract();
    assert_eq!(contract.campground_fee, 5);
    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    contract.change_campground_treasury_address(new_treasury());
}

#[test]
fn contract_should_change_campground_minimum_fee() {
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
fn contract_should_reject_when_non_owner_changing_campground_minimum_fee() {
    let (mut context, mut contract) = setup_contract();
    assert_eq!(contract.campground_fee, 5);
    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    contract.change_campground_minimum_fee(50000000);
}

///
/// SUB ADMIN tests
///

#[test]
fn sub_admin_should_be_able_to_remove_trail() {
    let (mut context, mut contract) = setup_contract();
    testing_env!(context
        .predecessor_account_id(owner())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    contract.add_setting(String::from(SUB_ADMIN_ADDRESS), String::from("alice"));
    contract.nonmintable_trails.insert(&String::from("1"));

    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    contract.remove_trail_from_nonmintable_list(String::from("1"));
}

#[test]
#[should_panic(expected = "Campground: Only Sub-admin can execute")]
fn any_user_cantremove_from_non_mintable_trail() {
    let (mut context, mut contract) = setup_contract();
    testing_env!(context
        .predecessor_account_id(owner())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    contract.add_setting(String::from(SUB_ADMIN_ADDRESS), String::from("alice"));
    contract.nonmintable_trails.insert(&String::from("1"));
    contract.remove_trail_from_nonmintable_list(String::from("1"));
}