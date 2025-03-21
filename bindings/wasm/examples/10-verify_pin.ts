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

    await sdk.verifyPin(pin);
    console.log("Pin verified successfully");

}

main();


