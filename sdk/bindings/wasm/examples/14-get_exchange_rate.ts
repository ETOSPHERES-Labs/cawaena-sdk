import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk } from './utils';

async function main() {

    let username = "satoshi";
    let pin = "1234";
    const sdk = await initSdk(username);
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].key);

    let course = await sdk.getExchangeRate();
    console.log(course);
}

main();

