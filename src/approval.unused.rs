use crate::*;
use near_sdk::ext_contract;

/// Approval Management
/// NEP-178 interface definition.
///
/// For more info,
/// see https://nomicon.io/Standards/Tokens/NonFungibleToken/ApprovalManagement.
///
/// Trait used when it is desired to have a non-fungible token that has a traditional escrow or approval system.
/// This allows Alice to allow Bob to take only the token with the unique identifier "19" but not others.
///
/// It should be noted that in the [core non-fungible token standard] there
/// is a method to do _transfer and call_ which may be preferred over using
/// an [approval management standard] in certain use cases.
///
/// [approval management standard]: https://nomicon.io/Standards/NonFungibleToken/ApprovalManagement.html
/// [core non-fungible token standard]: https://nomicon.io/Standards/NonFungibleToken/Core.html

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
