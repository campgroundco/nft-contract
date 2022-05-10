pub mod context;

use context::setup_contract;
use ito_contract::nft_core::NonFungibleTokenCore;
use near_sdk::test_utils::accounts;

#[test]
#[ignore = "reason"]
fn it_should_change_campground_minimum_fee() {
    let (mut _context, mut contract) = setup_contract();
    let a = accounts(1);
    contract.nft_transfer(a, "".to_string(), None);
}
