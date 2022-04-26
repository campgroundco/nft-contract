use crate::*;

pub trait SeriesBridge {
    fn series_exists(&self, series_id: &TrailId) -> bool;
    fn get_owner(&self) -> &AccountId;
    fn get_trail_by_id(&self, series_id: &TrailId) -> TrailSeries;
    fn get_trail_by_id_optional(&self, series_id: &TrailId) -> Option<TrailSeries>;
    fn is_owner(&self, series_id: &TrailId, owner_id: &AccountId) -> bool;
    fn is_creator(&self, series_id: &TrailId, owner_id: &AccountId) -> bool;
    fn get_trail_business(&self, trail_and_copy_id: &TrailIdAndCopyNumber)
        -> Option<TrailBusiness>;
    fn get_all_trails_by_owner(&self, owner_id: &AccountId) -> Vec<TrailSeries>;
    fn get_all_trails_by_creator(&self, creator_id: &AccountId) -> Vec<TrailSeries>;
    fn get_current_fee(&self) -> u128;
    fn get_fee_percentage(&self) -> u64;
    fn get_treasury_address(&self) -> AccountId;
    fn is_caller_contract_owner(&self) -> bool;
}

#[near_bindgen]
impl SeriesBridge for Contract {
    fn series_exists(&self, series_id: &TrailId) -> bool {
        self.trails_metadata_by_id.get(series_id).is_some()
    }

    fn get_owner(&self) -> &AccountId {
        &self.owner_id
    }

    fn get_trail_by_id_optional(&self, series_id: &TrailId) -> Option<TrailSeries> {
        let token_series = self.trails_metadata_by_id.get(series_id);

        token_series
    }

    fn get_trail_by_id(&self, series_id: &TrailId) -> TrailSeries {
        let token_series = self
            .trails_metadata_by_id
            .get(series_id)
            .expect("Campground: Trail does not exist");

        token_series
    }

    fn is_owner(&self, series_id: &TrailIdAndCopyNumber, owner_id: &AccountId) -> bool {
        let maybe_trails_owner = self.tokens_per_owner.get(owner_id);
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

    fn is_creator(&self, series_id: &TrailId, owner_id: &AccountId) -> bool {
        if let Some(trail) = self.get_trail_by_id_optional(series_id) {
            &trail.creator_id == owner_id
        } else {
            false
        }
    }

    fn get_trail_business(
        &self,
        trail_and_copy_id: &TrailIdAndCopyNumber,
    ) -> Option<TrailBusiness> {
        self.tokens_by_id.get(trail_and_copy_id)
    }

    fn get_all_trails_by_owner(&self, owner_id: &AccountId) -> Vec<TrailSeries> {
        let maybe_trails_owner = self.tokens_per_owner.get(owner_id);

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

            ids.iter()
                .map(|id| self.get_trail_by_id(id))
                .collect::<Vec<TrailSeries>>()
        } else {
            vec![]
        }
    }

    fn get_all_trails_by_creator(&self, creator_id: &AccountId) -> Vec<TrailSeries> {
        let maybe_trails_owner = self.trails_series_by_creator.get(creator_id);

        if let Some(trails_owner) = maybe_trails_owner {
            let mut ids: Vec<String> = vec![];
            trails_owner.iter().for_each(|trail_id| {
                if !(ids.contains(&trail_id)) {
                    ids.push(trail_id)
                }
            });

            ids.iter()
                .map(|id| self.get_trail_by_id(id))
                .collect::<Vec<TrailSeries>>()
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

    fn is_caller_contract_owner(&self) -> bool {
        let caller = env::predecessor_account_id();
        self.owner_id.eq(&caller)
    }
}
