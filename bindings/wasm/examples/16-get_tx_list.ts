import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk } from './utils';


async function main() {
    let username = "satoshi";
    let start = 0;
    let limit = 10;
    let pin = "1234"; // Define the PIN
    const sdk = await initSdk(username); // Initialize the SDK
    let mnemonic: string = (process.env.MNEMONIC as string);
    let password: string = (process.env.PASSWORD as string);

    await sdk.createNewUser(username); //Creating a new user 
    await sdk.initializeUser(username); // Initialize the user

    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic); // Initialize the wallet
    console.log("Wallet initialized!");

    let transactions = await sdk.getTransactionList(0, 10);  // Get the transaction list
    console.log("Transactions: " + JSON.stringify(transactions));
}

main();

