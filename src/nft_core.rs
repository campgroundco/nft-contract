use crate::*;
use near_sdk::{ext_contract, log, Gas, PromiseResult};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const NO_DEPOSIT: Balance = 0;

pub trait NonFungibleTokenCore {
    //transfers an NFT to a receiver ID
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TrailIdAndCopyNumber, memo: Option<String>);

    //transfers an NFT to a receiver and calls a function on the receiver ID's contract
    /// Returns `true` if the token was transferred from the sender's account.
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TrailIdAndCopyNumber,
        memo: Option<String>,
        msg: String,
    );

    //get information about the NFT token passed in
    fn nft_token(&self, token_id: TrailIdAndCopyNumber) -> Option<JsonTrail>;
}

#[ext_contract(ext_non_fungible_token_receiver)]
trait NonFungibleTokenReceiver {
    //Method stored on the receiver contract that is called via cross contract call when nft_transfer_call is called
    /// Returns `true` if the token should be returned back to the sender.
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TrailIdAndCopyNumber,
        msg: String,
    );
}

#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {
    /*
        resolves the promise of the cross contract call to the receiver contract
        this is stored on THIS contract and is meant to analyze what happened in the cross contract call when nft_on_transfer was called
        as part of the nft_transfer_call method
    */
    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TrailIdAndCopyNumber,
    );
}

/*
    resolves the promise of the cross contract call to the receiver contract
    this is stored on THIS contract and is meant to analyze what happened in the cross contract call when nft_on_transfer was called
    as part of the nft_transfer_call method
*/
trait NonFungibleTokenResolver {
    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TrailIdAndCopyNumber,
    );
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    //implementation of the nft_transfer method. This transfers the NFT from the current owner to the receiver.
    #[payable]
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TrailIdAndCopyNumber, memo: Option<String>) {
        let initial_storage_usage = env::storage_usage();
        let sender_id = env::predecessor_account_id();

        let (new_token, previous_token) = self.internal_transfer(
            &sender_id,
            &receiver_id,
            &token_id,
            None,
            memo,
        );

        refund_deposit(env::storage_usage() - initial_storage_usage, 0);
    }

    //implementation of the transfer call method. This will transfer the NFT and call a method on the reciver_id contract
    #[payable]
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TrailId,
        memo: Option<String>,
        msg: String,
    ) {
        /*
            FILL THIS IN
        */
    }

    //get the information for a specific token ID
    fn nft_token(&self, token_id: TrailIdAndCopyNumber) -> Option<JsonTrail> {
        //if there is some token ID in the tokens_by_id collection
        if let Some(token) = self.tokens_by_id.get(&token_id) {
            //we'll get the metadata for that token
            let serie = self.trails_metadata_by_id.get(&token.token_id).unwrap();
            //we return the JsonToken (wrapped by Some since we return an option)
            Some(JsonTrail {
                token_id,
                owner_id: token.owner_id,
                series: serie,
                metadata: token.partial_metadata.to_owned(),
            })
        } else {
            //if there wasn't a token ID in the tokens_by_id collection, we return None
            None
        }
    }
}

#[near_bindgen]
impl NonFungibleTokenResolver for Contract {
    //resolves the cross contract call when calling nft_on_transfer in the nft_transfer_call method
    //returns true if the token was successfully transferred to the receiver_id
    #[private]
    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TrailId,
    ) {
        /*
            FILL THIS IN
        */
    }
}
