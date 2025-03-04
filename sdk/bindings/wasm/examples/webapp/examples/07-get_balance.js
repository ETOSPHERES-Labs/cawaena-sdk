import * as wasm from "cryptpay-sdk-wasm";
import { initSdk } from './util';

async function main() {
    console.log("start");
    const sdk = await initSdk();
    let username = "satoshi";
    let password = "StrongP@55w0rd";
    let pin = "1234";
    let mnemonic = process.env.MNEMONIC;

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic);

    let address = await sdk.generateNewAddress(pin);
    console.log("Address:", address);


    let balance = await sdk.getWalletBalance(pin);
    console.log("Balance:", balance);

}

export { main }
