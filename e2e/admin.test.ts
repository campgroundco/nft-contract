import { expect } from 'chai';
import { Account, Contract } from 'near-api-js';
import { parseNearAmount } from 'near-api-js/lib/utils/format';
import * as ITO from '../ito';
import { setup } from './lib/setup';

let ito: Account;
let owner: Contract & ITO.Contract;
let alice: Contract & ITO.Contract;
let bob: Contract & ITO.Contract;

describe('admin', () => {

    before(async () => {
        [ito, owner, alice, bob] = await setup();
    });

    it('contract should change treasury address', async () => {
        await owner.change_campground_treasury_address({ args: { addr: 'hola.somenet' } } as any);
        const newTreasuryAddress = await owner.get_treasury_address();
        expect(newTreasuryAddress).to.be.equal('hola.somenet');
    });

});
