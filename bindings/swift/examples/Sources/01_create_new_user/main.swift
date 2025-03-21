// The Swift Programming Language
// https://docs.swift.org/swift-book

import ETOPaySdk
import Foundation
import utils

// We use a DispatchGroup to make the program wait until async functions finish before exiting.
// Not needed in long-running applications.
let group = DispatchGroup()
group.enter()

Task {
    do {
        // Get environment variables from the Utils module
        let env = getEnvironment()

        // Initialize SDK
        let sdk = try await initSdk(username: env.username, password: env.password)

        // Create new user
        try await sdk.createNewUser(env.username)
        print("created new user: \(env.username)")
        try await sdk.initUser(env.username)
        print("initialized new user: \(env.username)")

    } catch let error as RustString {
        fatalError("Create new user example failed: \(error.toString())")
    } catch {
        fatalError("Unexpected error occurred: \(error)")
    }

    group.leave()
}

group.wait()
