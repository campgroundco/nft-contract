import { expect } from 'chai';
import { Account, Contract } from 'near-api-js';
import { parseNearAmount } from 'near-api-js/lib/utils/format';
import * as ITO from '../ito';
import { setup } from './lib/setup';

let ito: Account;
let owner: Contract & ITO.Contract;
let alice: Contract & ITO.Contract;
let bob: Contract & ITO.Contract;

describe('init', () => {

    before(async () => {
        [ito, owner, alice, bob] = await setup();
    });

    it('should return correct metadata', async () => {
        const metadata = await alice.nft_metadata();
        expect(metadata).to.be.deep.equal({
            spec: 'nft-1.0.0',
            name: 'Campground NFT Contract',
            symbol: 'CMPGRND',
            icon: null,
            base_uri: null,
            reference: null,
            reference_hash: null
        });
    });

    it('should return correct contract owner', async () => {
        const contractOwner = await alice.get_owner();
        expect(contractOwner).to.be.equal(owner.account.accountId);
    });

    it('should return campground treasury account id', async () => {
        const contractOwner = await alice.get_treasury_address();
        expect(contractOwner).to.be.equal(owner.account.accountId);
    });

    it('should return minimum fee', async () => {
        const current_fee = await alice.get_current_fee();
        expect(current_fee).to.be.equal(parseNearAmount('0.1')!);
    });

    it('should return fee percentage', async () => {
        const fee_percentage = await alice.get_fee_percentage();
        expect(fee_percentage).to.be.equal(5);
    });

});
