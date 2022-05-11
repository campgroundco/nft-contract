import test from 'ava';
import BN from 'bn.js';
import { Account, connect, Contract, KeyPair, keyStores, Near } from 'near-api-js';
import { formatNearAmount, parseNearAmount } from 'near-api-js/lib/utils/format';
import { getConfig } from './config';
import { deployContract, initContract } from './deploy';
import * as ITO from '../ito';
import { createFSKeyStore, getAccount } from './store';

// function NEAR(amount: string) {
//     return new BN(parseNearAmount(amount)!);
// }

const env = process.argv[2];

let ito: Account;
let owner: Contract & ITO.Contract;
let alice: Contract & ITO.Contract;
let bob: Contract & ITO.Contract;

test.before(async _t => {
    const near = await connect({
        keyStore: createFSKeyStore(),
        ...getConfig(env),
    });

    function contract(account: Account): Contract & ITO.Contract {
        return new Contract(account, ito.accountId, ITO.ContractMethods) as any;
    }

    ito = await deployContract(near, 'ito', 'target/wasm32-unknown-unknown/release/ito_contract.wasm');
    owner = contract(await getAccount('owner', near));
    alice = contract(await getAccount('alice', near));
    bob = contract(await getAccount('bob', near));

    await initContract<ITO.Contract, keyof ITO.Contract>(owner,
        {
            func: 'new_default_meta',
            args: {
                owner_id: owner.account.accountId,
                treasury_id: owner.account.accountId
            },
        }
    );

});

test('contract should return correct metadata', async t => {
    const metadata = await alice.nft_metadata();
    t.deepEqual(metadata, {
        spec: 'nft-1.0.0',
        name: 'Campground NFT Contract',
        symbol: 'CMPGRND',
        icon: null,
        base_uri: null,
        reference: null,
        reference_hash: null
    });
});

test('contract should return correct contract owner', async t => {
    const contractOwner = await alice.get_owner();
    t.is(contractOwner, owner.account.accountId);
});

test('contract should return campground treasury account id', async t => {
    const contractOwner = await alice.get_treasury_address();
    t.is(contractOwner, owner.account.accountId);
});

test.skip('contract should return minimum fee', async t => {
    const current_fee = await alice.get_current_fee();
    t.is(current_fee, parseNearAmount('0.1')!);
});

test('contract should return fee percentage', async t => {
    const fee_percentage = await alice.get_fee_percentage();
    t.is(fee_percentage, 5);
});
