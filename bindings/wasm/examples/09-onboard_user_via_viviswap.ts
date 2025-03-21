import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk } from './utils';

async function main() {
    let username = "satoshi";
    const sdk = await initSdk(username);
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);

    let is_verified = await sdk.isKycVerified(username)

    if (is_verified) {
        console.log("user is verified");
        return;
    }

    // Start KYC verification for viviswap
    // The user already exists in viviswap db. Therefore, the test will fail here.

    let newUser = await sdk.startKycVerificationForViviswap("wasmtest@gmail.com", true);
    console.log(`New viviswap user: ${newUser}`);
}


main();


