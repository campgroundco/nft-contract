#![deny(warnings)]
#![warn(missing_docs)]

pub mod context;

use context::setup_contract;
use ito_contract::{bridge::SeriesBridge, nft_core::NonFungibleTokenCore, BUY_STORAGE, ONE_NEAR};
use near_sdk::{test_utils::accounts, testing_env};

use context::create_series;

#[test]
fn it_should_get_nft_token_info() {
    let (mut context, mut contract) = setup_contract();

    testing_env!(context
        .predecessor_account_id(accounts(1))
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
        .predecessor_account_id(accounts(1))
        .attached_deposit(ONE_NEAR + BUY_STORAGE)
        .build());

    let token_id = contract.nft_buy_series(trail.token_id.clone(), accounts(3));
    let token = contract.nft_token(token_id.clone()).unwrap();

    assert_eq!(token.token_id, token_id);
    assert_eq!(token.owner_id, accounts(3));
}

#[test]
fn it_should_transfer_nft_to_receiver() {
    let (mut context, mut contract) = setup_contract();
    let a = accounts(1);

    testing_env!(context
        .predecessor_account_id(accounts(1))
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
        .predecessor_account_id(accounts(1))
        .attached_deposit(ONE_NEAR + BUY_STORAGE)
        .build());

    let token_id = contract.nft_buy_series(trail.token_id.clone(), accounts(3));

    testing_env!(context
        .predecessor_account_id(accounts(3))
        .attached_deposit(ONE_NEAR + BUY_STORAGE)
        .build());
    contract.nft_transfer(a.clone(), token_id, None);
    assert!(contract.is_owner(&trail.token_id, &a));
}
