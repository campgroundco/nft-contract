use crate::*;
pub type TrailId = String;
pub type TrailIdAndCopyNumber = String;
//defines the payout type we'll be returning as a part of the royalty standards.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    /// Required, essentially a version like "nft-1.0.0"
    pub spec: String,
    /// required, ex. "Mosaics"
    pub name: String,
    /// required, ex. "MOSIAC"
    pub symbol: String,
    /// Data URL
    pub icon: Option<String>,
    /// Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    pub base_uri: Option<String>,
    /// URL to a JSON file with more info
    pub reference: Option<String>,
    /// Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
    pub reference_hash: Option<Base64VecU8>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TrailResource {
    /// ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub title: Option<String>,
    /// free-form description
    pub description: Option<String>,
    /// URL to associated media, preferably to decentralized, content-addressed storage
    pub media: String,
    /// anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub extra: Option<String>,
    /// Url referencing something of this resource
    pub reference: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SeriesSupply {
    pub total: u64,
    pub circulating: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TrailSeriesMetadata {
    pub title: String,
    pub description: String,
    pub tickets_amount: u64,
    pub media: Option<String>,
    pub data: Option<String>,
    pub resources: Vec<TrailResource>,
    /// When token starts being valid, Unix epoch in milliseconds
    pub starts_at: Option<u64>,
    /// When token expires, Unix epoch in milliseconds,
    pub expires_at: Option<u64>,
    /// Url referencing something of this resource,
    pub reference: Option<String>,
    pub campground_id: String,
}

/// As defined in NEP-177.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    /// ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub title: Option<String>,
    /// free-form description
    pub description: Option<String>,
    /// URL to associated media, preferably to decentralized, content-addressed storage
    pub media: Option<String>,
    /// Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub media_hash: Option<Base64VecU8>,
    /// number of copies of this set of metadata in existence when token was minted.
    pub copies: Option<u64>,
    /// When token was issued or minted, Unix epoch in milliseconds
    pub issued_at: Option<u64>,
    /// When token expires, Unix epoch in milliseconds
    pub expires_at: Option<u64>,
    /// When token starts being valid, Unix epoch in milliseconds
    pub starts_at: Option<u64>,
    /// When token was last updated, Unix epoch in milliseconds
    pub updated_at: Option<u64>,
    /// anything extra the NFT wants to store on-chain. Can be stringified JSON.
    pub extra: Option<String>,
    /// URL to an off-chain JSON file with more info.
    pub reference: Option<String>,
    /// Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
    pub reference_hash: Option<Base64VecU8>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TrailSeries {
    pub is_mintable: bool,
    pub creator_id: AccountId,
    pub issue_at: u64,
    pub metadata: TrailSeriesMetadata,
    pub supply: SeriesSupply,
    pub price: U128,
    pub campground_fee_near: U128,
    pub creator_royalty_near: Option<U128>,
    pub royalties: HashMap<AccountId, u32>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TrailBusiness {
    /// Owner of the token
    pub owner_id: AccountId,
    pub token_id: TrailId,
    pub partial_metadata: TokenMetadata,
}

/// The Json token is what will be returned from view calls.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonTrail {
    /// token ID
    pub token_id: TrailId,
    /// owner of the token
    pub owner_id: AccountId,
    /// token metadata
    pub series: TrailSeries,
    /// NEAR metadata
    pub metadata: TokenMetadata,
}

/// NEP-177 interface definition.
///
/// See https://nomicon.io/Standards/Tokens/NonFungibleToken/Metadata.
pub trait NonFungibleTokenMetadata {
    /// View call for returning the contract metadata
    fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
