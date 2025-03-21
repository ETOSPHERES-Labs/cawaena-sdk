import * as wasm from "etopay-sdk-wasm";
const sleep = (delay) => new Promise((resolve) => setTimeout(resolve, delay))

export { sleep }

// Function to initialize the SDK
export async function initSdk() {

    console.log("Starting SDK initialization...");

    const sdk = new wasm.ETOPaySdk(); // Create an instance of the SDK

    const url = process.env.EXAMPLES_BACKEND_URL;
    await sdk.setConfig(`
    {
        "backend_url": "${url}",
        "log_level": "info",
        "auth_provider": "standalone"
    }
    `);

    await sdk.refreshAccessToken(process.env.ACCESS_TOKEN);
    return sdk; // Return the initialized SDK instance
}
