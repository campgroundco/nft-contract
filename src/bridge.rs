use crate::*;

pub trait SeriesBridge {
    fn series_exists(&self, series_id: &TrailId) -> bool;
}

#[near_bindgen]
impl SeriesBridge for Contract {
    fn series_exists(&self, series_id: &TrailId) -> bool {
        self.trails_series_by_id.get(series_id).is_some()
    }
}
