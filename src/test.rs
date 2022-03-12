#[cfg(all(test, not(target_arch = "wasm32")))]
mod test {
    use near_sdk_sim::{init_simulator, ContractAccount, deploy};
    use crate::{Contract as CampgroundContract, Contract, NFTContractMetadata, TrailSeriesMetadata, TrailResource, JsonTrail};
    use near_sdk::VMContext;
    use near_sdk::test_utils::{VMContextBuilder, accounts};
    use std::convert::TryInto;
    use near_sdk::json_types::{ValidAccountId, U128};
    use crate::bridge::SeriesBridge;
    use near_sdk::{testing_env};
    use crate::create_serie::CreateTrailSeries;
    use near_sdk_sim::types::Balance;

    const STORAGE_FOR_CREATE_SERIES: Balance = 8540000000000000000000;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn setup_contract() -> (VMContextBuilder, Contract) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let contract = Contract::new_default_meta(accounts(0), accounts(4));
        (context, contract)
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(0), accounts(4));
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.get_owner(), &accounts(0));
        assert_eq!(contract.campground_fee, 5 as u64);
    }

    fn create_series(contract: &mut Contract, title: &str, starts_at: Option<u64>, expires_at: Option<u64>, price: Option<U128>) -> JsonTrail {
        let trail = contract.create_trail_series(TrailSeriesMetadata {
            title: String::from(title),
            description: String::new(),
            tickets_amount: 100,
            media: None,
            data: None,
            resources: vec![TrailResource {
                title: Some(format!("{}-{}", title, "resource")),
                description: None,
                media: format!("{}.png", title),
                extra: None,
                reference: None
            }],
            starts_at,
            expires_at,
            reference: None,
            campground_id: String::from("CMPGR123")
        }, price);

        trail
    }

    #[test]
    fn create_trail_series() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build()
        );

        let trail_series = create_series(&mut contract, "CampgroundTest", Some(1647109675), Some(1647216000), Some(U128::from(1 * 10u128.pow(24))));
        let trail_by_id = contract.get_trail_by_id(&String::from("1"));
        assert_eq!(trail_series.series.creator_id, trail_by_id.creator_id);
        assert_eq!(trail_series.series.price, trail_by_id.price);
        assert_eq!(trail_series.series.metadata.expires_at, trail_by_id.metadata.expires_at);
        assert_eq!(trail_series.series.metadata.starts_at, trail_by_id.metadata.starts_at);
        assert_eq!(trail_by_id.metadata.resources.get(0).unwrap().media, "CampgroundTest.png");

    }

}
