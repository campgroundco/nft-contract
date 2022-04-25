pub trait SeriesBridge {
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;
}
