#![deny(warnings)]

pub mod context;

use ito_contract::{bridge::SeriesBridge, Contract};
use near_sdk::testing_env;

use context::{alice, get_context, owner, treasury};

#[test]
fn contract_should_be_initialized_with_default_values() {
    let mut context = get_context(alice());
    testing_env!(context.build());
    let contract = Contract::new_default_meta(owner(), treasury());
    testing_env!(context.is_view(true).build());
    assert_eq!(contract.get_owner(), &owner());
    assert_eq!(contract.token_metadata_by_id.len(), 0);
    assert_eq!(contract.trails_metadata_by_id.len(), 0);
    assert_eq!(contract.campground_fee, 5);
    assert_eq!(contract.campground_treasury_address, treasury());
    assert_eq!(contract.campground_minimum_fee_yocto_near, 10u128.pow(23));
}
