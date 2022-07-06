use crate::*;
use crate::bridge::SeriesBridge;
use crate::vars::SUB_ADMIN_ADDRESS;

/// This trait complies with functions that can only be accessed by settings.SUB_ADMIN_ADDRESS
pub trait SubAdminBridge {
    /// Remove a trail from nonmintable_trails so that it can be minted by users again
    fn remove_trail_from_nonmintable_list(&mut self, trail_id: TrailId) -> bool;

    /// Verifies whether caller is subadmin
    fn is_caller_subadmin(&self) -> bool;

    /// Gets accountId of sub admin
    fn get_subadmin(&self) -> AccountId;

    /// Includes a trail in the list of non-user mintable
    fn insert_trail_from_nonmintable_list(&mut self, trail_id: TrailId) -> bool;

    /// Panic if caller is not able to toggle
    fn verify_toggle_minting_caller(&self, trail_id: &TrailId);

    /// Toggle minting for all trails
    /// enable_minting must be true to make all trails accept public minting (nft_buy_series)
    /// Or false to lock them all
    fn toggle_for_all(&mut self, enable_minting: bool);
}

#[near_bindgen]
impl SubAdminBridge for Contract {
    fn remove_trail_from_nonmintable_list(&mut self, trail_id: TrailId) -> bool {
        self.verify_toggle_minting_caller(&trail_id);
        self.nonmintable_trails.remove(&trail_id)
    }

    fn insert_trail_from_nonmintable_list(&mut self, trail_id: TrailId) -> bool {
        self.verify_toggle_minting_caller(&trail_id);
        self.nonmintable_trails.insert(&trail_id)
    }

    fn verify_toggle_minting_caller(&self, trail_id: &TrailId) {
        let trail = self.get_trail_by_id_optional(trail_id).expect("Trail does not exist");
        let is_sender_owner = trail.creator_id == env::predecessor_account_id();
        if !is_sender_owner {
            self.panic_if_not_subadmin();
        }
    }

    fn toggle_for_all(&mut self, enable_minting: bool) {
        self.panic_if_not_subadmin();
        let keys: Vec<TrailId> = self.trails_metadata_by_id.keys().map(|trail_id| trail_id).collect();
        for trail_id in keys {
            if enable_minting {
                self.remove_trail_from_nonmintable_list(trail_id);
            } else {
                self.insert_trail_from_nonmintable_list(trail_id);
            }
        }
    }

    fn is_caller_subadmin(&self) -> bool {
        let subadmin = self.get_subadmin();
        let predecessor = env::predecessor_account_id();
        subadmin == predecessor
    }

    fn get_subadmin(&self) -> AccountId {
        let key = String::from(SUB_ADMIN_ADDRESS);
        let value = self.settings.get(&key).unwrap_or_else(|| String::new());
        let acct_id: AccountId = AccountId::try_from(value).unwrap();
        acct_id
    }
}