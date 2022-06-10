use ito_contract::admin::AdminBridge;
use ito_contract::vars::WHITELISTED_ADDRESS_MINTING_KEY;
use ito_contract::{
    create_serie::CreateTrailSeries, Contract, JsonTrail, TrailResource, TrailSeriesMetadata,
};
use near_sdk::{json_types::U128, test_utils::VMContextBuilder, testing_env, AccountId, Balance};

pub const STORAGE_FOR_CREATE_SERIES: Balance = 6960000000000000000000;

pub fn owner() -> AccountId {
    AccountId::new_unchecked("campground_owner.near".into())
}

pub fn alice() -> AccountId {
    AccountId::new_unchecked("alice".into())
}

pub fn bob() -> AccountId {
    AccountId::new_unchecked("bob".into())
}

pub fn carol() -> AccountId {
    AccountId::new_unchecked("carol".into())
}

pub fn treasury() -> AccountId {
    AccountId::new_unchecked("campground_treasury.near".into())
}

pub fn new_treasury() -> AccountId {
    AccountId::new_unchecked("campground_new_treasury.near".into())
}

pub fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .current_account_id(owner())
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);
    builder
}

pub fn setup_contract() -> (VMContextBuilder, Contract) {
    let mut context = VMContextBuilder::new();
    testing_env!(context.predecessor_account_id(owner()).build());
    let mut contract = Contract::new_default_meta(owner(), treasury());
    contract.add_setting(
        String::from(WHITELISTED_ADDRESS_MINTING_KEY),
        carol().to_string(),
    );
    (context, contract)
}

pub fn create_series(
    contract: &mut Contract,
    title: &str,
    starts_at: Option<u64>,
    expires_at: Option<u64>,
    price: Option<U128>,
    tickets: Option<u64>,
    resources: Option<Vec<TrailResource>>,
    allow_user_minting: Option<bool>,
) -> JsonTrail {
    contract.create_trail_series(
        TrailSeriesMetadata {
            title: String::from(title),
            description: String::new(),
            tickets_amount: tickets.unwrap_or(100),
            media: None,
            data: None,
            resources: resources.unwrap_or(vec![TrailResource {
                title: Some(format!("{}-{}", title, "resource")),
                description: None,
                media: format!("{}.png", title),
                extra: None,
                reference: None,
            }]),
            starts_at,
            expires_at,
            reference: None,
            campground_id: String::from("CMPGR123"),
        },
        price,
        None,
        None,
        allow_user_minting,
    )
}
