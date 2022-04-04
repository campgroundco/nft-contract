function guidGenerator() {
    var S4 = function() {
        return (((1+Math.random())*0x10000)|0).toString(16).substring(1);
    };
    return (S4()+S4()+"-"+S4()+"-"+S4()+"-"+S4()+"-"+S4()+S4()+S4());
}

const getConfig = (env, port) => {
    const usePort = port || 3030;
    switch (env) {
        case "sandbox":
        case "local":
            return {
                networkId: "sandbox",
                nodeUrl: `http://localhost:${usePort}`,
                masterAccount: "test.near",
                contractAccount: `ito-${guidGenerator()}.test.near`,
                keyPath: "/tmp/near-sandbox/validator_key.json",
            };
    }
}

const contractMethods = {
    viewMethods: ["series_exists", "get_owner", "get_trail_by_id", "is_owner", "get_trail_business", "get_all_trails_by_owner", "get_current_fee", "get_fee_percentage", "get_treasury_address", "is_creator", "get_trail_by_id_optional", "get_all_trails_by_creator"],
    changeMethods: ["create_trail_series", "buy_series", "nft_mint", "nft_metadata", "trail_ticket", "new_default_meta"],
};

module.exports = {
    getConfig,
    contractMethods,
    guidGenerator
}
