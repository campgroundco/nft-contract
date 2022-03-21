use crate::*;

pub trait SeriesBridge {
    fn series_exists(&self, series_id: &TrailId) -> bool;
    fn get_owner(&self) -> &AccountId;
    fn get_trail_by_id(&self, series_id: &TrailId) -> TrailSeries;
    fn is_owner(&self, series_id: &TrailId, owner_id: &AccountId) -> bool;
    fn get_trail_business(&self, trail_and_copy_id: &TrailIdAndCopyNumber) -> Option<TrailBusiness>;
    fn get_all_trails_by_owner(&self, owner_id: &AccountId) -> Vec<TrailSeries>;
    fn get_current_fee(&self) -> u128;
    fn get_fee_percentage(&self) -> u64;
    fn get_treasury_address(&self) -> AccountId;
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

    fn is_owner(&self, series_id: &TrailId, owner_id: &AccountId) -> bool {
        let maybe_trails_owner = self.trails_per_owner.get(owner_id);
        if let Some(trails_owner) = maybe_trails_owner {
            trails_owner.iter().any(|trail_id_and_copy_id| {
                let id_and_copy: Vec<&str> = trail_id_and_copy_id.split(TRAIL_DELIMETER).collect();
                let _series_id = id_and_copy.get(0).unwrap().clone();
                _series_id == series_id
            })
        } else {
            false
        }
    }

    fn get_trail_business(&self, trail_and_copy_id: &TrailIdAndCopyNumber) -> Option<TrailBusiness> {
        self.trails_by_id.get(trail_and_copy_id)
    }

    fn get_all_trails_by_owner(&self, owner_id: &AccountId) -> Vec<TrailSeries> {
        let maybe_trails_owner = self.trails_per_owner.get(owner_id);

        if let Some(trails_owner) = maybe_trails_owner {
            let mut ids: Vec<String> = vec![];
            trails_owner.iter().for_each(|trail_id_and_copy_id| {
                let id_and_copy: Vec<&str> = trail_id_and_copy_id.split(TRAIL_DELIMETER).collect();
                let _series_id = id_and_copy.get(0).unwrap().clone();
                let _series_id_str = _series_id.to_string();
                if !(ids.contains(&_series_id_str)) {
                    ids.push(_series_id_str)
                }
            });

            ids.iter().map(|id| {
                self.get_trail_by_id(id)
            }).collect::<Vec<TrailSeries>>()
        } else {
            vec![]
        }

    }

    fn get_current_fee(&self) -> u128 {
        self.campground_minimum_fee_yocto_near
    }

    fn get_fee_percentage(&self) -> u64 {
        self.campground_fee
    }

    fn get_treasury_address(&self) -> AccountId {
        self.campground_treasury_address.clone()
    }
}
