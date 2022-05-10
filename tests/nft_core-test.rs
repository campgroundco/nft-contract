#![deny(warnings)]
#![warn(missing_docs)]

pub mod context;

use context::setup_contract;
use ito_contract::{bridge::SeriesBridge, nft_core::NonFungibleTokenCore, BUY_STORAGE, ONE_NEAR};
use near_sdk::testing_env;

use context::{alice, carol, create_series};

#[test]
fn it_should_get_nft_token_info() {
    let (mut context, mut contract) = setup_contract();

    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(6920000000000000000000)
        .build());
    let trail = create_series(
        &mut contract,
        "CampgroundTest",
        Some(1647109675),
        Some(1647216000),
        Some(ONE_NEAR.into()),
        Some(10),
        None,
    );

    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(ONE_NEAR + BUY_STORAGE)
        .build());

    let token_id = contract.nft_buy_series(trail.token_id.clone(), carol());
    let token = contract.nft_token(token_id.clone()).unwrap();

    assert_eq!(token.token_id, token_id);
    assert_eq!(token.owner_id, carol());
}

#[test]
fn it_should_transfer_nft_to_receiver() {
    let (mut context, mut contract) = setup_contract();

    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(6920000000000000000000)
        .build());
    let trail = create_series(
        &mut contract,
        "CampgroundTest",
        Some(1647109675),
        Some(1647216000),
        Some(ONE_NEAR.into()),
        Some(10),
        None,
    );

    testing_env!(context
        .predecessor_account_id(alice())
        .attached_deposit(ONE_NEAR + BUY_STORAGE)
        .build());

    let token_id = contract.nft_buy_series(trail.token_id.clone(), carol());

    testing_env!(context
        .predecessor_account_id(carol())
        .attached_deposit(ONE_NEAR + BUY_STORAGE)
        .build());
    contract.nft_transfer(alice(), token_id, None);
    assert!(contract.is_owner(&trail.token_id, &alice()));
}
