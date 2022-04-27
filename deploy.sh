#!/bin/bash
./build.sh && near deploy --accountId campgroundv3.testnet --wasmFile target/wasm32-unknown-unknown/release/ito_contract.wasm && near call campgroundv3.testnet new_default_meta '{ "owner_id": "campgroundv3.testnet", "treasury_id": "campgroundtreasury.testnet"}' --accountId campgroundv3.testnet
