import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk } from './utils';

// Send compliment example with sender `alice` and receiver `satoshi`

const sleep = (delay: number) => new Promise((resolve) => setTimeout(resolve, delay));
const timeoutMs = 3 * 60 * 1000;
const timeoutPromise = new Promise<never>((_, reject) =>
    setTimeout(() => reject(new Error("Timeout reached!")), timeoutMs)
);

async function main() {
    let username = "alice";
    let pin = "1234";

    // Initialize SDK
    const sdk = await initSdk(username);

    // Get env variables
    let mnemonic: string = (process.env.MNEMONIC_ALICE as string);
    let password: string = (process.env.PASSWORD as string);

    // Create new user and initialize it
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);


    // Create new wallet and initialize it
    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic);

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].id);

    // Generate new receiver address and fetch current balance
    let address = await sdk.generateNewAddress(pin);
    console.log("Address:", address);

    let balance = await sdk.getWalletBalance(pin);
    console.log("Balance:", balance);

    // Create new purchase request
    let product_hash = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
    let app_data = "wasm send compliment example";
    let purchase_type = "CLIK";

    let purchase_id = await sdk.createPurchaseRequest("satoshi", 2, product_hash, app_data, purchase_type);
    console.log("Purchase Request created: ", purchase_id);

    // Wait 3 min while transaction status becomes `Valid`
    console.log("Waiting until transaction status is `Valid`..");
    await Promise.race([
        (async () => {
            while (true) {
                await sleep(5000);

                let details = await sdk.getPurchaseDetails(purchase_id);
                console.log("Details: ", details);

                if (details.status == wasm.TxStatus.Valid) {
                    console.log("Purchase request valid, moving on...");
                    return;
                } else if (details.status == wasm.TxStatus.WaitingForVerification || details.status == wasm.TxStatus.Invalid) {
                    throw new Error("Purchase request invalid. Reason: " + details.invalid_reasons);
                }
            }
        })(),
        timeoutPromise
    ]);

    // When the transaction is Valid, confirm it
    await sdk.confirmPurchaseRequest(pin, purchase_id);

    // Wait 3 min while transaction status is `Complete`
    console.log("Waiting until transaction status is `Complete`..");
    await Promise.race([
        (async () => {
            while (true) {
                await sleep(5000);

                let details = await sdk.getPurchaseDetails(purchase_id);
                console.log("Details: ", details);

                if (details.status == wasm.TxStatus.Completed) {
                    console.log("Purchase request completed, done!");
                    return;
                } else if (details.status == wasm.TxStatus.Failed) {
                    throw new Error("Purchase request failed");
                }
            }
        })(),
        timeoutPromise
    ]);

    // Get new balance after sending the compliment
    let new_balance = await sdk.getWalletBalance(pin);
    console.log("New Balance:", new_balance);

    // Forcefully exit the process to ensure it completes.
    // The process may hang if there are pending operations or unresolved promises
    // This ensures that the script terminates properly without delays
    process.exit();
}

main();
