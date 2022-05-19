import BN from 'bn.js';
import { expect } from 'chai';
import { Account, Contract } from 'near-api-js';
import { ServerError } from 'near-api-js/lib/utils/rpc_errors';
import * as ITO from '../ito';
import { MAX_GAS_ALLOWED } from './lib/deploy';
import { ITOContract, setup } from './lib/setup';

let ito: Account;
let owner: ITOContract;
let alice: ITOContract;
let bob: ITOContract;
let carol: ITOContract;

describe('create-series', () => {

    before(async () => {
        [ito, owner, alice, bob, carol] = await setup();
    });

    it.only('should create trail serie', async () => {
        const createIto = await createTrail(alice, ito);

        const address = alice.account.accountId;
        console.log(alice.account.accountId);
        expect(createIto.owner_id).to.be.equal(address);
        expect(await alice.is_creator({
            series_id: createIto.token_id,
            owner_id: address,
        })).to.be.true;
        const trails_by_creator = await alice.get_all_trails_by_creator({
            creator_id: address
        });
        // expect(trails_by_creator.length).to.be.equal(1);
        expect(trails_by_creator[0].is_mintable).to.be.true;
        expect(trails_by_creator[0].supply.total).to.be.equal(10);
        expect(trails_by_creator[0].metadata.title).to.be.equal("My Trail");
    });

    // it('should panic when buying series with less than price', async () => {
    //     const createIto = await createTrail(carol);
    //     const err = await t.throwsAsync(
    //         bob.nft_buy_series({
    //             args: {
    //                 trail_series_id: createIto.token_id,
    //                 receiver_id: bob.account.accountId,
    //             },
    //             amount: "1000000000000000000000000"
    //         } as any));
    //     t.regex((err as any)?.kind?.ExecutionError, /Smart contract panicked: panicked at 'Campground: Attached deposit is less than price/);
    // });

    it('contract should be able to allow account to buy series (last assert not working)', async () => {
        const createIto = await createTrail(carol, ito, "5000000000000000000000000");
        const getCampgroundBalance = () => owner.account.getAccountBalance();
        const getLuisBalance = () => bob.account.getAccountBalance();
        const beforeBuying = await getCampgroundBalance();
        const luisBeforeBuying = await getLuisBalance();

        console.log('ito', await ito.getAccountBalance());
        console.log('alice', await alice.account.getAccountBalance());
        console.log('bob', await bob.account.getAccountBalance());
        console.log('carol', await carol.account.getAccountBalance());

        await bob.nft_buy_series({
            args: {
                trail_series_id: createIto.token_id,
                receiver_id: bob.account.accountId,
            },
            amount: "5007940000000001000000000"
        } as any);

        console.log('ito', await ito.getAccountBalance());
        console.log('alice', await alice.account.getAccountBalance());
        console.log('bob', await bob.account.getAccountBalance());
        console.log('carol', await carol.account.getAccountBalance());

        const afterBuying = await getCampgroundBalance();
        const luisAfterBuying = await getLuisBalance();

        // t.assert(Number(beforeBuying.total) < Number(afterBuying.total));
        // t.assert(Number(luisBeforeBuying.total) > Number(luisAfterBuying.total));
        // const approximateCampgroundRevenue = Number(5000000000000000000000000) * 0.1;
        // t.assert(Number(afterBuying.total) >= approximateCampgroundRevenue);
        // t.assert(Number(luisAfterBuying.total) > 4.98e+24);
    });

});

const createTrail = async (account: ITOContract, ito: Account, price?: any) => {
    const args = {
        metadata: {
            title: "My Trail",
            description: "Some description",
            tickets_amount: 10,
            resources: [
                {
                    media: 'http://arweave.net/image.png'
                }
            ],
            campground_id: '123'
        },
        price: price || '10000000000000000000000000',
    };

    const attachedDeposit = '5780000000000000000000';

    console.log('ito', await ito.getAccountBalance());
    console.log('alice', await account.account.getAccountBalance());
    const tx = await account.account.functionCall({
        contractId: ito.accountId,
        methodName: 'create_trail_series',
        args,
        gas: MAX_GAS_ALLOWED,
        attachedDeposit: attachedDeposit.bn(),
    });
    console.log('ito', await ito.getAccountBalance());
    console.log('alice', await account.account.getAccountBalance());
    console.log(tx.receipts_outcome.map(r => r.outcome));
    console.log(tx.transaction);
    console.log(tx.transaction_outcome);

    const itoBalance = await ito.getAccountBalance();
    // const accBalance = await account.account.getAccountBalance();


    const trail = await account.create_trail_series({
        args,
        amount: attachedDeposit,
    } as any);

    console.log('ito', await ito.getAccountBalance());
    console.log('alice', await account.account.getAccountBalance());

    const postItoBalance = await ito.getAccountBalance();
    // const postAccBalance = await account.account.getAccountBalance();

    // expect(postItoBalance.stateStaked.bn().gt(itoBalance.stateStaked.bn())).to.be.true;
    const trailStorage = postItoBalance.stateStaked.bn().sub(itoBalance.stateStaked.bn());
    const available = attachedDeposit.bn().sub(trailStorage);

    // expect(postItoBalance.total.bn()).to.be.equal(itoBalance.total.bn().add(available));
    // expect(postItoBalance.available.bn()).to.be.equal(itoBalance.available.bn().add(available));
    // expect(postItoBalance.staked.bn()).to.be.equal('0'.bn());

    return trail;
};
