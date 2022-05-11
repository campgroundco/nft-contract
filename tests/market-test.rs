pub mod context;

use ito_contract::{
    admin::AdminBridge, bridge::SeriesBridge, internal::calculate_yocto_near, BUY_STORAGE, ONE_NEAR,
};
use near_sdk::{json_types::U128, testing_env};

use context::{alice, bob, carol, create_series, owner, setup_contract, STORAGE_FOR_CREATE_SERIES};

#[test]
#[should_panic(expected = "Campground: Attached deposit is less than price")]
fn test_buy_invalid_amount() {
    let (mut context, mut contract) = setup_contract();
    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    create_series(
        &mut contract,
        "CampgroundTest",
        Some(1647109675),
        Some(1647216000),
        Some(U128::from(1000 as u128)),
        Some(10),
        None,
    );
    testing_env!(context
        .predecessor_account_id(bob())
        .attached_deposit(500 as u128)
        .build());

    // Panics
    contract.nft_buy_series("1".to_string(), carol());
}

#[test]
#[should_panic(expected = "Campground: Attached deposit is less than minimum buying fee")]
fn test_buy_invalid_fee() {
    let (mut context, mut contract) = setup_contract();
    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    create_series(
        &mut contract,
        "CampgroundTest",
        Some(1647109675),
        Some(1647216000),
        Some(U128::from(calculate_yocto_near(0.01))),
        Some(10),
        None,
    );
    testing_env!(context
        .predecessor_account_id(bob())
        .attached_deposit(contract.campground_minimum_fee_yocto_near - 1)
        .build());

    contract.nft_buy_series("1".to_string(), carol());
}

#[test]
fn test_buy_just_enough_fee() {
    let (mut context, mut contract) = setup_contract();
    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    create_series(
        &mut contract,
        "CampgroundTest",
        Some(1647109675),
        Some(1647216000),
        Some(U128::from(calculate_yocto_near(0.01))),
        Some(10),
        None,
    );
    testing_env!(context
        .predecessor_account_id(bob())
        .attached_deposit(contract.campground_minimum_fee_yocto_near)
        .build());

    contract.nft_buy_series(String::from("1"), carol());
}

#[test]
fn test_buy_one_near() {
    let (mut context, mut contract) = setup_contract();
    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(6920000000000000000000)
        .build());

    create_series(
        &mut contract,
        "CampgroundTest",
        Some(1647109675),
        Some(1647216000),
        Some(U128::from(ONE_NEAR)),
        Some(10),
        None,
    );
    testing_env!(context
        .predecessor_account_id(bob())
        .attached_deposit(ONE_NEAR + BUY_STORAGE)
        .build());

    contract.nft_buy_series(String::from("1"), carol());

    testing_env!(context
        .predecessor_account_id(carol())
        .attached_deposit(ONE_NEAR + BUY_STORAGE)
        .build());

    contract.nft_buy_series(String::from("1"), carol());

    let get_account_trails = contract.tokens_per_owner.get(&carol()).unwrap();
    let trails_as_vec = get_account_trails.to_vec();
    assert_eq!(get_account_trails.len(), 2);
    assert_eq!(trails_as_vec.get(0).unwrap(), &String::from("1:1"));
    assert_eq!(trails_as_vec.get(1).unwrap(), &String::from("1:2"));

    let trails_by_id = contract.tokens_by_id.get(&String::from("1:1")).unwrap();
    assert_eq!(trails_by_id.owner_id, carol());
    assert_eq!(trails_by_id.token_id, "1");

    assert!(contract.is_owner(&String::from("1"), &carol()));
    assert!(!(contract.is_owner(&String::from("1"), &bob())));
    assert!(!(contract.is_owner(&String::from("2"), &carol())));

    let get_trails_by_owner = contract.get_all_trails_by_owner(&carol());
    assert_eq!(get_trails_by_owner.len(), 1);
    let data = get_trails_by_owner.get(0).unwrap();
    assert_eq!(data.creator_id, alice());

    // Re run test to verify ownership
    let get_trails_by_owner = contract.get_all_trails_by_owner(&carol());
    assert_eq!(get_trails_by_owner.len(), 1);
}

#[test]
#[ignore = "reason"]
fn it_should() {
    let (mut context, mut contract) = setup_contract();

    testing_env!(context
        .predecessor_account_id(owner())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());
    contract.change_campground_minimum_fee(0);
    contract.change_campground_fee(120);

    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    create_series(
        &mut contract,
        "CampgroundTest",
        Some(1647109675),
        Some(1647216000),
        Some(1000.into()),
        Some(10),
        None,
    );
    testing_env!(context
        .predecessor_account_id(bob())
        // .attached_deposit(1000)
        .attached_deposit(ONE_NEAR + BUY_STORAGE)
        .build());

    contract.nft_buy_series("1".to_string(), carol());
}
