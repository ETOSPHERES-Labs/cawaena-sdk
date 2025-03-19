import { initSdk } from './util';
import * as wasm from "etopay-sdk-wasm";

async function main() {
    let username = "satoshi";
    let pin = "1234";
    let password = "StrongP@55w0rd";
    let mnemonic = process.env.MNEMONIC;

    const sdk = await initSdk();
    await sdk.createNewUser(username);
    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic);

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].key);

    let recipient_address = await sdk.generateNewAddress(pin);

    console.log("address", recipient_address);

    let balance_before = await sdk.getWalletBalance(pin);
    console.log("balance before sending amount : ", balance_before);

    let tx_id = await sdk.sendAmount(pin, recipient_address, 1.0);
    console.log("sent amount with transaction", tx_id);

    let balance_after = await sdk.getWalletBalance(pin);
    console.log("balance after sending amount : ", balance_after);
}

export { main }


