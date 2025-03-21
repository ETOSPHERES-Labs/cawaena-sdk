import { initSdk } from './util';
import * as wasm from "etopay-sdk-wasm";

async function main() {
    let username = "satoshi";
    let pin = "1234";
    let new_pin = "54321";
    let password = "StrongP@55w0rd";
    let mnemonic = process.env.MNEMONIC;
    const sdk = await initSdk();

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic);

    await sdk.resetPin(pin, new_pin);
    console.log("Reset pin successfully");

    await sdk.verifyPin(new_pin);
    console.log("new pin verified");
}

export { main }
