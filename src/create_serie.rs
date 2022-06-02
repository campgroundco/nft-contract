use crate::bridge::SeriesBridge;
use crate::*;
use std::convert::TryFrom;

/// Provides operations to deal with trail series.
pub trait CreateTrailSeries {
    /// Creates a series (trail) inside the smart contract.
    fn create_trail_series(
        &mut self,
        metadata: TrailSeriesMetadata,
        price: Option<U128>,
        creator: Option<AccountId>,
        creator_royalty: Option<U128>,
    ) -> JsonTrail;

    fn create_trail_series_estimated(
        &self,
        metadata: TrailSeriesMetadata,
        price: Option<U128>,
        creator: Option<AccountId>,
        creator_royalty: Option<U128>,
    ) -> Option<U128>;
}

#[near_bindgen]
impl CreateTrailSeries for Contract {
    #[allow(unused_variables)]
    fn create_trail_series_estimated(
        &self,
        metadata: TrailSeriesMetadata,
        price: Option<U128>,
        creator_id: Option<AccountId>,
        creator_royalty: Option<U128>,
    ) -> Option<U128> {
        let input_bytes = env::input().unwrap_or(vec![]).len();
        let high_approximate = input_bytes + 500;
        let usize_to_u128 = u128::try_from(high_approximate);
        if usize_to_u128.is_ok() {
            Some(U128(
                (usize_to_u128.unwrap() * env::storage_byte_cost()).into(),
            ))
        } else {
            None
        }
    }

    #[payable]
    fn create_trail_series(
        &mut self,
        metadata: TrailSeriesMetadata,
        price: Option<U128>,
        creator_id: Option<AccountId>,
        creator_royalty: Option<U128>,
    ) -> JsonTrail {
        let initial_storage_usage = env::storage_usage();
        let creator_id = creator_id.unwrap_or(env::predecessor_account_id());
        let current_block_timestamp = env::block_timestamp();
        let token_series_id = format!("{}", (self.trails_metadata_by_id.len() + 1));

        assert!(
            !(self.series_exists(&token_series_id)),
            "Campground: Duplicate series id"
        );

        let maybe_price = price.clone();

        let price_res: Option<u128> = if maybe_price.is_some() {
            let indicated_price = maybe_price.unwrap().0;
            assert!(
                indicated_price < MAX_PRICE,
                "Campground: price higher than {}",
                MAX_PRICE
            );
            Some(indicated_price)
        } else {
            None
        };

        let quantity = metadata.tickets_amount;
        assert!(
            quantity > 0,
            "Campground: At least 1 ticket is required per trail series"
        );

        let resources_len = metadata.resources.len();
        assert!(
            resources_len > 0,
            "Campground: At least 1 resource is needed per trail"
        );

        // let can_be_traded_at = metadata.starts_at.unwrap_or(current_block_timestamp.clone());
        // let valid_until = metadata.expires_at.unwrap_or_else(|| u64::MAX);
        // assert!(valid_until > can_be_traded_at, "Campground: Trail tickets need to be valid in a greater date than the start date");

        let price = price_res.unwrap_or(0);
        let campground_fee_near = U128(calculate_fee(
            price,
            self.campground_fee,
            self.campground_minimum_fee_yocto_near,
        ));

        let trail_series = TrailSeries {
            is_mintable: true,
            creator_id: creator_id.clone(),
            issue_at: current_block_timestamp,
            metadata,
            supply: SeriesSupply {
                total: quantity,
                circulating: 0 as u64,
            },
            price: price.into(),
            campground_fee_near,
            creator_royalty_near: creator_royalty,
            royalties: HashMap::new(),
        };

        self.trails_metadata_by_id
            .insert(&token_series_id, &trail_series);
        self.internal_add_trail_to_creator(&creator_id, &token_series_id);

        refund_deposit(env::storage_usage() - initial_storage_usage, 0);

        format_json_trail(
            token_series_id,
            creator_id,
            trail_series.clone(),
            partial_metadata_from_trail_series(&trail_series),
            false,
        )
    }
}
