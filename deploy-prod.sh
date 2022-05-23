#!/bin/bash
./build.sh && NEAR_ENV=mainnet near deploy --accountId ito.campground.near --wasmFile target/wasm32-unknown-unknown/release/ito_contract.wasm && NEAR_ENV=mainnet near call ito.campground.near new_default_meta '{ "owner_id": "campground.near", "treasury_id": "campgroundtreasury.near"}' --accountId campground.near
