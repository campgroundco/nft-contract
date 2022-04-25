#!/bin/bash
./build.sh && near deploy --accountId campgroundv2.testnet --wasmFile target/wasm32-unknown-unknown/release/ito_contract.wasm
