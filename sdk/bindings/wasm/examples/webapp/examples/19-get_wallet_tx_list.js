import { initSdk } from './util';
import * as wasm from "cryptpay-sdk-wasm";

async function main() {
    let username = "satoshi";
    let start = 0;
    let limit = 10;
    let pin = "1234";
    let password = "StrongP@55w0rd";
    let mnemonic = process.env.MNEMONIC;

    const sdk = await initSdk();

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic);
    console.log("Wallet initialized!");

    let txs = await sdk.getWalletTransactionList(pin, start, limit);
    console.log("Wallet transactions list : " + JSON.stringify(txs));
}

export { main }
