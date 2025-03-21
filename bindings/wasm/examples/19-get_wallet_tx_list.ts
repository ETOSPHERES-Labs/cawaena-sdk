import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk } from './utils';


async function main() {
    let username = "satoshi";
    let pin = "1234"; // Define the PIN

    const sdk = await initSdk(username); // Initialize the SDK
    let mnemonic: string = (process.env.MNEMONIC as string);
    let password: string = (process.env.PASSWORD as string);

    await sdk.createNewUser(username); //Creating a new user 
    await sdk.initializeUser(username); // Initialize the user
    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic); // Initialize the wallet
    console.log("Wallet initialized!");

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].id);

    let transactions = await sdk.getWalletTransactionList(pin, 0, 10);  // Get the transaction list
    console.log("Wallet transactions list: " + JSON.stringify(transactions));
}

main();
