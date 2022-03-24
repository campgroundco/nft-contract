const {NearTestInstance} = require("./near.js");

describe("Campground <> Near Tests", () => {
    let near;
    let andres, luis;
    let andresName, luisName;

    jest.setTimeout(600000);
    beforeAll(async () => {

        near = NearTestInstance();
        await near.initialized;
        const [_andres, _luis] = await near.initTest();
        andres = _andres.instance;
        luis = _luis.instance;

        andresName = _andres.name;
        luisName = _luis.name;

        await andres.new_default_meta({
            args: {
                owner_id: "andres.test.near",
                treasury_id: "andres.test.near"
            }
        });
    })

    test("Campground metadata", async () => {
        const metadata = await (andres.nft_metadata());
        expect(metadata).toBeDefined();
        expect(metadata).toBe({
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
        expect(owner).toBe("andres.test.near");
    });

    test("Campground treasury address", async () => {
        const owner = await andres.get_treasury_address();
        expect(owner).toBe("andres.test.near");
    });

    test("Get minimum fee", async () => {
        const current_fee = await andres.get_current_fee();
        expect(current_fee).toBe(10000000000000000000000000 * 0.1);
    });

    test("Get fee percentage", async () => {
        const fee_percentage = await andres.get_fee_percentage();
        expect(fee_percentage).toBe(5);
    });

    test("Create Trail Serie", async () => {
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

       const createIto = await andres.create_trail_series({
           args: {
               metadata: trailMetadata,
               price: "10000000000000000000000000" // One Near
           },
            amount: "5780000000000000000000"
       });
        expect(createIto).toBeDefined();
        const address = `${andresName}.test.near`;
        expect(createIto.owner_id).toBe(address);
        expect(await andres.is_creator({
            series_id: createIto.token_id,
            owner_id: address,
        })).toBeTruthy();
        expect((await andres.get_all_trails_by_creator({
            creator_id: address
        })).length).toBe(1);
    });



});
