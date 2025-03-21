import { debug } from "util";
import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk } from './utils';

async function main() {
    let username = "satoshi";
    let pin = "1234";
    let receiver = "alice";
    const sdk = await initSdk(username);
    let mnemonic: string = (process.env.MNEMONIC as string);
    let password: string = (process.env.PASSWORD as string);

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic);

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].id);

    let address = await sdk.generateNewAddress(pin);
    debug(`Generated new IOTA receiver address: ${address}`);
    let balance = await sdk.getWalletBalance(pin);

    console.log("balance : ", balance);

    let product_hash = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
    let app_data = JSON.stringify({
        "imageUrl": "https://httpbin.org/",
        "imageId": "a846ad10-fc69-4b22-b442-5dd740ace361"
    });

    let purchase_type = "CLIK";

    let purchase_id = await sdk.createPurchaseRequest(receiver, 2.0, product_hash, app_data, purchase_type);
    console.log("Purchase ID:", purchase_id);

}

main();

