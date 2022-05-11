import BN from "bn.js";
import { Account, ConnectConfig, Contract } from "near-api-js";

// async function createContractUser(
//     accountPrefix: string,
//     contractAccountId: string,
//     contractMethods: any,
//     amount: any
// ) {
//     let accountId = accountPrefix + "." + this.config.masterAccount;
//     await this.masterAccount.createAccount(
//         accountId,
//         this.pubKey,
//         amount || new BN(10).pow(new BN(25))
//     );
//     // this.keyStore.setKey(this.config.networkId, accountId, this.masterKey);
//     const account = new Account(this.near.connection, accountId);
//     return new Contract(
//         account,
//         contractAccountId,
//         contractMethods
//     );
// }

export function getConfig(env: string):
    ConnectConfig &
    { accountId: (prefix: string) => string } {
    switch (env) {
        case 'sandbox':
            return {
                networkId: 'sandbox',
                nodeUrl: 'http://localhost:3030',
                masterAccount: 'test.near',
                keyPath: '/tmp/near-sandbox/validator_key.json',
                headers: {},
                accountId: function (prefix: string): string {
                    return `${prefix}.${this.masterAccount}`;
                },
            };
        case 'testnet':
            return {
                networkId: 'testnet',
                nodeUrl: 'https://rpc.testnet.near.org',
                walletUrl: 'https://wallet.testnet.near.org',
                helperUrl: 'https://helper.testnet.near.org',
                headers: {},
                accountId: function (prefix: string): string {
                    return `${prefix}.${this.networkId}`;
                },
            };
    }

    throw new Error(`Config ${env} not found`);
};
