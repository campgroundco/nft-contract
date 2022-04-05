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
| buy_series | Buys a trail series if still available given a price and attached deposit | String |
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
