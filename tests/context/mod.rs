#![deny(warnings)]
#![warn(missing_docs)]

use ito_contract::{
    create_serie::CreateTrailSeries, Contract, JsonTrail, TrailResource, TrailSeriesMetadata,
};
use near_sdk::{
    json_types::U128,
    test_utils::{accounts, VMContextBuilder},
    testing_env, AccountId, Balance,
};

pub const STORAGE_FOR_CREATE_SERIES: Balance = 6960000000000000000000;

pub fn owner() -> AccountId {
    accounts(0)
}

pub fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);
    builder
}

pub fn setup_contract() -> (VMContextBuilder, Contract) {
    let mut context = VMContextBuilder::new();
    testing_env!(context.predecessor_account_id(accounts(0)).build());
    let contract = Contract::new_default_meta(accounts(0), accounts(4));
    (context, contract)
}

pub fn create_series(
    contract: &mut Contract,
    title: &str,
    starts_at: Option<u64>,
    expires_at: Option<u64>,
    price: Option<U128>,
    tickets: Option<u64>,
    resources: Option<Vec<TrailResource>>,
) -> JsonTrail {
    let trail = contract.create_trail_series(
        TrailSeriesMetadata {
            title: String::from(title),
            description: String::new(),
            tickets_amount: tickets.unwrap_or(100),
            media: None,
            data: None,
            resources: resources.unwrap_or(vec![TrailResource {
                title: Some(format!("{}-{}", title, "resource")),
                description: None,
                media: format!("{}.png", title),
                extra: None,
                reference: None,
            }]),
            starts_at,
            expires_at,
            reference: None,
            campground_id: String::from("CMPGR123"),
        },
        price,
    );

    trail
}
