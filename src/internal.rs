use crate::*;
use near_sdk::{CryptoHash};
use std::mem::size_of;

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

//refund the initial deposit based on the amount of storage that was used up
pub(crate) fn refund_deposit(storage_used: u64) {
    //get how much it would cost to store the information
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    //get the attached deposit
    let attached_deposit = env::attached_deposit();

    //make sure that the attached deposit is greater than or equal to the required cost
    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    //get the refund amount from the attached deposit - required cost
    let refund = attached_deposit - required_cost;

    //if the refund is greater than 1 yocto NEAR, we refund the predecessor that amount
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

pub(crate) fn calculate_fee(price: u128, campground_fee: u64, min_treasury: u128) -> u128 {
    let fee = (price as u128 * campground_fee as u128) / 100;
    if fee < min_treasury {
        min_treasury
    } else {
        fee
    }
}

pub(crate) fn calculate_yocto_near(nears: f64) -> Balance {
    (nears * (ONE_NEAR as f64)) as u128
}

impl Contract {
    //add a token to the set of tokens an owner has
    pub(crate) fn internal_add_trail_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TrailId,
    ) {
        //get the set of tokens for the given account
        let mut tokens_set = self.trails_per_owner.get(account_id).unwrap_or_else(|| {
            //if the account doesn't have any tokens, we create a new unordered set
            UnorderedSet::new(
                StorageKey::TokenPerOwnerInner {
                    //we get a new unique prefix for the collection
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        //we insert the token ID into the set
        tokens_set.insert(token_id);

        //we insert that set for the given account ID. 
        self.trails_per_owner.insert(account_id, &tokens_set);
    }

    pub(crate) fn internal_add_trail_to_creator(
        &mut self,
        account_id: &AccountId,
        trail_id: &TrailId,
    ) {
        //get the set of tokens for the given account
        let mut trails_set = self.trails_series_by_creator.get(account_id).unwrap_or_else(|| {
            //if the account doesn't have any tokens, we create a new unordered set
            UnorderedSet::new(
                StorageKey::TokenPerCreator.try_to_vec().unwrap(),
            )
        });

        //we insert the token ID into the set
        trails_set.insert(trail_id);

        //we insert that set for the given account ID.
        self.trails_series_by_creator.insert(account_id, &trails_set);
    }
}

#[cfg(test)]
mod tests {
    use crate::internal::{calculate_fee, calculate_yocto_near};
    use crate::ONE_NEAR;

    #[test]
    fn calculate_fee_test() {
        assert_eq!(calculate_fee(100, 5, 2), 5);
        assert_eq!(calculate_fee(10, 1, 2), 2);
        assert_eq!(calculate_fee(0, 0, 2), 2);
    }

    #[test]
    fn calculate_yoctonear_test() {
        assert!(calculate_yocto_near(0.1) < ONE_NEAR);
    }
}
