import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk } from './utils';

async function main() {
    let username = "archiveme";
    let pin = "1234";

    const sdk = await initSdk(username);
    let password: string = (process.env.PASSWORD as string);
    let mnemonic: string = (process.env.MNEMONIC as string);

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic);
    await sdk.deleteUser(pin);
    console.log("user deleted_successfully");
}

export { main }
