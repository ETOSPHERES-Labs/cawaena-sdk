import * as wasm from "etopay-sdk-wasm";
import { initSdk } from './util';

async function main() {
    console.log("start");
    let username = "satoshi";
    const sdk = await initSdk();
    await sdk.createNewUser(username);
    console.log("done");
}

export { main }