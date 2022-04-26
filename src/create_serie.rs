use crate::bridge::SeriesBridge;
use crate::*;

pub trait CreateTrailSeries {
    fn create_trail_series(
        &mut self,
        metadata: TrailSeriesMetadata,
        price: Option<U128>,
    ) -> JsonTrail;
}

#[near_bindgen]
impl CreateTrailSeries for Contract {
    #[payable]
    fn create_trail_series(
        &mut self,
        metadata: TrailSeriesMetadata,
        price: Option<U128>,
    ) -> JsonTrail {
        let initial_storage_usage = env::storage_usage();
        let creator_id = env::predecessor_account_id();
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

        let trail_series = TrailSeries {
            is_mintable: true,
            creator_id: creator_id.clone(),
            issue_at: current_block_timestamp,
            metadata,
            supply: SeriesSupply {
                total: quantity,
                circulating: 0 as u64,
            },
            price: price_res.unwrap_or(0),
        };

        self.trails_metadata_by_id
            .insert(&token_series_id, &trail_series);
        self.internal_add_trail_to_creator(&creator_id, &token_series_id);

        refund_deposit(env::storage_usage() - initial_storage_usage, 0);

        JsonTrail {
            token_id: token_series_id,
            owner_id: creator_id,
            metadata: partial_metadata_from_trail_series(&trail_series),
            series: trail_series,
        }
    }
}
