use crate::{JsonTrail, TrailId};

pub trait SeriesBridge {
    fn nft_token(&self, token_id: TrailId) -> Option<JsonTrail>;
}
