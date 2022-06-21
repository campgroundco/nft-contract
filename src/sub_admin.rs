use crate::*;
use crate::vars::SUB_ADMIN_ADDRESS;

/// This trait complies with functions that can only be accessed by settings.SUB_ADMIN_ADDRESS
pub trait SubAdminBridge {
    /// Remove a trail from nonmintable_trails so that it can be minted by users again
    fn remove_trail_from_nonmintable_list(&mut self, trail_id: TrailId) -> bool;

    /// Verifies whether caller is subadmin
    fn is_caller_subadmin(&self) -> bool;

    /// Gets accountId of sub admin
    fn get_subadmin(&self) -> AccountId;
}

#[near_bindgen]
impl SubAdminBridge for Contract {
    fn remove_trail_from_nonmintable_list(&mut self, trail_id: TrailId) -> bool {
        self.panic_if_not_subadmin();
        self.nonmintable_trails.remove(&trail_id)
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