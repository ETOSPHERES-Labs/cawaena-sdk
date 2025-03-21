import { initSdk } from './util';
import * as wasm from "etopay-sdk-wasm";

async function main() {
    let username = "satoshi";
    let pin = "1234";
    let password = "StrongP@55w0rd";
    let mnemonic = process.env.MNEMONIC;
    const sdk = await initSdk();

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic);

    let new_password = "new password"
    await sdk.setWalletPassword(pin, new_password);
    console.log("change password successful");

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].id);

    // use wallet
    let _address = await sdk.generateNewAddress(pin);
}

export { main }
