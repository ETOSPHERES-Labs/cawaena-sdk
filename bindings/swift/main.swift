// we use this file as a `test` to verify that we have a connection established between Rust and Swift
// we can call our Rust bindings from this Swift file and test if we get the expected response
import Foundation

// Extend the converted Rust strings into Swift strings (RustString) to print them as error strings
extension RustString: @unchecked Sendable {}
extension RustString: Error {}

// We use a DispatchGroup to make the program wait until async functions finish before exiting.
// Not needed in long-running applications.
let group = DispatchGroup()
group.enter()

print("We're in Swift about to call our async Rust functions.")
Task {
    do {
        let sdk = ETOPaySdk()

        try await sdk.setConfig(
            """
            {
                "backend_url": "http://test.url.com/api",
                "storage_path": ".",
                "log_level": "info",
                "auth_provider": "standalone"
            }
            """)
        print("sdk configured")

    } catch let error as RustString {  // catch string errors that we return from Rust
        print(error.toString())

    } catch {  // catch other unexpected error types
        print("unexpected error: \(error)")
    }

    group.leave()
}

group.wait()
