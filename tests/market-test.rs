pub mod context;

use ito_contract::{admin::AdminBridge, bridge::SeriesBridge, ONE_NEAR};
use near_sdk::testing_env;

use context::{alice, bob, carol, create_series, owner, setup_contract, STORAGE_FOR_CREATE_SERIES};

#[test]
#[should_panic(expected = "Campground: Attached deposit needs to be equal to ITO price")]
fn contract_should_reject_buying_with_invalid_amount() {
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
        Some(1000.into()),
        Some(10),
        None,
        None
    );
    testing_env!(context
        .predecessor_account_id(bob())
        .attached_deposit(500)
        .build());

    // Panics
    contract.nft_buy_series("1".to_string(), carol());
}

#[test]
#[should_panic(expected = "Campground: Attached deposit is less than minimum buying fee")]
fn contract_should_reject_when_buying_with_invalid_fee() {
    let (mut context, mut contract) = setup_contract();
    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    let min_fee = contract.campground_minimum_fee_yocto_near;
    create_series(
        &mut contract,
        "CampgroundTest",
        Some(1647109675),
        Some(1647216000),
        Some((min_fee - 1).into()),
        Some(10),
        None,
        None
    );
    testing_env!(context
        .predecessor_account_id(bob())
        .attached_deposit(contract.campground_minimum_fee_yocto_near - 1)
        .build());

    contract.nft_buy_series("1".to_string(), carol());
}

#[test]
fn contract_should_allow_account_to_buy_with_just_enough_fee() {
    let (mut context, mut contract) = setup_contract();
    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    let min_fee = contract.campground_minimum_fee_yocto_near;
    create_series(
        &mut contract,
        "CampgroundTest",
        Some(1647109675),
        Some(1647216000),
        Some(min_fee.into()),
        Some(10),
        None,
        None
    );
    testing_env!(context
        .predecessor_account_id(bob())
        .attached_deposit(min_fee)
        .build());

    contract.nft_buy_series(String::from("1"), carol());
}

#[test]
fn contract_should_allow_account_to_buy_with_one_near() {
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
        Some(ONE_NEAR.into()),
        Some(10),
        None,
        None
    );
    testing_env!(context
        .predecessor_account_id(bob())
        .attached_deposit(ONE_NEAR)
        .build());

    contract.nft_buy_series("1".into(), carol());

    testing_env!(context
        .predecessor_account_id(carol())
        .attached_deposit(ONE_NEAR)
        .build());

    contract.nft_buy_series("1".into(), carol());

    let get_account_trails = contract.tokens_per_owner.get(&carol()).unwrap();
    let trails_as_vec = get_account_trails.to_vec();
    assert_eq!(get_account_trails.len(), 2);
    assert_eq!(trails_as_vec.get(0).unwrap(), &"1:1".clone());
    assert_eq!(trails_as_vec.get(1).unwrap(), &"1:2".clone());

    let trails_by_id = contract.tokens_by_id.get(&"1:1".into()).unwrap();
    assert_eq!(trails_by_id.owner_id, carol());
    assert_eq!(trails_by_id.token_id, "1");

    assert!(contract.is_owner(&"1".into(), &carol()));
    assert!(!(contract.is_owner(&"1".into(), &bob())));
    assert!(!(contract.is_owner(&"2".into(), &carol())));

    let get_trails_by_owner = contract.get_all_trails_by_owner(&carol());
    assert_eq!(get_trails_by_owner.len(), 1);
    let data = get_trails_by_owner.get(0).unwrap();
    assert_eq!(data.creator_id, alice());

    // Re run test to verify ownership
    let get_trails_by_owner = contract.get_all_trails_by_owner(&carol());
    assert_eq!(get_trails_by_owner.len(), 1);
}

#[test]
#[should_panic(expected = "Campground: Buying operation is invalid")]
fn contract_should_reject_when_buying_with_campground_fee_greater_than_100() {
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
        Some(ONE_NEAR.into()),
        Some(10),
        None,
        None
    );
    testing_env!(context
        .predecessor_account_id(bob())
        .attached_deposit(ONE_NEAR)
        .build());

    contract.nft_buy_series("1".to_string(), carol());
}

#[test]
#[should_panic(expected = "Campground: a fee needs to be paid")]
fn contract_should_reject_when_campground_fee_is_missing() {
    let (mut context, mut contract) = setup_contract();

    testing_env!(context
        .predecessor_account_id(owner())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());
    contract.change_campground_minimum_fee(0);
    contract.change_campground_fee(0);

    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    create_series(
        &mut contract,
        "CampgroundTest",
        Some(1647109675),
        Some(1647216000),
        Some(ONE_NEAR.into()),
        Some(10),
        None,
        None
    );
    testing_env!(context
        .predecessor_account_id(bob())
        .attached_deposit(ONE_NEAR)
        .build());

    contract.nft_buy_series("1".to_string(), carol());
}

#[test]
#[should_panic(expected = "Campground: Trail is not allowed to be minted by user")]
fn contract_should_reject_when_trail_is_not_mintable() {
    let (mut context, mut contract) = setup_contract();

    testing_env!(context
        .predecessor_account_id(owner())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    create_series(
        &mut contract,
        "CampgroundTest",
        Some(1647109675),
        Some(1647216000),
        Some(ONE_NEAR.into()),
        Some(10),
        None,
        Some(false)
    );
    testing_env!(context
        .predecessor_account_id(bob())
        .attached_deposit(ONE_NEAR)
        .build());

    contract.nft_buy_series("1".to_string(), carol());
}
