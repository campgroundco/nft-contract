use crate::bridge::SeriesBridge;
use crate::event::NearEvent;
use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn nft_internal_mint_series(
        &mut self,
        series_id: TrailId,
        receiver_id: AccountId,
    ) -> TrailIdAndCopyNumber {
        let mut token_series = self.get_trail_by_id(&series_id);

        assert!(
            token_series.is_mintable,
            "Campground: Trail is not mintable"
        );

        let max_supply = token_series.supply.total;
        let mut circulating_supply = token_series.supply.circulating;
        // 10 (max) > 1 (circulating) = true
        assert!(
            max_supply > circulating_supply || max_supply == circulating_supply,
            "Campground: No more minting allowed"
        );

        circulating_supply += 1;
        if circulating_supply >= max_supply {
            token_series.is_mintable = false;
        }

        token_series.supply.circulating = circulating_supply;

        self.trails_metadata_by_id.insert(&series_id, &token_series);

        let ownership_id: TrailIdAndCopyNumber =
            format!("{}{}{}", series_id, TRAIL_DELIMETER, circulating_supply);

        let token = TrailBusiness {
            owner_id: receiver_id.clone(),
            token_id: series_id.to_owned(),
            partial_metadata: partial_metadata_from_trail_series(&token_series),
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&ownership_id, &token).is_none(),
            "Trail copy already exists"
        );

        assert!(
            self.token_metadata_by_id
                .insert(&ownership_id, &series_id)
                .is_none(),
            "Trail copy might already exist"
        );

        //call the internal method for adding the token to the owner
        self.internal_add_trail_to_owner(&token.owner_id, &ownership_id);

        let price = &token_series.price.clone();

        NearEvent::log_nft_mint(
            receiver_id.to_string(),
            vec![ownership_id.clone()],
            Some(near_sdk::serde_json::json!({ "price": &price }).to_string()),
        );

        ownership_id
    }

    /// Returns the price of the given `trail_series_id`.
    /// The price is the final amount to be payed to buy the nft.
    pub fn nft_get_series_price(&self, trail_series_id: TrailId) -> U128 {
        let trail_series = self
            .trails_metadata_by_id
            .get(&trail_series_id)
            .expect("Campground: Trail series does not exist");
        U128(get_price_and_fee(&trail_series).0)
    }

    /// Buys a trail series if still available given a price and attached deposit.
    #[payable]
    pub fn nft_buy_series(
        &mut self,
        trail_series_id: TrailId,
        receiver_id: AccountId,
    ) -> TrailIdAndCopyNumber {
        let trail_series = self
            .trails_metadata_by_id
            .get(&trail_series_id)
            .expect("Campground: Trail series does not exist");
        let (price, fee) = get_price_and_fee(&trail_series);
        let attached_deposit = env::attached_deposit();

        assert_eq!(
            attached_deposit, price,
            "Campground: Attached deposit needs to be equal to ITO price or Campground Fee"
        );

        // If for_treasury <= campground_minimum_fee_yocto_near, the buyer pays the fees
        // Otherwise, the seller pays the fee (price - for_treasury)
        let price_after_fee = price - fee;

        let trail_id_with_copy: TrailIdAndCopyNumber =
            self.nft_internal_mint_series(trail_series_id, receiver_id);

        if price_after_fee > 0 {
            Promise::new(trail_series.creator_id).transfer(price_after_fee);
        }

        Promise::new(self.campground_treasury_address.clone()).transfer(fee);

        trail_id_with_copy
    }

    #[payable]
    pub fn nft_mint(&mut self, token_id: TrailId, receiver_id: AccountId) -> TrailIdAndCopyNumber {
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let token_series = self.get_trail_by_id(&token_id);

        assert_eq!(
            env::predecessor_account_id(),
            token_series.creator_id,
            "Campground: Only Trail creator can directly mint"
        );

        let trail_mint_id = self.nft_internal_mint_series(token_id, receiver_id);

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes, 0);

        trail_mint_id
    }
}

/// Returns both the price and the corresponding Campground fee
/// of the given `trail_series`.
/// The first component represents the price to be payed to buy the nft.
/// The second components represents the Campground fee.
fn get_price_and_fee(trail_series: &TrailSeries) -> (u128, u128) {
    if trail_series.price.0 > trail_series.campground_fee_near.0 {
        (trail_series.price.0, trail_series.campground_fee_near.0)
    } else {
        (
            trail_series.campground_fee_near.0,
            trail_series.campground_fee_near.0,
        )
    }
}
