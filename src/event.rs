use std::fmt::Display;

use near_sdk::serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "standard")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub enum NearEvent {
    Nep171(Nep171Event),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Nep171Event {
    pub version: String,
    #[serde(flatten)]
    pub event_kind: Nep171EventKind,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
#[serde(crate = "near_sdk::serde")]
pub enum Nep171EventKind {
    NftMint(Vec<NftMintData>),
    NftTransfer(Vec<NftTransferData>),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftMintData {
    pub owner_id: String,
    pub token_ids: Vec<String>,
    pub memo: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftTransferData {
    pub authorized_id: Option<String>,
    pub old_owner_id: String,
    pub new_owner_id: String,
    pub token_ids: Vec<String>,
    pub memo: Option<String>,
}

impl Display for NearEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("EVENT_JSON:{}", self.to_json_string()))
    }
}

impl NearEvent {
    pub fn new_171(version: String, event_kind: Nep171EventKind) -> Self {
        NearEvent::Nep171(Nep171Event {
            version,
            event_kind,
        })
    }

    pub fn new_171_v1(event_kind: Nep171EventKind) -> Self {
        NearEvent::new_171("1.0.0".to_string(), event_kind)
    }

    pub fn nft_transfer(data: Vec<NftTransferData>) -> Self {
        NearEvent::new_171_v1(Nep171EventKind::NftTransfer(data))
    }

    pub fn nft_mint(data: Vec<NftMintData>) -> Self {
        NearEvent::new_171_v1(Nep171EventKind::NftMint(data))
    }

    pub(crate) fn to_json_string(&self) -> String {
        near_sdk::serde_json::to_string(self).unwrap()
    }

    pub fn log(&self) {
        near_sdk::env::log_str(&self.to_string());
    }

    pub fn log_nft_mint(owner_id: String, token_ids: Vec<String>, memo: Option<String>) {
        NearEvent::log_nft_mints(vec![NftMintData {
            owner_id,
            token_ids,
            memo,
        }]);
    }

    pub fn log_nft_mints(data: Vec<NftMintData>) {
        NearEvent::nft_mint(data).log();
    }

    pub fn log_nft_transfer(
        old_owner_id: String,
        new_owner_id: String,
        token_ids: Vec<String>,
        memo: Option<String>,
        authorized_id: Option<String>,
    ) {
        NearEvent::log_nft_transfers(vec![NftTransferData {
            authorized_id,
            old_owner_id,
            new_owner_id,
            token_ids,
            memo,
        }]);
    }

    pub fn log_nft_transfers(data: Vec<NftTransferData>) {
        NearEvent::nft_transfer(data).log();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tokens(s_vec: Vec<&str>) -> Vec<String> {
        s_vec.iter().map(|t| t.to_string()).collect()
    }

    #[test]
    fn nft_mint() {
        let owner_id = "bob".to_string();
        let token_ids = make_tokens(vec!["0", "1"]);
        let mint_log = NftMintData {
            owner_id,
            token_ids,
            memo: None,
        };
        let event_log = NearEvent::nft_mint(vec![mint_log]);
        assert_eq!(
            serde_json::to_string(&event_log).unwrap(),
            r#"{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"bob","token_ids":["0","1"]}]}"#
        );
    }

    #[test]
    fn nft_mints() {
        let owner_id = "bob".to_string();
        let token_ids = make_tokens(vec!["0", "1"]);
        let mint_log = NftMintData {
            owner_id,
            token_ids,
            memo: None,
        };
        let event_log = NearEvent::nft_mint(vec![
            mint_log,
            NftMintData {
                owner_id: "alice".to_string(),
                token_ids: make_tokens(vec!["2", "3"]),
                memo: Some("has memo".to_string()),
            },
        ]);
        assert_eq!(
            serde_json::to_string(&event_log).unwrap(),
            r#"{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"bob","token_ids":["0","1"]},{"owner_id":"alice","token_ids":["2","3"],"memo":"has memo"}]}"#
        );
    }

    #[test]
    fn nft_transfer() {
        let old_owner_id = "bob".to_string();
        let new_owner_id = "alice".to_string();
        let token_ids = make_tokens(vec!["0", "1"]);
        let log = NearEvent::nft_transfer(vec![NftTransferData {
            old_owner_id,
            new_owner_id,
            authorized_id: None,
            token_ids,
            memo: None,
        }])
        .to_json_string();
        assert_eq!(
            log,
            r#"{"standard":"nep171","version":"1.0.0","event":"nft_transfer","data":[{"old_owner_id":"bob","new_owner_id":"alice","token_ids":["0","1"]}]}"#
        );
    }

    #[test]
    fn nft_transfers() {
        let old_owner_id = "bob";
        let new_owner_id = "alice";
        let token_ids = make_tokens(vec!["0", "1"]);
        let log = NearEvent::nft_transfer(vec![
            NftTransferData {
                old_owner_id: new_owner_id.to_string(),
                new_owner_id: old_owner_id.to_string(),
                authorized_id: Some("4".to_string()),
                token_ids: make_tokens(vec!["2", "3"]),
                memo: Some("has memo".to_string()),
            },
            NftTransferData {
                old_owner_id: old_owner_id.to_string(),
                new_owner_id: new_owner_id.to_string(),
                authorized_id: None,
                token_ids,
                memo: None,
            },
        ])
        .to_json_string();
        assert_eq!(
            log,
            r#"{"standard":"nep171","version":"1.0.0","event":"nft_transfer","data":[{"authorized_id":"4","old_owner_id":"alice","new_owner_id":"bob","token_ids":["2","3"],"memo":"has memo"},{"old_owner_id":"bob","new_owner_id":"alice","token_ids":["0","1"]}]}"#
        );
    }
}
