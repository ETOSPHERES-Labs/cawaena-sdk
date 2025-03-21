import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk } from './utils';

async function main() {
    let username = "satoshi";
    let pin = "1234";

    const sdk = await initSdk(username);
    let password: string = (process.env.PASSWORD as string);
    let mnemonic: string = (process.env.MNEMONIC as string);

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].id);

    await sdk.createNewWallet(pin);
    console.log("Wallet initialized!");
    let address = await sdk.generateNewAddress(pin);
    console.log("Address:", address);
}

main();

