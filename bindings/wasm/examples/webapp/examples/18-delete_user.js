import { initSdk } from './util';
import * as wasm from "etopay-sdk-wasm";

async function main() {

    let username = "archiveme";
    let pin = "1234";
    let password = "StrongP@55w0rd";
    let mnemonic = process.env.MNEMONIC;

    const sdk = await initSdk();

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic);
    await sdk.deleteUser(pin);

    console.log("user deleted_successfully");
}

export { main }
