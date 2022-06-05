use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet, LookupSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};
use std::collections::HashMap;

// pub use crate::approval::*;
use crate::internal::*;
pub use crate::market::*;
pub use crate::metadata::*;
pub use crate::nft_core::*;
// pub use crate::royalty::*;
// mod approval;
pub mod bridge;
pub mod create_serie;
pub mod enumeration;
pub mod internal;
pub mod market;
mod metadata;
pub mod nft_core;
// mod royalty;

pub mod admin;
pub mod event;

pub const TRAIL_DELIMETER: char = ':';
pub const ONE_NEAR: Balance = 1000000000000000000000000;
pub const BUY_STORAGE: Balance = 6920000000000000000000;
pub const MAX_PRICE: Balance = 1_000_000_000 * 10u128.pow(24);

/// Holds the state for the ITO (Initial Trail Offering) Smart Contract.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// Represents the owner of the contract.
    pub owner_id: AccountId,

    /// Keeps track of all the token IDs for a given account.
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TrailIdAndCopyNumber>>,

    /// Keeps track of the token struct for a given token ID.
    pub tokens_by_id: LookupMap<TrailIdAndCopyNumber, TrailBusiness>,

    /// Keeps track of the token metadata ID for a given token ID with copy number.
    pub token_metadata_by_id: UnorderedMap<TrailIdAndCopyNumber, TrailId>,

    /// Keeps track of the trail series by `TrailId`.
    pub trails_metadata_by_id: UnorderedMap<TrailId, TrailSeries>,

    /// Keeps track of the token created by creator, represented by `AccountId`.
    pub trails_series_by_creator: LookupMap<AccountId, UnorderedSet<TrailId>>,

    pub nonmintable_trails: LookupSet<TrailId>,

    /// Represents the metadata for the contract.
    pub metadata: LazyOption<NFTContractMetadata>,

    /// Fee from 1 to 100, each unit representing %. 5 = 5% of each market sale
    pub campground_fee: u64,

    /// Where campground fees will be sent.
    pub campground_treasury_address: AccountId,

    pub campground_minimum_fee_yocto_near: Balance,

    pub settings: UnorderedMap<String, String>
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokenPerCreator,
    TokensById,
    TokenMetadataById,
    TrailsMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[derive(BorshSerialize)]
pub enum StorageKeysV2 {
    Settings
}

#[derive(BorshSerialize)]
pub enum StorageKeysV3 {
    NonMintableTrails
}


#[near_bindgen]
impl Contract {
    /// Initialization function (can only be called once).
    /// This initializes the contract with default metadata so the
    /// user doesn't have to manually type metadata.
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
            treasury_id,
        )
    }

    /// Initialization function (can only be called once).
    /// This initializes the contract with metadata that was passed in and
    /// the `owner_id`.
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata, treasury_id: AccountId) -> Self {
        //create a variable of type Self with all the fields initialized.
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            trails_metadata_by_id: UnorderedMap::new(
                StorageKey::TrailsMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id.
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            campground_fee: 5,
            campground_treasury_address: treasury_id,
            campground_minimum_fee_yocto_near: ONE_NEAR / 10,
            trails_series_by_creator: LookupMap::new(
                StorageKey::TokenPerCreator.try_to_vec().unwrap(),
            ),
            settings: UnorderedMap::new(
                StorageKeysV2::Settings.try_to_vec().unwrap()
            ),
            nonmintable_trails: LookupSet::new(StorageKeysV3::NonMintableTrails.try_to_vec().unwrap())
        };

        //return the Contract object
        this
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate_v1_to_v2() -> Self {
        #[derive(BorshDeserialize)]
        pub struct CampgroundContractV1 {
            pub owner_id: AccountId,
            pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TrailIdAndCopyNumber>>,
            pub tokens_by_id: LookupMap<TrailIdAndCopyNumber, TrailBusiness>,
            pub token_metadata_by_id: UnorderedMap<TrailIdAndCopyNumber, TrailId>,
            pub trails_metadata_by_id: UnorderedMap<TrailId, TrailSeries>,
            pub trails_series_by_creator: LookupMap<AccountId, UnorderedSet<TrailId>>,
            pub metadata: LazyOption<NFTContractMetadata>,
            pub campground_fee: u64,
            pub campground_treasury_address: AccountId,
            pub campground_minimum_fee_yocto_near: Balance,
        }

        let state: CampgroundContractV1 = env::state_read().unwrap();

        Self {
            owner_id: state.owner_id,
            tokens_per_owner: state.tokens_per_owner,
            tokens_by_id: state.tokens_by_id,
            token_metadata_by_id: state.token_metadata_by_id,
            trails_metadata_by_id: state.trails_metadata_by_id,
            trails_series_by_creator: state.trails_series_by_creator,
            metadata: state.metadata,
            campground_fee: state.campground_fee,
            campground_treasury_address: state.campground_treasury_address,
            campground_minimum_fee_yocto_near: state.campground_minimum_fee_yocto_near,
            settings: UnorderedMap::new(
                StorageKeysV2::Settings.try_to_vec().unwrap()
            ),
            nonmintable_trails: LookupSet::new(StorageKeysV3::NonMintableTrails.try_to_vec().unwrap())
        }

    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate_v2_to_v3() -> Self {
        #[derive(BorshDeserialize)]
        pub struct CampgroundContractV2 {
            pub owner_id: AccountId,
            pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TrailIdAndCopyNumber>>,
            pub tokens_by_id: LookupMap<TrailIdAndCopyNumber, TrailBusiness>,
            pub token_metadata_by_id: UnorderedMap<TrailIdAndCopyNumber, TrailId>,
            pub trails_metadata_by_id: UnorderedMap<TrailId, TrailSeries>,
            pub trails_series_by_creator: LookupMap<AccountId, UnorderedSet<TrailId>>,
            pub metadata: LazyOption<NFTContractMetadata>,
            pub campground_fee: u64,
            pub campground_treasury_address: AccountId,
            pub campground_minimum_fee_yocto_near: Balance,
            pub settings: UnorderedMap<String, String>
        }

        let state: CampgroundContractV2 = env::state_read().unwrap();

        Self {
            owner_id: state.owner_id,
            tokens_per_owner: state.tokens_per_owner,
            tokens_by_id: state.tokens_by_id,
            token_metadata_by_id: state.token_metadata_by_id,
            trails_metadata_by_id: state.trails_metadata_by_id,
            trails_series_by_creator: state.trails_series_by_creator,
            metadata: state.metadata,
            campground_fee: state.campground_fee,
            campground_treasury_address: state.campground_treasury_address,
            campground_minimum_fee_yocto_near: state.campground_minimum_fee_yocto_near,
            settings: state.settings,
            nonmintable_trails: LookupSet::new(StorageKeysV3::NonMintableTrails.try_to_vec().unwrap())
        }

    }
}
