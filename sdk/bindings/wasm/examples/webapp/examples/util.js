import * as wasm from "cryptpay-sdk-wasm";
const sleep = (delay) => new Promise((resolve) => setTimeout(resolve, delay))

export { sleep }

// Function to initialize the SDK
export async function initSdk() {

    console.log("Starting SDK initialization...");

    const sdk = new wasm.CryptpaySdk(); // Create an instance of the SDK

    const url = process.env.EXAMPLES_BACKEND_URL;
    await sdk.setConfig(`
    {
        "backend_url": "${url}",
        "log_level": "info",
        "auth_provider": "standalone",
        "node_urls": {
            "iota": ["https://api.testnet.iotaledger.net"]
        }
    }
    `);
    await sdk.setCurrency(wasm.Currency.Iota);
    await sdk.refreshAccessToken(process.env.ACCESS_TOKEN);
    return sdk; // Return the initialized SDK instance
}
