use crate::*;

pub trait SeriesBridge {
    fn series_exists(&self, series_id: &TrailId) -> bool;
    fn get_owner(&self) -> &AccountId;
    fn get_trail_by_id(&self, series_id: &TrailId) -> TrailSeries;
}

#[near_bindgen]
impl SeriesBridge for Contract {
    fn series_exists(&self, series_id: &TrailId) -> bool {
        self.trails_series_by_id.get(series_id).is_some()
    }

    fn get_owner(&self) -> &AccountId {
        &self.owner_id
    }

    fn get_trail_by_id(&self, series_id: &TrailId) -> TrailSeries {
        let token_series = self
            .trails_series_by_id
            .get(series_id)
            .expect("Campground: Trail does not exist");

        token_series
    }
}
