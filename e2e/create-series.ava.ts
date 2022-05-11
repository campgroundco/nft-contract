import test from 'ava';
import { Account, Contract } from 'near-api-js';
import * as ITO from '../ito';
import { setup } from './lib/setup';

let ito: Account;
let owner: Contract & ITO.Contract;
let alice: Contract & ITO.Contract;
let bob: Contract & ITO.Contract;

test.before(async _t => {
    [ito, owner, alice, bob] = await setup();
});

const createTrail = async (price?: any) => {
    const trailMetadata = {
        title: "My Trail",
        description: "Some description",
        tickets_amount: 10,
        resources: [
            {
                media: "http://arweave.net/image.png"
            }
        ],
        campground_id: "123"
    };

    return await alice.create_trail_series({
        args: {
            metadata: trailMetadata,
            price: price || "10000000000000000000000000" // One Near
        },
        amount: "5780000000000000000000"
    } as any);
}

test("contract should create trail serie", async t => {
    const createIto = await createTrail();
    const address = alice.account.accountId;
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

// test("Buy series Less than Price", async () => {
//     const createIto = await createTrail();
//     try {
//         await luis.nft_buy_series({
//             args: {
//                 trail_series_id: createIto.token_id,
//                 receiver_id: `${luisName}.test.near`
//             },
//             amount: "1000000000000000000000000"
//         });
//         expect(true).toBeFalsy();
//     } catch (e) {
//         console.log(e);
//         expect(e.kind.ExecutionError).toContain("Smart contract panicked: panicked at 'Campground: Attached deposit is less than price");
//     }
// });

// test("Buy series", async () => {
//     const createIto = await createTrail("5000000000000000000000000");
//     const getCampgroundBalance = async () => await campground.account.getAccountBalance();
//     const getLuisBalance = async () => await luis.account.getAccountBalance();
//     const beforeBuying = await getCampgroundBalance();
//     const luisBeforeBuying = await getLuisBalance();
//     await luis.nft_buy_series({
//         args: {
//             trail_series_id: createIto.token_id,
//             receiver_id: `${luisName}.test.near`
//         },
//         amount: "5007940000000001000000000"
//     });
//     const afterBuying = await getCampgroundBalance();
//     const luisAfterBuying = await getLuisBalance();
//     expect(Number(beforeBuying.total)).toBeLessThan(Number(afterBuying.total));
//     expect(Number(luisBeforeBuying.total)).toBeGreaterThan(Number(luisAfterBuying.total));
//     const approximateCampgroundRevenue = Number(5000000000000000000000000) * 0.1;
//     expect(Number(afterBuying.total)).toBeGreaterThanOrEqual(approximateCampgroundRevenue);
//     expect(Number(luisAfterBuying.total)).toBeGreaterThan(4.98e+24)
//     expect(Number(luisAfterBuying.total)).toBeLessThan(5.0e+24)
// });