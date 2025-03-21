import * as wasm from "../pkg/etopay_sdk_wasm";// Import the WASM module
import { initSdk } from './utils';

async function main() {
    let username = "satoshi";
    const sdk = await initSdk(username);
    await sdk.createNewUser(username);

    console.log("user created successfully");

}

main();
