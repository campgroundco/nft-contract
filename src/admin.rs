use crate::*;

/// Provides admin operations to change different configurations of
/// the contract.
///
/// The contract `owner` is the only account allowed to perform these operations.
/// Otherwise, these operations panic.
pub trait AdminBridge {
    /// Changes Campground percentage `fee`.
    /// When a creator creates a trail series,
    /// the series takes this `fee` as a default `fee`.
    fn change_campground_fee(&mut self, fee: u64);

    /// Changes treasury address to a new one.
    /// The treasury address receives the applied `fee` after an NFT
    /// has been bought.
    fn change_campground_treasury_address(&mut self, addr: AccountId);

    /// Changes campground minimum `fee`, in yoctoNEAR.
    fn change_campground_minimum_fee(&mut self, fee: Balance);

    /// Adds a setting key-val to the map
    fn add_setting(&mut self, key: String, value: String);
}

#[near_bindgen]
impl AdminBridge for Contract {
    fn change_campground_fee(&mut self, fee: u64) {
        self.panic_if_not_owner();
        self.campground_fee = fee;
    }

    fn change_campground_treasury_address(&mut self, addr: AccountId) {
        self.panic_if_not_owner();
        self.campground_treasury_address = addr;
    }

    fn change_campground_minimum_fee(&mut self, fee: Balance) {
        self.panic_if_not_owner();
        self.campground_minimum_fee_yocto_near = fee
    }

    fn add_setting(&mut self, key: String, value: String) {
        self.panic_if_not_owner();
        self.settings.insert(&key, &value);
    }
}
