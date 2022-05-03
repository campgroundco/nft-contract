// TypeScript bindings generated with near-syn v0.3.0 https://github.com/epam/near-syn on 2022-05-03 19:44:55.234884 UTC

// Exports common NEAR Rust SDK types
export type U64 = string;
export type I64 = string;
export type U128 = string;
export type Base64VecU8 = string;
export type I128 = string;
export type Balance = U128;
export type AccountId = string;
export type ValidAccountId = string;

/**
 */
export interface AdminBridge {
    /**
     */
    change_campground_fee(args: { fee: number }, gas?: any): Promise<void>;

    /**
     */
    change_campground_treasury_address(args: { addr: AccountId }, gas?: any): Promise<void>;

    /**
     */
    change_campground_minimum_fee(args: { fee: Balance }, gas?: any): Promise<void>;

}

/**
 */
export interface NonFungibleTokenCore {
    /**
     */
    nft_approve(args: { token_id: TrailId, account_id: AccountId, msg: string|null }, gas?: any, amount?: any): Promise<void>;

    /**
     */
    nft_is_approved(args: { token_id: TrailId, approved_account_id: AccountId, approval_id: number|null }): Promise<void>;

    /**
     */
    nft_revoke(args: { token_id: TrailId, account_id: AccountId }, gas?: any, amount?: any): Promise<void>;

    /**
     */
    nft_revoke_all(args: { token_id: TrailId }, gas?: any, amount?: any): Promise<void>;

}

/**
 */
export interface SeriesBridge {
    /**
     */
    series_exists(args: { series_id: TrailId }): Promise<boolean>;

    /**
     */
    get_owner(): Promise<AccountId>;

    /**
     */
    get_trail_by_id_optional(args: { series_id: TrailId }): Promise<TrailSeries|null>;

    /**
     */
    get_trail_by_id(args: { series_id: TrailId }): Promise<TrailSeries>;

    /**
     */
    is_owner(args: { series_id: TrailIdAndCopyNumber, owner_id: AccountId }): Promise<boolean>;

    /**
     */
    is_creator(args: { series_id: TrailId, owner_id: AccountId }): Promise<boolean>;

    /**
     */
    get_trail_business(args: { trail_and_copy_id: TrailIdAndCopyNumber }): Promise<TrailBusiness|null>;

    /**
     */
    get_all_trails_by_owner(args: { owner_id: AccountId }): Promise<TrailSeries[]>;

    /**
     */
    get_all_trails_by_creator(args: { creator_id: AccountId }): Promise<TrailSeries[]>;

    /**
     */
    get_current_fee(): Promise<u128>;

    /**
     */
    get_fee_percentage(): Promise<number>;

    /**
     */
    get_treasury_address(): Promise<AccountId>;

    /**
     */
    is_caller_contract_owner(): Promise<boolean>;

}

/**
 */
export interface CreateTrailSeries {
    /**
     */
    create_trail_series(args: { metadata: TrailSeriesMetadata, price: U128|null }, gas?: any, amount?: any): Promise<JsonTrail>;

}

/**
 */
export interface Contract {
    /**
     */
    nft_total_supply(): Promise<U128>;

    /**
     */
    nft_tokens(args: { from_index: U128|null, limit: number|null }): Promise<JsonTrail[]>;

    /**
     */
    nft_supply_for_owner(args: { account_id: AccountId }): Promise<U128>;

    /**
     */
    trail_tickets_for_owner(args: { account_id: AccountId, from_index: U128|null, limit: number|null }): Promise<JsonTrail[]>;

    /**
     */
    nft_tokens_for_owner(args: { account_id: AccountId, from_index: U128|null, limit: number|null }): Promise<JsonTrail[]>;

}

/**
 */
export interface Contract {
    /**
     *  Initialization function (can only be called once).
     *  This initializes the contract with default metadata so the
     *  user doesn't have to manually type metadata.
     */
    new_default_meta: { owner_id: AccountId, treasury_id: AccountId };

    /**
     *  Initialization function (can only be called once).
     *  This initializes the contract with metadata that was passed in and
     *  the `owner_id`.
     */
    new: { owner_id: AccountId, metadata: NFTContractMetadata, treasury_id: AccountId };

}

/**
 */
export interface Contract {
    /**
     */
    nft_buy_series(args: { trail_series_id: TrailId, receiver_id: AccountId }, gas?: any, amount?: any): Promise<TrailIdAndCopyNumber>;

}

/**
 */
export type TrailId = string;

/**
 */
export type TrailIdAndCopyNumber = string;

/**
 */
export type Payout = {
    /**
     */
    payout: Record<AccountId, U128>;

}

/**
 */
export type NFTContractMetadata = {
    /**
     */
    spec: string;

    /**
     */
    name: string;

    /**
     */
    symbol: string;

    /**
     */
    icon: string|null;

    /**
     */
    base_uri: string|null;

    /**
     */
    reference: string|null;

    /**
     */
    reference_hash: Base64VecU8|null;

}

/**
 */
export type TrailResource = {
    /**
     */
    title: string|null;

    /**
     */
    description: string|null;

    /**
     */
    media: string;

    /**
     */
    extra: string|null;

    /**
     */
    reference: string|null;

}

/**
 */
export type SeriesSupply = {
    /**
     */
    total: number;

    /**
     */
    circulating: number;

}

/**
 */
export type TrailSeriesMetadata = {
    /**
     */
    title: string;

    /**
     */
    description: string;

    /**
     */
    tickets_amount: number;

    /**
     */
    media: string|null;

    /**
     */
    data: string|null;

    /**
     */
    resources: TrailResource[];

    /**
     */
    starts_at: number|null;

    /**
     */
    expires_at: number|null;

    /**
     */
    reference: string|null;

    /**
     */
    campground_id: string;

}

/**
 */
export type TokenMetadata = {
    /**
     */
    title: string|null;

    /**
     */
    description: string|null;

    /**
     */
    media: string|null;

    /**
     */
    media_hash: Base64VecU8|null;

    /**
     */
    copies: number|null;

    /**
     */
    issued_at: number|null;

    /**
     */
    expires_at: number|null;

    /**
     */
    starts_at: number|null;

    /**
     */
    updated_at: number|null;

    /**
     */
    extra: string|null;

    /**
     */
    reference: string|null;

    /**
     */
    reference_hash: Base64VecU8|null;

}

/**
 */
export type TrailSeries = {
    /**
     */
    is_mintable: boolean;

    /**
     */
    creator_id: AccountId;

    /**
     */
    issue_at: number;

    /**
     */
    metadata: TrailSeriesMetadata;

    /**
     */
    supply: SeriesSupply;

    /**
     */
    price: U128;

}

/**
 */
export type TrailBusiness = {
    /**
     */
    owner_id: AccountId;

    /**
     */
    token_id: TrailId;

    /**
     */
    partial_metadata: TokenMetadata;

}

/**
 */
export type JsonTrail = {
    /**
     */
    token_id: TrailId;

    /**
     */
    owner_id: AccountId;

    /**
     */
    series: TrailSeries;

    /**
     */
    metadata: TokenMetadata;

}

/**
 */
export interface NonFungibleTokenMetadata {
    /**
     */
    nft_metadata(): Promise<NFTContractMetadata>;

}

/**
 */
export interface NonFungibleTokenCore {
    /**
     */
    nft_transfer(args: { receiver_id: AccountId, token_id: TrailIdAndCopyNumber, memo: string|null }, gas?: any, amount?: any): Promise<void>;

    /**
     */
    nft_transfer_call(args: { receiver_id: AccountId, token_id: TrailId, memo: string|null, msg: string }, gas?: any, amount?: any): Promise<PromiseOrValue>;

    /**
     */
    nft_token(args: { token_id: TrailIdAndCopyNumber }): Promise<JsonTrail|null>;

}

/**
 */
export interface NonFungibleTokenCore {
    /**
     */
    nft_payout(args: { token_id: TrailId, balance: U128, max_len_payout: number }): Promise<void>;

    /**
     */
    nft_transfer_payout(args: { receiver_id: AccountId, token_id: TrailId, approval_id: number, memo: string, balance: U128, max_len_payout: number }, gas?: any, amount?: any): Promise<void>;

}

export interface Contract extends AdminBridge, NonFungibleTokenCore, SeriesBridge, CreateTrailSeries, NonFungibleTokenMetadata, NonFungibleTokenCore, NonFungibleTokenCore {}

export const ContractMethods = {
    viewMethods: [
        "nft_is_approved",
        "series_exists",
        "get_owner",
        "get_trail_by_id_optional",
        "get_trail_by_id",
        "is_owner",
        "is_creator",
        "get_trail_business",
        "get_all_trails_by_owner",
        "get_all_trails_by_creator",
        "get_current_fee",
        "get_fee_percentage",
        "get_treasury_address",
        "is_caller_contract_owner",
        "nft_total_supply",
        "nft_tokens",
        "nft_supply_for_owner",
        "trail_tickets_for_owner",
        "nft_tokens_for_owner",
        "nft_metadata",
        "nft_token",
        "nft_payout",
    ],
    changeMethods: [
        "change_campground_fee",
        "change_campground_treasury_address",
        "change_campground_minimum_fee",
        "nft_approve",
        "nft_revoke",
        "nft_revoke_all",
        "create_trail_series",
        "nft_buy_series",
        "nft_transfer",
        "nft_transfer_call",
        "nft_transfer_payout",
    ],
};
