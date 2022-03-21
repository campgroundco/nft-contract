const {contractMethods, getConfig} = require("./testCore.js");
const {readFileSync} = require("fs");
const nearAPI = require("near-api-js");
const BN = require("bn.js");

class NearTest {
    config;
    masterAccount;
    masterKey;
    pubKey;
    keyStore;
    near;
    initialized;

    constructor(env, port) {
        this.initialized = this.#initializeNear(env, port);
    }

    async #initializeNear(env, port) {
        this.config = getConfig(env, port);
        const keyFile = JSON.parse(readFileSync(this.config.keyPath, "utf-8"));
        this.masterKey = nearAPI.utils.KeyPair.fromString(
            keyFile.secret_key || keyFile.private_key
        );
        this.pubKey = this.masterKey.getPublicKey();
        this.keyStore = new nearAPI.keyStores.InMemoryKeyStore();
        this.keyStore.setKey(this.config.networkId, this.config.masterAccount, this.masterKey);
        this.near = await nearAPI.connect({
            deps: {
                keyStore: this.keyStore,
            },
            networkId: this.config.networkId,
            nodeUrl: this.config.nodeUrl,
            headers: {}
        });
        this.masterAccount = new nearAPI.Account(this.near.connection, this.config.masterAccount);
    }

    async createContractUser(
        accountPrefix,
        contractAccountId,
        contractMethods
    ) {
        let accountId = accountPrefix + "." + this.config.masterAccount;
        await this.masterAccount.createAccount(
            accountId,
            this.pubKey,
            new BN(10).pow(new BN(25))
        );
        this.keyStore.setKey(this.config.networkId, accountId, this.masterKey);
        const account = new nearAPI.Account(this.near.connection, accountId);
        return new nearAPI.Contract(
            account,
            contractAccountId,
            contractMethods
        );
    }

    async initTest() {
        const contract = readFileSync("../../target/wasm32-unknown-unknown/release/nft_simple.wasm");

        const _contractAccount = await this.masterAccount.createAndDeployContract(
            this.config.contractAccount,
            this.pubKey,
            contract,
            new BN(10).pow(new BN(25))
        );

        const andresUseContract = await this.createContractUser(
            "andres",
            this.config.contractAccount,
            contractMethods
        );

        const luisUseContract = await this.createContractUser(
            "luis",
            this.config.contractAccount,
            contractMethods
        );

        return [andresUseContract, luisUseContract]
    }

}

const NearTestInstance = () => new NearTest(process.env.NEAR_ENV || 'sandbox', process.env.NEAR_PORT);

module.exports = {
    NearTestInstance,
    NearTest
}
