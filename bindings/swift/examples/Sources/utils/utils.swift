import ETOPaySdk
import Foundation

// Struct to hold environment variables for examples
public struct Environment {
    public let username: String
    public let password: String
    public let pin: String
    public let mnemonic: String

    public init() {
        guard
            let username = ProcessInfo.processInfo.environment["USERNAME"],
            let password = ProcessInfo.processInfo.environment["PASSWORD"],
            let mnemonic = ProcessInfo.processInfo.environment["MNEMONIC"]
        else {
            fatalError("Missing environment variables")
        }

        self.username = username
        self.password = password
        self.pin = "1234"
        self.mnemonic = mnemonic
    }
}

// Helper function to access env variables
public func getEnvironment() -> Environment {
    return Environment()
}

//  Create sdk instance and set env
public func initSdk(username: String, password: String) async throws -> ETOPaySdk {
    // remove user and wallet generated files
    cleanup(atPaths: ["sdk-user.db", "wallets"])

    let url = ProcessInfo.processInfo.environment["EXAMPLES_BACKEND_URL"]!

    // initialize the etopay sdk
    let sdk = ETOPaySdk()

    // set the sdk config and validate it
    try await sdk.setConfig(
        """
        {
          "backend_url": "\(url)",
          "storage_path": ".",
          "log_level": "info",
          "auth_provider": "standalone"
        }
        """)

    // get the access token
    let access_token = try await generateAccessToken(username: username, password: password)
    try await sdk.refreshAccessToken(access_token)
    print("retrieved access token")

    return sdk
}

// Enum with possible error cases which might happen during the generation of the access token call.
enum TokenError: Error {
    case missingEnvironmentVariable(String)
    case invalidURL
    case parsingError(String)
    case accessTokenNotFound
}

// Struct to get the access token from the response
struct TokenResponse: Codable {
    let accessToken: String
}

// Generate an access token by making a call to the KC API. This is mirroring the `hello.http` endpoint
func generateAccessToken(username: String, password: String) async throws -> String {

    // access environment variables
    guard
        let kcURL = ProcessInfo.processInfo.environment["KC_URL"],
        let kcRealm = ProcessInfo.processInfo.environment["KC_REALM"],
        let clientId = ProcessInfo.processInfo.environment["KC_CLIENT_ID"],
        let clientSecret = ProcessInfo.processInfo.environment["KC_CLIENT_SECRET"]
    else {
        throw TokenError.missingEnvironmentVariable("One or more environment variables are missing")
    }

    let urlString = "\(kcURL)/realms/\(kcRealm)/protocol/openid-connect/token"
    guard let url = URL(string: urlString) else {
        throw TokenError.invalidURL
    }

    var request = URLRequest(url: url)
    request.httpMethod = "POST"
    request.setValue("application/x-www-form-urlencoded", forHTTPHeaderField: "Content-Type")

    // Construct body parameters
    let bodyParameters = [
        "grant_type": "password",
        "scope": "profile email openid",
        "client_id": clientId,
        "client_secret": clientSecret,
        "username": username,
        "password": password,
    ]
    request.httpBody =
        bodyParameters
        .map { "\($0.key)=\($0.value)" }
        .joined(separator: "&")
        .data(using: .utf8)

    let (data, _) = try await URLSession.shared.data(for: request)

    let jsonString = String(data: data, encoding: .utf8)
    guard let jsonData = jsonString?.data(using: .utf8) else {
        throw TokenError.parsingError("failed to parse json data from string")
    }

    do {
        // Parse the JSON data
        if let json = try JSONSerialization.jsonObject(with: jsonData, options: [])
            as? [String: Any],
            let accessToken = json["access_token"] as? String
        {
            return accessToken
        } else {
            throw TokenError.accessTokenNotFound
        }
    } catch _ {
        throw TokenError.parsingError("failed to serialize json data")
    }
}

// cleanup
func cleanup(atPaths paths: [String]) {
    let fileManager = FileManager.default

    for path in paths {
        do {
            // Check if the file or directory exists
            if fileManager.fileExists(atPath: path) {
                // Remove the file or directory
                try fileManager.removeItem(atPath: path)
            } else {
                print("File or directory does not exist: \(path). Moving on ..")
            }
        } catch {
            print("Error removing file or directory at \(path): \(error)")
        }
    }
}
