//  Need to do manual verification on postident: https://postident-itu.deutschepost.de/testapp
import * as wasm from "etopay-sdk-wasm";
import { initSdk } from './util';

async function main() {
    console.log("start");
    let username = "satoshi";
    const sdk = await initSdk();
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);

    // Start KYC verification for Postident
    let response = await sdk.startKycVerificationForPostident();
    console.log("Postident case id:", response.caseId);
    console.log("Postident case url:", response.caseUrl);

    // --> Do Postident KYC process with URL

    // Get KYC details for Postident
    let caseDetails = await sdk.getKycDetailsForPostident();
    console.log("Case details:", caseDetails);

    // Update KYC status for Postident
    await sdk.updateKycStatusForPostident(response.caseId);
    console.log("Case status updated.");

    // Check if KYC is verified
    let isVerified = await sdk.isKycVerified(username);
    console.log("IsVerified:", isVerified);

}

export { main }