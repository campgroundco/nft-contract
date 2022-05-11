import test from 'ava';
import { Account, Contract } from 'near-api-js';
import { parseNearAmount } from 'near-api-js/lib/utils/format';
import * as ITO from '../ito';
import { setup } from './lib/setup';

let ito: Account;
let owner: Contract & ITO.Contract;
let alice: Contract & ITO.Contract;
let bob: Contract & ITO.Contract;

test.before(async _t => {
    [ito, owner, alice, bob] = await setup();
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
