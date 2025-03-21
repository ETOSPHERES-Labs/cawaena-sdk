import { initSdk } from './util';
import * as wasm from "etopay-sdk-wasm";

async function main() {
    let username = "satoshi";
    let pin = "1234";

    const sdk = await initSdk();
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);

    let course = await sdk.getExchangeRate();
    console.log(course);
}

export { main }
