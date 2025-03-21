import * as wasm from "etopay-sdk-wasm";
import { initSdk } from './util';

async function main() {
    let username = "satoshi";
    let password = "StrongP@55w0rd";
    let pin = "1234";

    const sdk = await initSdk();
    let mnemonic = process.env.MNEMONIC;

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);

    await sdk.createWalletFromMnemonic(pin, mnemonic);

    // Create wallet backup and delete it
    let backup_password = "backup_password";

    let backup = await sdk.createWalletBackup(pin, backup_password);
    await sdk.deleteWallet(pin)

    // Migrate wallet from backup
    await sdk.createWalletFromBackup(pin, backup, backup_password);

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].id);

    // use wallet
    let _address = await sdk.generateNewAddress(pin);
}

export { main }
