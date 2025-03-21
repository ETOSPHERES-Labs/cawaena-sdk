// The Swift Programming Language
// https://docs.swift.org/swift-book

// Onboard with viviswap example
// The user already exists in viviswap db. Therefore, the test will fail here.

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

        // Start KYC verification for viviswap
        let new_user = try await sdk.startKycVerificationForViviswap(
            "swift_example@gmail.com", true)
        print("New viviswap user: \(new_user)")

        // Get KYC status for viviswap
        let status = try await sdk.getKycDetailsForViviswap()
        print("Status: \(status)")

        // Update KYC status for viviswap
        var isIndividual: Bool? = true
        var isPep: Bool? = false
        var isUsCitizen: Bool? = false
        var isRegulatoryDisclosure: Bool? = true
        var countryOfResidence: String? = "DE"
        var nationality: String? = "DE"
        var fullName: String? = "fake fake"
        var dateOfBirth: String? = "2001-11-05"

        let details =
            try await sdk
            .updateKycPartiallyStatusForViviswap(
                isIndividual,
                isPep,
                isUsCitizen,
                isRegulatoryDisclosure,
                countryOfResidence,
                nationality,
                fullName,
                dateOfBirth
            )
        print("Details: \(details)")
        try await sdk.submitKycPartiallyStatusForViviswap()

        // Create a waiting loop that prints a dot every 5 seconds for 30 seconds
        print("Waiting for KYC verification to complete")
        for _ in 0..<12 {
            sleep(6)
            print(".")
            fflush(stdout)
            let kycDetails = try await sdk.getKycDetailsForViviswap()
            if kycDetails.verified_step == .Personal {
                break
            }
        }

        // Check that the user is verified
        let isVerified = try await sdk.isKycVerified(env.username)
        print("IsVerified: \(isVerified)")

    } catch let error as RustString {
        fatalError("Onboard user with viviswap example failed: \(error.toString())")
    } catch {
        fatalError("Unexpected error occurred: \(error)")
    }

    group.leave()
}

group.wait()
