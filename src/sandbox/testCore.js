const getConfig = (env, port) => {
    const usePort = port || 3030;
    switch (env) {
        case "sandbox":
        case "local":
            return {
                networkId: "sandbox",
                nodeUrl: `http://localhost:${usePort}`,
                masterAccount: "test.near",
                contractAccount: "ito.test.near",
                keyPath: "/tmp/near-sandbox/validator_key.json",
            };
    }
}

const contractMethods = {
    viewMethods: ["series_exists", "get_owner", "get_trail_by_id", "is_owner", "get_trail_business", "get_all_trails_by_owner", "get_current_fee", "get_fee_percentage", "get_treasury_address"],
    changeMethods: ["create_trail_series", "trail_tickets_for_owner", "buy_series", "nft_mint", "nft_metadata", "trail_ticket", "new_default_meta"],
};

module.exports = {
    getConfig,
    contractMethods
}