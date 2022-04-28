use crate::bridge::SeriesBridge;
use crate::*;
use near_sdk::CryptoHash;
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
pub(crate) fn refund_deposit(storage_used: u64, extra_spend: Balance) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit() - extra_spend;

    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    let refund = attached_deposit - required_cost;
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

pub(crate) fn partial_metadata_from_trail_series(trail_series: &TrailSeries) -> TokenMetadata {
    TokenMetadata {
        title: Some(trail_series.metadata.title.to_owned()),
        description: Some(trail_series.metadata.description.to_owned()),
        media: trail_series.metadata.media.to_owned().map(|v| v.to_owned()).or(Some(String::from("https://assets.website-files.com/6183638541c07be9ecbe4559/6221296e8a95113bac90ffdd_transparent%20background.png"))),
        media_hash: None,
        copies: None,
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None
    }
}

impl Contract {
    //add a token to the set of tokens an owner has
    pub(crate) fn internal_add_trail_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TrailIdAndCopyNumber,
    ) {
        //get the set of tokens for the given account
        let mut tokens_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| {
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
        self.tokens_per_owner.insert(account_id, &tokens_set);
    }

    pub(crate) fn internal_add_trail_to_creator(
        &mut self,
        account_id: &AccountId,
        trail_id: &TrailId,
    ) {
        //get the set of tokens for the given account
        let mut trails_set = self
            .trails_series_by_creator
            .get(account_id)
            .unwrap_or_else(|| {
                //if the account doesn't have any tokens, we create a new unordered set
                UnorderedSet::new(StorageKey::TokenPerCreator.try_to_vec().unwrap())
            });

        //we insert the token ID into the set
        trails_set.insert(trail_id);

        //we insert that set for the given account ID.
        self.trails_series_by_creator
            .insert(account_id, &trails_set);
    }

    //remove a token from an owner (internal method and can't be called directly via CLI).
    pub(crate) fn internal_remove_trail_from_owner(
        &mut self,
        account_id: &AccountId,
        trail_id: &TrailIdAndCopyNumber,
    ) {
        //we get the set of tokens that the owner has
        let mut tokens_set = self
            .tokens_per_owner
            .get(account_id)
            //if there is no set of tokens for the owner, we panic with the following message:
            .expect("Account does not own any tokens");

        //we remove the the token_id from the set of tokens
        tokens_set.remove(trail_id);

        //if the token set is now empty, we remove the owner from the tokens_per_owner collection
        if tokens_set.is_empty() {
            self.tokens_per_owner.remove(account_id);
        } else {
            //if the token set is not empty, we simply insert it back for the account ID.
            self.tokens_per_owner.insert(account_id, &tokens_set);
        }
    }

    pub(crate) fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        trail_id: &TrailIdAndCopyNumber,
        //we introduce an approval ID so that people with that approval ID can transfer the token
        approval_id: Option<u64>,
        memo: Option<String>,
    ) -> TrailBusiness {
        let trail = self.tokens_by_id.get(trail_id).expect("Trail does not exist");

        assert_eq!(sender_id, &trail.owner_id, "Only owner can transfer trail");
        assert_ne!(receiver_id, &trail.owner_id, "The trail owner and receiver must be different");

        self.internal_remove_trail_from_owner(&trail.owner_id, trail_id);
        self.internal_add_trail_to_owner(receiver_id, trail_id);

        let mut new_trail_business = trail.clone();
        new_trail_business.owner_id = receiver_id.clone();

        self.tokens_by_id.insert(trail_id, &new_trail_business);

        new_trail_business
    }

    pub(crate) fn panic_if_not_owner(&self) {
        if !self.is_caller_contract_owner() {
            panic!("Campground: Only contract owner can execute")
        }
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
