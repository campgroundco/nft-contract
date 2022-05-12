# ITO Contract

This represents the ITO (Initial Trail Offering) Smart Contract system created for and by Campground.

# View Methods

| Method | Description | Return |
| ------ | ----------- | ---- |
| series_exists | Verifies whether a trail is available in the smart contract | bool |
| get_owner | Returns the owner of the smart contract | AccountId |
| get_trail_by_id_optional | Returns a trail if found or null if not | Struct |
| get_trail_by_id | Returns a trail if found or panics if not | Struct |
| is_owner | Verifies whether a given user (AccountId) owns the copy of a trail | bool |
| is_creator | Verifies whether a given user (AccountId) is creator of a given trail | bool |
| get_trail_business | Returns the business information of a trail or null if not found | Struct |
| get_all_trails_by_owner | Returns all the trail copies owned by a given user (AccountId) | Array<Struct> |
| get_all_trails_by_creator | Returns all the trails created by a given user (AccountId) | Array<Struct> |
| get_current_fee | Returns the current minimum fee in YoctoNEAR by campground | u128 |
| get_fee_percentage | Returns the percentage amount Campground takes from each buy order if higher than minimum fee | u64 |
| get_treasury_address | Returns the address where treasury funds are transferred to | String | 
| is_caller_contract_owner | Whether caller is the owner of the contract | String | 
| trail_tickets_for_owner | Similar to get_all_trails_by_owner with pagination | Array<Struct> | 
| trail_ticket | Gets the trail information for a specific copy | Struct |

# Change Methods

| Method | Description | Return |
| ------ | ----------- | ---- |
| create_trail_series | Creates a series (trail) inside the smart contract | Struct | 
| nft_buy_series | Buys a trail series if still available given a price and attached deposit | String |
| new_default_meta | Initializes the contract | Void |
| change_campground_fee | Change Campground percentage fee | Void |
| change_campground_treasury_address | Change treasury address | Void |
| change_campground_minimum_fee | Change campground minimum fee in yoctoNEAR | Void |

# Interacting with Contract 

```javascript
const cmpgContract = new Contract(walletAccount, "campgroundv1.testnet", {
    viewMethods: ["get_all_trails_by_creator"],
    changeMethods: ["create_trail_series"]
});

// Change method
await cmpgContract.create_trail_series({
    args: {
        ...
    },
    amount: '10000000000000000000000000' // One near in yocto near (attached deposit)
});


// View Method
await cmpgContract.get_trail_by_id({
    creator_id: "address.testnet"
});
```

## Prerequisites

To be able to compile using the `wasm` target,
you need to add it using `rustup`

```sh
rustup target add wasm32-unknown-unknown 
```

## Building

To build the ITO Smart Contract to `wasm` to be deployed into NEAR, run

```sh
cargo build --target wasm32-unknown-unknown --release
```

The `yarn build` is a shortcut for this command.
Check [`.cargo/config.toml`](.cargo/config.toml) to review compiler options for this target.

## Testing

These tests live under the `tests/` folder.
Each file tests a different aspect of the ITO Smart Contract.
Given that each test file creates a [separate crate](https://doc.rust-lang.org/book/ch11-03-test-organization.html#the-tests-directory),
you can find common test setup in the `context` module.

To run unit tests using the host target, run

```sh
cargo test
```

## End-to-End Testing

These tests live under the [`e2e/`](e2e/) folder.
They deploy the ITO Smart Contract to a live network
and run TypeScript tests using the [`near-js-api`](https://docs.near.org/docs/api/javascript-library).
The [`e2e/lib/`](e2e/lib/) folder contains common test setup and configuration shared across test suites.

Currently our End-to-End test support both a local `sandbox` or the public `testnet` networks.

### On Local `sandbox`

To run the End-to-End tests on a local sandbox, first you need to download it and compile it.

```sh
yarn sandbox:clone
yarn sandbox:make
```

The command `yarn sandbox:make` may take several minutes, so be patient.
This creates the `nearcore/target/debug/neard-sandbox` binary that you can execute to start a local instance of NEAR.
In one terminal, start the sandbox node, note the `&` to start a background process.

```sh
yarn sandbox:init
yarn sandbox:run &
```

Then, in another terminal, you are ready to run the end-to-end tests against the sandbox node

Finally,

```sh
yarn test:sandbox
```

Alternatively, if you want to cache last used test accounts

```sh
STORE=1 yarn test:sandbox
```

For more details on Sandbox and End-to-End testing,
see <https://docs.near.org/docs/develop/contracts/sandbox#start-and-stop-sandbox-node>.

### On Public `testnet`

To run the same e2e tests mentioned above on but NEAR testnet, just run

```sh
yarn test:testnet
```

Alternatively, if you want to cache last used test accounts

```sh
STORE=1 yarn test:testnet
```

This script should work out-of-the-box.
No configuration required.

### Cleaning Account Cache

The e2e tests writes account information in the `.neardev/` folder.
If you need to start from scratch, you can just remove this folder

```sh
yarn clean
```

## Generate TypeScript bindings and Markdown documentation

[`near-syn`](https://github.com/acuarica/near-syn) is a utility to generate TypeScript bindings and Markdown documentation from a contract written in Rust for the NEAR protocol.

Install `near-syn` with

```sh
cargo install near-syn
```

Then

```sh
yarn ts
yarn md
```

This command creates `ito.ts` and `ITO.md`.
