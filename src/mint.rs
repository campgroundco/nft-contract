use crate::*;

#[near_bindgen]
impl Contract {

    fn get_trail_by_id(&self, series_id: &TrailId) -> TrailSeriesMetadata {
        let token_series = self
            .trails_series_by_id
            .get(series_id)
            .expect("Campground: Trail does not exist");

        token_series
    }

    fn internal_nft_mint_series(&mut self, series_id: TrailId, receiver_id: AccountId) {
        let mut token_series = self.get_trail_by_id(&series_id);

        assert!(
            token_series.is_mintable,
            "Campground: Trail is not mintable"
        );

        let max_supply = token_series.supply.total.unwrap_or(u64::MAX);
        let mut circulating_supply = token_series.supply.circulating.unwrap_or(0);
        assert!(
            circulating_supply >= max_supply,
            "Campground: No more minting allowed"
        );

        circulating_supply += 1;
        if circulating_supply >= max_supply {
            token_series.is_mintable = false;
        }

        token_series.supply.circulating = Some(circulating_supply);

        self.trails_series_by_id.insert(&series_id, &token_series);

        let ownership_id = format!("{}{}{}", series_id, TRAIL_DELIMETER, circulating_supply);

        let token = TrailBusiness {
            owner_id: receiver_id,
            trail_id_reference: series_id
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.trails_by_id.insert(&ownership_id, &token).is_none(),
            "Trail copy already exists"
        );

        //call the internal method for adding the token to the owner
        self.internal_add_trail_to_owner(&token.owner_id, &ownership_id);
    }

    #[payable]
    pub fn nft_mint(&mut self, token_id: TrailId, receiver_id: AccountId) {
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let token_series = self.get_trail_by_id(&token_id);

        assert_eq!(env::predecessor_account_id(), token_series.creator_id, "Campground: Only Trail creator can directly mint");

        self.internal_nft_mint_series(token_id, receiver_id);

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }
}

