use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::market::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;

mod internal;
mod approval; 
mod enumeration; 
mod metadata; 
mod market;
mod nft_core; 
mod royalty;
mod create_serie;
mod bridge;

pub const TRAIL_DELIMETER: char = ':';
const MAX_PRICE: Balance = 1_000_000_000 * 10u128.pow(24);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub trails_per_owner: LookupMap<AccountId, UnorderedSet<TrailIdAndCopyNumber>>,

    //keeps track of the token struct for a given token ID
    pub trails_by_id: LookupMap<TrailIdAndCopyNumber, TrailBusiness>,

    //keeps track of the token metadata for a given token ID
    pub trails_series_by_id: UnorderedMap<TrailId, TrailSeries>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    // Fee from 1 to 100, each unit representing %. 5 = 5% of each market sale
    pub campground_fee: u64,

    // Where campground fees will be sent
    pub campground_treasury_address: AccountId
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId, treasury_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Campground NFT Contract".to_string(),
                symbol: "CMPGRND".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
            treasury_id
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata, treasury_id: AccountId) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            trails_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            trails_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            trails_series_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            campground_fee: 5,
            campground_treasury_address: treasury_id
        };

        //return the Contract object
        this
    }
}
