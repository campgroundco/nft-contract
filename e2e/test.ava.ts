import test from 'ava';
import BN from 'bn.js';
import { readFileSync, writeFileSync } from 'fs';
import { Account, connect, Contract, KeyPair, keyStores, Near } from 'near-api-js';
// import { NearConfig } from 'near-api-js/lib/near';
import { formatNearAmount, parseNearAmount } from 'near-api-js/lib/utils/format';
import { getConfig } from './config';
// import { ContractMethods as M } from '../ito';
import { createAccount, createFSKeyStore, ensureStorePath } from './store';

// const accountExists = (prefix: string) => fs.existsSync(path.resolve(__dirname, `../neardev/${prefix}-account`));

// function NEAR(amount: string) {
//     return new BN(parseNearAmount(amount)!);
// }

ensureStorePath();

test('near amount check', t => {
    t.is(formatNearAmount('1'), `0.${'0'.repeat(23)}1`);
    t.is(parseNearAmount('1'), '1' + '0'.repeat(24));
    t.is(parseNearAmount('0.1'), '1' + '0'.repeat(23));

    t.is(formatNearAmount('10000000000000000000000000'), '10');
    t.is(formatNearAmount('12114570000000000000000000'), '12.11457');

    t.is(formatNearAmount(new BN(10).pow(new BN(25)).toString()), '10');
});

test('wip', async t => {
    const near = await connect({
        keyStore: createFSKeyStore(),
        ...getConfig('sandbox'),
    });

    // const keyFile = JSON.parse(readFileSync(config.keyPath, "utf-8"));
    // const masterKey = KeyPair.fromString(keyFile.secret_key);
    // const pubKey = masterKey.getPublicKey();
    // keyStore.setKey(config.networkId, config.masterAccount, masterKey);


    const a = await createAccount('alice', near);

    const account = await near.account(a);
    console.log(await account.getAccountBalance());

    t.pass();

    // const contract = readFileSync("target/wasm32-unknown-unknown/release/ito_contract.wasm");
    // const masterAccount = new Account(near.connection, config.masterAccount);


    // const contractAccount = await masterAccount.createAndDeployContract(
    //     'holiiiia1.test.near',
    //     pubKey,
    //     contract,
    //     new BN(parseNearAmount('50')!),
    // );

    // const accountId = 'lala' + "." + config.masterAccount;
    // await masterAccount.createAccount(accountId, pubKey, NEAR('10'));
    // const account = new Account(near.connection, accountId);
    // const c = new Contract(account, contractAccount.accountId, ito.ContractMethods) as Contract & ito.Contract;
    // await (c as any).new_default_meta({
    //     args: {
    //         owner_id: contractAccount.accountId,
    //         treasury_id: contractAccount.accountId,
    //     }
    // });

    // t.is((await masterAccount.getAccountBalance()).total, 'asdf');
    // t.is((await masterAccount.getAccountBalance()).available, 'asdf');
    // t.is((await masterAccount.getAccountBalance()).staked, 'asdf');
    // t.is((await masterAccount.getAccountBalance()).stateStaked, 'asdf');
    // console.log(masterAccount);
    // const bar = Promise.resolve('bar');
    // t.is(await bar, 'bar');
    // t.is(masterAccount, new Account(near.connection, ''));
});
