import { expect } from 'chai';
import { Account, Contract } from 'near-api-js';
import { FinalExecutionStatus } from 'near-api-js/lib/providers';
import { parseNearAmount } from 'near-api-js/lib/utils/format';
import * as ITO from '../ito';
import { MAX_GAS_ALLOWED } from './lib/deploy';
import { setup } from './lib/setup';

let ito: Account;
let owner: Contract & ITO.Contract;
let alice: Contract & ITO.Contract;
let bob: Contract & ITO.Contract;

describe('admin', () => {

    before(async () => {
        [ito, owner, alice, bob] = await setup();
    });

    it('should change treasury address', async () => {
        await owner.change_campground_treasury_address({ args: { addr: 'hola.somenet' } } as any);
        const newTreasuryAddress = await owner.get_treasury_address();
        expect(newTreasuryAddress).to.be.equal('hola.somenet');
    });

    it('should measure gas for change treasury address', async () => {
        const itoBalance = await ito.getAccountBalance();
        const ownerBalance = await owner.account.getAccountBalance();

        const tx = await owner.account.functionCall({
            contractId: ito.accountId,
            methodName: 'change_campground_treasury_address',
            args: {
                addr: 'hola.somenet'
            },
        });

        const itoPostBalance = await ito.getAccountBalance();
        const ownerPostBalance = await owner.account.getAccountBalance();

        expect((tx.status as FinalExecutionStatus).SuccessValue).to.be.equal('');
    });
});
