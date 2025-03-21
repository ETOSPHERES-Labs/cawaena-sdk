//  Need to do manual verification on postident: https://postident-itu.deutschepost.de/testapp
import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk } from './utils';

async function main() {
    let username = "satoshi";
    const sdk = await initSdk(username);
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);

    // Start KYC verification for Postident
    let response = await sdk.startKycVerificationForPostident();
    console.log("Postident case id:", response.case_id);
    console.log("Postident case url:", response.case_url);

    // --> Do Postident KYC process with URL

    // Get KYC details for Postident
    let caseDetails = await sdk.getKycDetailsForPostident();
    console.log("Case details:", caseDetails);

    // Update KYC status for Postident
    await sdk.updateKycStatusForPostident(response.case_id);
    console.log("Case status updated.");

    // Check if KYC is verified
    let isVerified = await sdk.isKycVerified(username);
    console.log("IsVerified:", isVerified);

}

main();
