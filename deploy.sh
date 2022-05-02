#!/bin/bash
./build.sh && near deploy --accountId v1.campground.testnet --wasmFile target/wasm32-unknown-unknown/release/ito_contract.wasm && near call v1.campground.testnet new_default_meta '{ "owner_id": "v1.campground.testnet", "treasury_id": "campgroundtreasury.testnet"}' --accountId v1.campground.testnet
