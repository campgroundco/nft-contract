#[cfg(all(test, not(target_arch = "wasm32")))]
mod test {
    use near_sdk_sim::{init_simulator, ContractAccount, deploy};
    use crate::{Contract as CampgroundContract, Contract, NFTContractMetadata, TrailSeriesMetadata, TrailResource, JsonTrail, ONE_NEAR};
    use near_sdk::{VMContext, AccountId};
    use near_sdk::test_utils::{VMContextBuilder, accounts};
    use std::convert::TryInto;
    use near_sdk::json_types::{ValidAccountId, U128};
    use crate::bridge::SeriesBridge;
    use near_sdk::{testing_env};
    use crate::create_serie::CreateTrailSeries;
    use near_sdk_sim::types::Balance;
    use crate::internal::calculate_yocto_near;

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

    fn create_series(contract: &mut Contract, title: &str, starts_at: Option<u64>, expires_at: Option<u64>, price: Option<U128>, tickets: Option<u64>, resources: Option<Vec<TrailResource>>) -> JsonTrail {
        let trail = contract.create_trail_series(TrailSeriesMetadata {
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
                reference: None
            }]),
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

        let trail_series = create_series(&mut contract, "CampgroundTest", Some(1647109675), Some(1647216000), Some(U128::from(1 * 10u128.pow(24))), None, None);
        let trail_by_id = contract.get_trail_by_id(&String::from("1"));
        assert_eq!(trail_series.series.creator_id, trail_by_id.creator_id);
        assert_eq!(trail_series.series.price, trail_by_id.price);
        assert_eq!(trail_series.series.metadata.expires_at, trail_by_id.metadata.expires_at);
        assert_eq!(trail_series.series.metadata.starts_at, trail_by_id.metadata.starts_at);
        assert_eq!(trail_by_id.metadata.resources.get(0).unwrap().media, "CampgroundTest.png");
    }

    #[test]
    #[should_panic(expected = "Campground: price higher than 1000000000000000000000000000000000")]
    fn create_trail_series_invalid_price() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build()
        );

        let trail_series = create_series(&mut contract, "CampgroundTest", Some(1647109675), Some(1647216000), Some(U128::from(1_000_000_001 * 10u128.pow(24))), None, None);
    }

    #[test]
    #[should_panic(expected = "Campground: At least 1 ticket is required per trail series")]
    fn create_trail_series_invalid_ticket_amount() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build()
        );

        let trail_series = create_series(&mut contract, "CampgroundTest", Some(1647109675), Some(1647216000), Some(U128::from(1 * 10u128.pow(24))), Some(0 as u64), None);
    }

    #[test]
    #[should_panic(expected = "Campground: At least 1 resource is needed per trail")]
    fn create_trail_series_invalid_resources_amount() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build()
        );

        let trail_series = create_series(&mut contract, "CampgroundTest", Some(1647109675), Some(1647216000), Some(U128::from(1 * 10u128.pow(24))), Some(1 as u64), Some(vec![]));
    }

    #[test]
    #[should_panic(expected = "Campground: Trail is not mintable")]
    fn test_minting() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build()
        );

        let trail_series = create_series(&mut contract, "CampgroundTest", Some(1647109675), Some(1647216000), Some(U128::from(1 * 10u128.pow(24))), Some(1), None);
        let trail_by_id = contract.get_trail_by_id(&String::from("1"));
        assert_eq!(trail_by_id.is_mintable, true);
        contract.nft_mint(String::from("1"), accounts(2));

        let track_by_owner = contract.trail_tickets_for_owner(accounts(2), None, None);
        assert_eq!(track_by_owner.len(), 1);
        println!("{}", track_by_owner.get(0).unwrap().series.is_mintable);

        // Panics
        contract.nft_mint(String::from("1"), accounts(2));
    }

    #[test]
    fn test_copies_and_buys() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build()
        );

        let trail_series = create_series(&mut contract, "CampgroundTest", Some(1647109675), Some(1647216000), Some(U128::from(1000 as u128)), Some(10), None);
        let nft_mint_1 = contract.nft_mint(String::from("1"), accounts(2));
        assert_eq!(nft_mint_1, "1:1");

        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build()
        );

        // Panics
        let nft_mint_2 = contract.buy_series(String::from("1"), accounts(3));
        assert_eq!(nft_mint_2, "1:2");
    }

    #[test]
    #[should_panic(expected = "Campground: Attached deposit is less than price")]
    fn test_buy_invalid_amount() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build()
        );

        let trail_series = create_series(&mut contract, "CampgroundTest", Some(1647109675), Some(1647216000), Some(U128::from(1000 as u128)), Some(10), None);
        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(500 as u128)
            .build()
        );

        // Panics
        let nft_mint_2 = contract.buy_series(String::from("1"), accounts(3));
    }

    #[test]
    #[should_panic(expected = "Campground: Attached deposit is less than minimum buying fee")]
    fn test_buy_invalid_fee() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build()
        );

        let trail_series = create_series(&mut contract, "CampgroundTest", Some(1647109675), Some(1647216000), Some(U128::from(calculate_yocto_near(0.01))), Some(10), None);
        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(contract.campground_minimum_fee_yocto_near - 1)
            .build()
        );

        contract.buy_series(String::from("1"), accounts(3));
    }

    #[test]
    fn test_buy_just_enough_fee() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build()
        );

        let trail_series = create_series(&mut contract, "CampgroundTest", Some(1647109675), Some(1647216000), Some(U128::from(calculate_yocto_near(0.01))), Some(10), None);
        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(contract.campground_minimum_fee_yocto_near)
            .build()
        );

        contract.buy_series(String::from("1"), accounts(3));
    }


    #[test]
    fn test_buy_one_near() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(STORAGE_FOR_CREATE_SERIES)
            .build()
        );

        let trail_series = create_series(&mut contract, "CampgroundTest", Some(1647109675), Some(1647216000), Some(U128::from(ONE_NEAR)), Some(10), None);
        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(ONE_NEAR)
            .build()
        );

        contract.buy_series(String::from("1"), accounts(3));

        testing_env!(context
            .predecessor_account_id(accounts(3))
            .attached_deposit(ONE_NEAR)
            .build()
        );

        contract.buy_series(String::from("1"), accounts(3));

        let get_account_trails = contract.trails_per_owner.get(&accounts(3)).unwrap();
        let trails_as_vec = get_account_trails.to_vec();
        assert_eq!(get_account_trails.len(), 2);
        assert_eq!(trails_as_vec.get(0).unwrap(), &String::from("1:1"));
        assert_eq!(trails_as_vec.get(1).unwrap(), &String::from("1:2"));

        let trails_by_id = contract.trails_by_id.get(&String::from("1:1")).unwrap();
        assert_eq!(trails_by_id.owner_id, accounts(3));
        assert_eq!(trails_by_id.trail_id_reference, "1");
    }

}
