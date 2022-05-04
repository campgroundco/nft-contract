use crate::*;

/// Provides admin operations to change different configurations of
/// the contract.
/// 
/// The contract owner is the only account allowed to perform these operations.
pub trait AdminBridge {
    /// Changes Campground percentage `fee`.
    fn change_campground_fee(&mut self, fee: u64);

    /// Changes treasury address to a new one.
    fn change_campground_treasury_address(&mut self, addr: AccountId);

    /// Changes campground minimum `fee`, in yoctoNEAR.
    fn change_campground_minimum_fee(&mut self, fee: Balance);
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
}
