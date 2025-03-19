import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk } from './utils';

async function main() {
    let username = "satoshi";
    let pin = "1234";
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
    sdk.setNetwork(networks[0].key);

    let recipient_address = await sdk.generateNewAddress(pin);
    console.log("address", recipient_address);

    let balance_before = await sdk.getWalletBalance(pin);
    console.log("balance before sending amount", balance_before);

    const data = new TextEncoder().encode("wasm example");
    let tx_id = await sdk.sendAmount(pin, recipient_address, 1.0, data);
    console.log("sent amount with transaction", tx_id);

    let balance_after = await sdk.getWalletBalance(pin);
    console.log("balance after sending amount", balance_after);

}

main();


