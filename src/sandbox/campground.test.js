const {NearTestInstance} = require("./near.js");

describe("Campground <> Near Tests", () => {
    let near;
    let andres, luis;

    jest.setTimeout(600000);
    beforeAll(async () => {

        near = NearTestInstance();
        await near.initialized;
        const [_andres, _luis] = await near.initTest();
        andres = _andres;
        luis = _luis;

        await andres.new_default_meta({
            args: {
                owner_id: "andres.test.near",
                treasury_id: "andres.test.near"
            }
        });
    })

    test("Campground metadata", async () => {
        console.log(await (andres.nft_metadata()));
    });
});
