const {NearTestInstance} = require("./near.js");
const { spawn } = require('child_process');

describe("Campground <> Near Tests", () => {
    let near;
    let andres, luis, campground;
    let andresName, luisName, campgroundName;

    let campgroundAddress;

    jest.setTimeout(600000);

    let testnetCmd;
    beforeAll(async () => {
        try {
            testnetCmd = spawn("npm", ["run", "clean"], {detached: true});
            testnetCmd.on('error', function (err) {
                console.log(err);
            });
            await new Promise((r) => setTimeout(r, 5000));
        } catch (e) {
            console.log(e);
        }
    });

    beforeEach(async () => {
        near = NearTestInstance();
        await near.initialized;
        const [_andres, _luis, _campground] = await near.initTest();
        andres = _andres.instance;
        luis = _luis.instance;
        campground = _campground.instance;

        andresName = _andres.name;
        luisName = _luis.name;
        campgroundName = _campground.name;

        campgroundAddress = `${campgroundName}.test.near`;

        await campground.new_default_meta({
            args: {
                owner_id: campgroundAddress,
                treasury_id: campgroundAddress
            }
        });
    })

    test("Campground metadata", async () => {
        const metadata = await (andres.nft_metadata());
        expect(metadata).toBeDefined();
        expect(metadata).toStrictEqual({
                spec: 'nft-1.0.0',
                name: 'Campground NFT Contract',
                symbol: 'CMPGRND',
                icon: null,
                base_uri: null,
                reference: null,
                reference_hash: null
        });
    });

    test("Campground contract owner", async () => {
        const owner = await andres.get_owner();
        expect(owner).toBe(campgroundAddress);
    });

    test("Campground treasury address", async () => {
        const owner = await andres.get_treasury_address();
        expect(owner).toBe(campgroundAddress);
    });

    test("Get minimum fee", async () => {
        const current_fee = await andres.get_current_fee();
        expect(current_fee).toBe("1000000000000000117440512");
    });

    test("Get fee percentage", async () => {
        const fee_percentage = await andres.get_fee_percentage();
        expect(fee_percentage).toBe(5);
    });

    const createTrail = async (price) => {
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

        return await andres.create_trail_series({
            args: {
                metadata: trailMetadata,
                price: price || "10000000000000000000000000" // One Near
            },
            amount: "5780000000000000000000"
        });
    }

    test("Create Trail Serie", async () => {
        const createIto = await createTrail();
        expect(createIto).toBeDefined();
        const address = `${andresName}.test.near`;
        expect(createIto.owner_id).toBe(address);
        expect(await andres.is_creator({
            series_id: createIto.token_id,
            owner_id: address,
        })).toBeTruthy();
        const trails_by_creator = await andres.get_all_trails_by_creator({
            creator_id: address
        });
        expect(trails_by_creator.length).toBe(1);
        expect(trails_by_creator[0].is_mintable).toBeTruthy();
        expect(trails_by_creator[0].supply.total).toBe(10);
        expect(trails_by_creator[0].metadata.title).toBe("My Trail");
    });

    test("Buy series Less than Price", async () => {
        const createIto = await createTrail();
        try {
            await luis.nft_buy_series({
                args: {
                    trail_series_id: createIto.token_id,
                    receiver_id: `${luisName}.test.near`
                },
                amount: "1000000000000000000000000"
            });
            expect(true).toBeFalsy();
        } catch (e) {
            console.log(e);
            expect(e.kind.ExecutionError).toContain("Smart contract panicked: panicked at 'Campground: Attached deposit is less than price");
        }
    });

    test("Buy series", async () => {
        const createIto = await createTrail("5000000000000000000000000");
        const getCampgroundBalance = async () => await campground.account.getAccountBalance();
        const getLuisBalance = async () => await luis.account.getAccountBalance();
        const beforeBuying = await getCampgroundBalance();
        const luisBeforeBuying = await getLuisBalance();
        await luis.nft_buy_series({
            args: {
                trail_series_id: createIto.token_id,
                receiver_id: `${luisName}.test.near`
            },
            amount: "5007940000000001000000000"
        });
        const afterBuying = await getCampgroundBalance();
        const luisAfterBuying = await getLuisBalance();
        expect(Number(beforeBuying.total)).toBeLessThan(Number(afterBuying.total));
        expect(Number(luisBeforeBuying.total)).toBeGreaterThan(Number(luisAfterBuying.total));
        const approximateCampgroundRevenue = Number(5000000000000000000000000) * 0.1;
        expect(Number(afterBuying.total)).toBeGreaterThanOrEqual(approximateCampgroundRevenue);
        expect(Number(luisAfterBuying.total)).toBeGreaterThan(4.98e+24)
        expect(Number(luisAfterBuying.total)).toBeLessThan(5.0e+24)
    });

    afterAll(async () => {
        process.kill(-testnetCmd.pid);
        await new Promise(resolve => setTimeout(() => resolve(), 10000));
    });



});
