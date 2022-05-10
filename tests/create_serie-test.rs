pub mod context;

use ito_contract::{bridge::SeriesBridge, internal::calculate_yocto_near, Contract};
use near_sdk::{json_types::U128, testing_env};

use context::{
    alice, bob, carol, create_series, get_context, owner, setup_contract, treasury,
    STORAGE_FOR_CREATE_SERIES,
};

#[test]
fn test_new() {
    let mut context = get_context(alice());
    testing_env!(context.build());
    let contract = Contract::new_default_meta(owner(), treasury());
    testing_env!(context.is_view(true).build());
    assert_eq!(contract.get_owner(), &owner());
    assert_eq!(contract.campground_fee, 5 as u64);
}

#[test]
fn create_trail_series() {
    let (mut context, mut contract) = setup_contract();

    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    let trail_series = create_series(
        &mut contract,
        "CampgroundTest",
        Some(1647109675),
        Some(1647216000),
        Some(U128::from(1 * 10u128.pow(24))),
        None,
        None,
    );

    let trail_by_id = contract.get_trail_by_id(&String::from("1"));
    assert_eq!(trail_series.series.creator_id, trail_by_id.creator_id);
    assert_eq!(trail_series.series.price, trail_by_id.price);
    assert_eq!(
        trail_series.series.metadata.expires_at,
        trail_by_id.metadata.expires_at
    );
    assert_eq!(
        trail_series.series.metadata.starts_at,
        trail_by_id.metadata.starts_at
    );
    assert_eq!(
        trail_by_id.metadata.resources.get(0).unwrap().media,
        "CampgroundTest.png"
    );
}

#[test]
#[should_panic(expected = "Campground: price higher than 1000000000000000000000000000000000")]
fn create_trail_series_invalid_price() {
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
        Some(U128::from(1_000_000_001 * 10u128.pow(24))),
        None,
        None,
    );
}

#[test]
#[should_panic(expected = "Campground: At least 1 ticket is required per trail series")]
fn create_trail_series_invalid_ticket_amount() {
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
        Some(U128::from(1 * 10u128.pow(24))),
        Some(0 as u64),
        None,
    );
}

#[test]
#[should_panic(expected = "Campground: At least 1 resource is needed per trail")]
fn create_trail_series_invalid_resources_amount() {
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
        Some(U128::from(1 * 10u128.pow(24))),
        Some(1 as u64),
        Some(vec![]),
    );
}

#[test]
#[should_panic(expected = "Campground: Trail is not mintable")]
fn test_minting() {
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
        Some(U128::from(1 * 10u128.pow(24))),
        Some(1),
        None,
    );
    let trail_by_id = contract.get_trail_by_id(&String::from("1"));
    assert_eq!(trail_by_id.is_mintable, true);
    contract.nft_mint(String::from("1"), bob());

    let track_by_owner = contract.trail_tickets_for_owner(bob(), None, None);
    assert_eq!(track_by_owner.len(), 1);
    println!("{}", track_by_owner.get(0).unwrap().series.is_mintable);

    // Panics
    contract.nft_mint(String::from("1"), bob());
}

fn test_copies_and_buys_internal() -> Contract {
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
    let nft_mint_1 = contract.nft_mint(String::from("1"), bob());
    assert_eq!(nft_mint_1, "1:1");

    testing_env!(context
        .predecessor_account_id(bob())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES + calculate_yocto_near(0.1))
        .build());

    // Panics
    let nft_mint_2 = contract.nft_buy_series(String::from("1"), carol());
    assert_eq!(nft_mint_2, "1:2");

    assert_eq!(
        contract
            .token_metadata_by_id
            .get(&String::from("1:1"))
            .unwrap(),
        String::from("1")
    );
    assert_eq!(
        contract
            .token_metadata_by_id
            .get(&String::from("1:2"))
            .unwrap(),
        String::from("1")
    );

    contract
}

#[test]
fn test_copies_and_buys() {
    test_copies_and_buys_internal();
}

#[test]
fn test_nft_tokens_total() {
    let contract = test_copies_and_buys_internal();
    assert_eq!(contract.nft_total_supply(), 2.into());
}

#[test]
fn test_nft_tokens_enumeration() {
    let contract = test_copies_and_buys_internal();
    let enumeration = contract.nft_tokens(None, None);
    let enumeration_unwrap = enumeration.get(0).unwrap();
    assert_eq!(enumeration_unwrap.token_id, String::from("1:1"));
    assert_eq!(
        enumeration_unwrap.metadata.title.to_owned().unwrap(),
        String::from("CampgroundTest #1")
    );
    assert_eq!(enumeration_unwrap.owner_id, bob());
    assert_eq!(enumeration_unwrap.series.creator_id, alice());
}
