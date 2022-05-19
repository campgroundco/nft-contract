use crate::*;
use near_sdk::ext_contract;

/// NEP-178 interface definition.
///
/// See https://nomicon.io/Standards/Tokens/NonFungibleToken/ApprovalManagement.
pub trait NonFungibleTokenApproval {
    /// Approve an account ID to transfer a token on your behalf.
    fn nft_approve(&mut self, token_id: TrailId, account_id: AccountId, msg: Option<String>);

    /// Check if the passed in account has access to approve the token ID
    fn nft_is_approved(
        &self,
        token_id: TrailId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    );

    /// Revoke a specific account from transferring the token on your behalf.
    fn nft_revoke(&mut self, token_id: TrailId, account_id: AccountId);

    /// Revoke all accounts from transferring the token on your behalf.
    fn nft_revoke_all(&mut self, token_id: TrailId);
}

#[ext_contract(ext_non_fungible_approval_receiver)]
trait NonFungibleTokenApprovalsReceiver {
    //cross contract call to an external contract that is initiated during nft_approve
    fn nft_on_approve(
        &mut self,
        token_id: TrailId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

#[near_bindgen]
impl NonFungibleTokenApproval for Contract {
    //allow a specific account ID to approve a token on your behalf
    #[payable]
    fn nft_approve(&mut self, token_id: TrailId, account_id: AccountId, msg: Option<String>) {
        /*
            FILL THIS IN
        */
    }

    //check if the passed in account has access to approve the token ID
    fn nft_is_approved(
        &self,
        token_id: TrailId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) {
        /*
            FILL THIS IN
        */
    }

    //revoke a specific account from transferring the token on your behalf
    #[payable]
    fn nft_revoke(&mut self, token_id: TrailId, account_id: AccountId) {
        /*
            FILL THIS IN
        */
    }

    //revoke all accounts from transferring the token on your behalf
    #[payable]
    fn nft_revoke_all(&mut self, token_id: TrailId) {
        /*
            FILL THIS IN
        */
    }
}
