import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'fs';
import { Account, KeyPair, keyStores, Near } from "near-api-js";
import { KeyStore } from "near-api-js/lib/key_stores";
import { NearConfig } from 'near-api-js/lib/near';
import { getConfig } from './config';

const STORE_PATH = '.neardev';

export function createFSKeyStore() {
    return new keyStores.UnencryptedFileSystemKeyStore(STORE_PATH);
}

const generateUniqueAccountId = (name: string) => `${name}-${Date.now()}-${Math.round(Math.random() * 1000000)}`;

const accountPath = (name: string) => `${STORE_PATH}/${name}-account`;

export function ensureStorePath() {
    if (!existsSync(STORE_PATH)) {
        mkdirSync(STORE_PATH);
    }
}

export async function createAccount(
    name: string,
    near: Near,
): Promise<string> {
    const config: ReturnType<typeof getConfig> = near.config;

    const accountId = config.accountId(generateUniqueAccountId(name));
    const newKeyPair = KeyPair.fromRandom('ed25519');

    await near.createAccount(accountId, newKeyPair.getPublicKey());
    await config.keyStore!.setKey(config.networkId, accountId, newKeyPair);

    writeFileSync(accountPath(name), accountId);

    return accountId;
};

export async function getAccount(name: string, near: Near): Promise<Account> {
    const accountId = readFileSync(accountPath(name)).toString();
    const account = await near.account(accountId);

    return account;
};
