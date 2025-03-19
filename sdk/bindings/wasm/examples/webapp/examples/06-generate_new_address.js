import * as wasm from "etopay-sdk-wasm";
import { initSdk } from './util';

async function main() {

    let username = "satoshi";
    let password = "StrongP@55w0rd";
    let pin = "1234";
    const sdk = await initSdk();
    let mnemonic = process.env.MNEMONIC;
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);

    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic);
    console.log("Wallet initialized!");

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].key);

    let address = await sdk.generateNewAddress(pin);
    console.log("Address:", address);
}

export { main }
