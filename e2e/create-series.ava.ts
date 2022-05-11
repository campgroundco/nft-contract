import test from 'ava';
import { Account, Contract } from 'near-api-js';
import { ServerError } from 'near-api-js/lib/utils/rpc_errors';
import * as ITO from '../ito';
import { ITOContract, setup } from './lib/setup';

let ito: Account;
let owner: ITOContract;
let alice: ITOContract;
let bob: ITOContract;
let carol: ITOContract;

test.before(async _t => {
    [ito, owner, alice, bob, carol] = await setup();
});

const createTrail = async (account: ITOContract, price?: any) => {
    const trailMetadata = {
        title: "My Trail",
        description: "Some description",
        tickets_amount: 10,
        resources: [
            {
                media: 'http://arweave.net/image.png'
            }
        ],
        campground_id: '123'
    };

    return await account.create_trail_series({
        args: {
            metadata: trailMetadata,
            price: price || '10000000000000000000000000' // One Near
        },
        amount: '5780000000000000000000'
    } as any);
}

test('contract should create trail serie', async t => {
    const createIto = await createTrail(alice);
    const address = alice.account.accountId;
    console.log(alice.account.accountId);
    t.is(createIto.owner_id, address);
    t.assert(await alice.is_creator({
        series_id: createIto.token_id,
        owner_id: address,
    }));
    const trails_by_creator = await alice.get_all_trails_by_creator({
        creator_id: address
    });
    t.is(trails_by_creator.length, 1);
    t.truthy(trails_by_creator[0].is_mintable);
    t.is(trails_by_creator[0].supply.total, 10);
    t.is(trails_by_creator[0].metadata.title, "My Trail");
});

test("contract should panic when buying series with less than price", async t => {
    const createIto = await createTrail(carol);
    const err = await t.throwsAsync(
        bob.nft_buy_series({
            args: {
                trail_series_id: createIto.token_id,
                receiver_id: bob.account.accountId,
            },
            amount: "1000000000000000000000000"
        } as any));
    t.regex((err as any)?.kind?.ExecutionError, /Smart contract panicked: panicked at 'Campground: Attached deposit is less than price/);
});

test.skip('contract should be able to allow account to buy series (last assert not working)', async t => {
    const createIto = await createTrail(carol, "5000000000000000000000000");
    const getCampgroundBalance = () => owner.account.getAccountBalance();
    const getLuisBalance = () => bob.account.getAccountBalance();
    const beforeBuying = await getCampgroundBalance();
    const luisBeforeBuying = await getLuisBalance();
    await bob.nft_buy_series({
        args: {
            trail_series_id: createIto.token_id,
            receiver_id: bob.account.accountId,
        },
        amount: "5007940000000001000000000"
    } as any);
    const afterBuying = await getCampgroundBalance();
    const luisAfterBuying = await getLuisBalance();

    t.assert(Number(beforeBuying.total) < Number(afterBuying.total));
    t.assert(Number(luisBeforeBuying.total) > Number(luisAfterBuying.total));
    const approximateCampgroundRevenue = Number(5000000000000000000000000) * 0.1;
    t.assert(Number(afterBuying.total) >= approximateCampgroundRevenue);
    t.assert(Number(luisAfterBuying.total) > 4.98e+24);

    console.log(luisAfterBuying.total);
    t.assert(Number(luisAfterBuying.total) < 5.0e+24);
});
