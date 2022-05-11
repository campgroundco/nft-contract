import { existsSync, readFileSync, writeFileSync } from 'fs';
import { Account, KeyPair, keyStores, Near } from "near-api-js";
import { getConfig } from './config';

const STORE_PATH = '.neardev';

export function createFSKeyStore() {
    return new keyStores.UnencryptedFileSystemKeyStore(STORE_PATH);
}

const generateUniqueAccountId = (name: string) => `${name}-${Date.now()}-${Math.round(Math.random() * 1000000)}`;

const accountPath = (name: string, networkId: string) => `${STORE_PATH}/${networkId}/${name}.account`;

export async function createAccount(
    name: string,
    near: Near,
): Promise<string> {
    const config: ReturnType<typeof getConfig> = near.config;

    const accountId = config.accountId(generateUniqueAccountId(name));

    console.log(`Creating account '${accountId}' for network '${config.networkId}'`);

    const newKeyPair = KeyPair.fromRandom('ed25519');

    await near.createAccount(accountId, newKeyPair.getPublicKey());

    await config.keyStore!.setKey(config.networkId, accountId, newKeyPair);

    writeFileSync(accountPath(name, config.networkId), accountId);

    return accountId;
};

function accountExists(name: string, networkId: string) {
    return existsSync(accountPath(name, networkId));
}

export async function getAccount(name: string, near: Near): Promise<Account> {
    const networkId = near.connection.networkId;

    let accountId;
    if (accountExists(name, networkId)) {
        accountId = readFileSync(accountPath(name, networkId)).toString();
        console.log(`Account '${accountId}' for network '${networkId}' cached`);
    } else {
        accountId = await createAccount(name, near);
    }

    const account = await near.account(accountId);

    return account;
};
