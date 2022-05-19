pub mod context;

use ito_contract::{
    bridge::SeriesBridge, create_serie::CreateTrailSeries, Contract, TrailResource,
    TrailSeriesMetadata, ONE_NEAR,
};
use near_sdk::{env, json_types::U128, testing_env};

use context::{alice, bob, carol, create_series, setup_contract, STORAGE_FOR_CREATE_SERIES};

#[test]
fn contract_should_allow_account_to_create_trail_series() {
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
        Some((1 * 10u128.pow(24)).into()),
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
fn contract_should_reject_creating_trail_series_with_invalid_price() {
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
fn contract_should_reject_creating_trail_series_with_invalid_ticket_amount() {
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
fn contract_should_reject_creating_trail_series_with_invalid_resources_amount() {
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
fn contract_should_reject_minting_when_tickets_are_sold_out() {
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
    let trail_by_id = contract.get_trail_by_id(&"1".into());
    assert_eq!(trail_by_id.is_mintable, true);
    contract.nft_mint("1".into(), bob());

    let track_by_owner = contract.trail_tickets_for_owner(bob(), None, None);
    assert_eq!(track_by_owner.len(), 1);
    println!("{}", track_by_owner.get(0).unwrap().series.is_mintable);

    // Panics
    contract.nft_mint("1".into(), bob());
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
        Some(1000.into()),
        Some(10),
        None,
    );
    let nft_mint_1 = contract.nft_mint("1".into(), bob());
    assert_eq!(nft_mint_1, "1:1");

    testing_env!(context
        .predecessor_account_id(bob())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES + ONE_NEAR / 10)
        .build());

    // Panics
    let nft_mint_2 = contract.nft_buy_series("1".into(), carol());
    assert_eq!(nft_mint_2, "1:2");

    assert_eq!(
        contract.token_metadata_by_id.get(&"1:1".into()).unwrap(),
        "1"
    );
    assert_eq!(
        contract.token_metadata_by_id.get(&"1:2".into()).unwrap(),
        "1"
    );

    contract
}

#[test]
fn contract_should_allow_account_to_buy_and_mint() {
    test_copies_and_buys_internal();
}

#[test]
fn contract_should_return_nft_tokens_total_after_buying() {
    let contract = test_copies_and_buys_internal();
    assert_eq!(contract.nft_total_supply(), 2.into());
}

#[test]
fn contract_should_return_nft_tokens_enumeration_after_buying() {
    let contract = test_copies_and_buys_internal();
    let enumeration = contract.nft_tokens(None, None);
    let enumeration_unwrap = enumeration.get(0).unwrap();
    assert_eq!(enumeration_unwrap.token_id, "1:1");
    assert_eq!(
        enumeration_unwrap.metadata.title.to_owned().unwrap(),
        "CampgroundTest #1"
    );
    assert_eq!(enumeration_unwrap.owner_id, bob());
    assert_eq!(enumeration_unwrap.series.creator_id, alice());
}

#[test]
fn estimate_create_series_storage_usage() {
    fn measure_create_series(contract: &mut Contract, usage_estimate: u64) {
        let usage = env::storage_usage();
        contract.create_trail_series(
            TrailSeriesMetadata {
                title: "My Trail".to_owned(),
                description: "Some description".to_owned(),
                tickets_amount: 10,
                media: None,
                data: None,
                resources: vec![TrailResource {
                    title: None,
                    description: None,
                    media: "http://arweave.net/image.png".to_owned(),
                    extra: None,
                    reference: None,
                }],
                starts_at: None,
                expires_at: None,
                reference: None,
                campground_id: "123".to_owned(),
            },
            Some(10000000000000000000000000.into()),
            None,
            None,
        );

        let usage = env::storage_usage() - usage;

        println!("{}", usage);
        assert_eq!(usage, usage_estimate);
    }

    let (mut context, mut contract) = setup_contract();

    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(STORAGE_FOR_CREATE_SERIES)
        .build());

    measure_create_series(&mut contract, 499);

    for i in 2..200 {
        let token_id_len_extra = (i.to_string().len() - 1) * 4;
        measure_create_series(&mut contract, 429 + token_id_len_extra as u64);
    }
}
