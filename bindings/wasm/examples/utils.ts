import * as wasm from "../pkg";
import * as dotenv from 'dotenv';
import axios from 'axios';
import { LocalStorage } from "node-localstorage";

export async function initSdk(username: string) {
    dotenv.config();

    // setup localStorage to use a file-based mock version
    globalThis.window = { localStorage: new LocalStorage('./local-storage') } as any;

    // make sure the localStorage is clear to run each example in isolation
    try {
        window.localStorage.clear();
    } catch (e) {
        console.log("Could not clear local storage: ", e);
    }

    console.log("Starting SDK initialization...");

    const sdk = new wasm.ETOPaySdk();

    // set the backend url if the environment variable is set
    let url: string = (process.env.EXAMPLES_BACKEND_URL as string);
    if (url == undefined) {
        throw new Error("EXAMPLES_BACKEND_URL environment variable must be present")
    }

    await sdk.setConfig(`
    {
        "backend_url": "${url}",
        "log_level": "info",
        "auth_provider": "standalone"
    }
    `);

    // Generate access token
    let access_token = await generateAccessToken(username);
    await sdk.refreshAccessToken(access_token);

    return sdk;
}

// Custom error class for handling token errors
class TokenError extends Error {
    constructor(message: string) {
        super(message);
        this.name = "TokenError";
    }

    static missingEnvironmentVariable(message: string) {
        return new TokenError(`Missing environment variable: ${message}`);
    }

    static invalidURL() {
        return new TokenError('Invalid URL');
    }

    static parsingError(message: string) {
        return new TokenError(`Parsing error: ${message}`);
    }

    static accessTokenNotFound() {
        return new TokenError('Access token not found');
    }
}

// Generate an access token by making a call to the KC API. This is mirroring the `hello.http` endpoint
async function generateAccessToken(username: string): Promise<string> {
    // Access environment variables
    const kcURL = process.env.KC_URL;
    const kcRealm = process.env.KC_REALM;
    const clientId = process.env.KC_CLIENT_ID;
    const clientSecret = process.env.KC_CLIENT_SECRET;
    const password = process.env.PASSWORD

    if (!kcURL || !kcRealm || !clientId || !clientSecret || !password) {
        throw TokenError.missingEnvironmentVariable('One or more environment variables are missing');
    }


    const urlString = `${kcURL}/realms/${kcRealm}/protocol/openid-connect/token`;

    let env_data = {
        grant_type: 'password',
        scope: 'profile email openid',
        client_id: clientId,
        client_secret: clientSecret,
        username: username,
        password: password
    };

    try {
        const response = await axios.post(urlString, env_data, {
            headers: { 'content-type': 'application/x-www-form-urlencoded' },

        });

        const data = response.data;
        if (data && data.access_token) {
            return data.access_token;
        } else {
            throw TokenError.accessTokenNotFound();
        }
    } catch (error: any) {
        if (error.response) {
            // Server responded with a status other than 2xx
            throw TokenError.parsingError(`Server responded with status ${error.response.status}: ${error.response.statusText}`);
        } else if (error.request) {
            // No response was received
            throw TokenError.invalidURL();
        } else {
            // Something happened in setting up the request
            throw TokenError.parsingError(error.message);
        }
    }
}
