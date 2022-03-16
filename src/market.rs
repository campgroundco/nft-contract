use crate::*;
use crate::bridge::SeriesBridge;

#[near_bindgen]
impl Contract {

    fn internal_nft_mint_series(&mut self, series_id: TrailId, receiver_id: AccountId) -> TrailIdAndCopyNumber {
        let mut token_series = self.get_trail_by_id(&series_id);

        assert!(
            token_series.is_mintable,
            "Campground: Trail is not mintable"
        );

        let max_supply = token_series.supply.total;
        let mut circulating_supply = token_series.supply.circulating;
        /// 10 (max) > 1 (circulating) = true
        assert!(
            max_supply > circulating_supply || max_supply == circulating_supply,
            "Campground: No more minting allowed"
        );

        circulating_supply += 1;
        if circulating_supply >= max_supply {
            token_series.is_mintable = false;
        }

        token_series.supply.circulating = circulating_supply;

        self.trails_series_by_id.insert(&series_id, &token_series);

        let ownership_id: TrailIdAndCopyNumber = format!("{}{}{}", series_id, TRAIL_DELIMETER, circulating_supply);

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

        ownership_id
    }

    #[payable]
    pub fn buy_series(&mut self, trail_series_id: TrailId, receiver_id: AccountId) -> TrailIdAndCopyNumber {
        let initial_storage_usage = env::storage_usage();

        let trail_series = self.trails_series_by_id.get(&trail_series_id).expect("Campground: Trail series does not exist");
        let price = trail_series.price;
        let attached_deposit = env::attached_deposit();

        assert!(attached_deposit >= price, "Campground: Attached deposit is less than price");

        let for_treasury = calculate_fee(price, self.campground_fee);
        let price_deducted = price - for_treasury;

        assert!(price_deducted > 0, "Campground: Buying operation is invalid");
        assert!(for_treasury > 0, "Campground: a fee needs to be paid");

        let trail_id_with_copy: TrailIdAndCopyNumber = self.internal_nft_mint_series(trail_series_id, receiver_id);

        Promise::new(trail_series.creator_id).transfer(price_deducted);
        Promise::new(self.campground_treasury_address.clone()).transfer(for_treasury);

        refund_deposit(env::storage_usage() - initial_storage_usage);

        trail_id_with_copy
    }

    #[payable]
    pub fn nft_mint(&mut self, token_id: TrailId, receiver_id: AccountId) -> TrailIdAndCopyNumber {
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let token_series = self.get_trail_by_id(&token_id);

        assert_eq!(env::predecessor_account_id(), token_series.creator_id, "Campground: Only Trail creator can directly mint");

        let trail_mint_id= self.internal_nft_mint_series(token_id, receiver_id);

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);

        trail_mint_id
    }
}

