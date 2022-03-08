use near_sdk::{AccountId, env};
use crate::{TrailSeriesMetadata, Contract};
use crate::bridge::SeriesBridge;

pub trait CreateTrailSeries {
    fn create_trail_series(&mut self, metadata: TrailSeriesMetadata);
}

#[near_bindgen]
impl CreateTrailSeries for Contract {

    #[payable]
    fn create_trail_series(&mut self, metadata: TrailSeriesMetadata) {
        let initial_storage_usage = env::storage_usage();
        let creator_id = env::predecessor_account_id();
        let token_series_id = format!("{}", (self.trails_series_by_id.len() + 1));

        assert!(
            !(self.series_exists(&token_series_id)),
            "Campground: Duplicate series id"
        );



    }

}
