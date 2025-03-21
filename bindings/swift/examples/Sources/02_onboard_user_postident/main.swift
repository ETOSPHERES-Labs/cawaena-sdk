// The Swift Programming Language
// https://docs.swift.org/swift-book

// ! Need to do manual verification on postident: https://postident-itu.deutschepost.de/testapp

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

        // Exit if user is already verified
        let is_verified = try await sdk.isKycVerified(env.username)
        print("is verified: \(is_verified)")
        if is_verified {
            print("User is already verified. No need to delete. Exiting")
            return
        }

        // Start KYC verification for postident
        let new_user = try await sdk.initKycVerificationForPostident()
        print("New postident user: \(new_user.case_id.toString()), \(new_user.case_url.toString())")

        // Do manual postident verification at https://postident-itu.deutschepost.de/testapp

        // Finish KYC verification for postident
        try await sdk.updateKycDetailsForPostident(new_user.case_id)

        // Check that the user is verified.
        // Should be true if the manual verification in postident is done.
        // Here it will return false.
        let verified = try await sdk.isKycVerified(env.username)
        print("Is Verified: \(verified)")

    } catch let error as RustString {
        fatalError("Onboard user with postident example failed: \(error.toString())")
    } catch {
        fatalError("Unexpected error occurred: \(error)")
    }

    group.leave()
}

group.wait()
