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
        let username_alice = ProcessInfo.processInfo.environment["ALICE"]!
        let mnemonic_alice = ProcessInfo.processInfo.environment["MNEMONIC_ALICE"]!

        // Initialize SDK
        let sdk = try await initSdk(username: username_alice, password: env.password)

        // Create new user
        try await sdk.createNewUser(username_alice)
        print("created new user: \(username_alice)")
        try await sdk.initUser(username_alice)
        print("initialized new user: \(username_alice)")

        // Migrate wallet
        try await sdk.setWalletPassword(env.pin, env.password)
        let _ = try await sdk.createWalletFromMnemonic(env.pin, mnemonic_alice)
        print("migrated wallet from mnemonic")

        // Fetch networks from backend
        let networks = try await sdk.getNetworks()
        try await sdk.setNetwork(networks[0].id())
        print("retrieved available networks and set the network for the wallet")

        // Generate address
        let address = try await sdk.generateNewAddress(env.pin)
        print("generated new receiver address: \(address.toString())")

        let balance = try await sdk.getWalletBalance(env.pin)
        print("balance: \(balance)")

        // Create purchase request
        let product_hash = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        let app_data = "swift example"
        let purchase_type = "CLIK"

        let purchase_id = try await sdk.createPurchaseRequest(
            "satoshi", 3, product_hash, app_data, purchase_type)
        print("Purchase Request created: \(purchase_id.toString())")

        // Wait 3 min while tx status becomes valid
        let timeoutDate1 = Date().addingTimeInterval(3 * 60)
        test: while Date() < timeoutDate1 {
            try await Task.sleep(nanoseconds: 5 * 1_000_000_000)
            let details = try await sdk.getPurchaseDetails(purchase_id.toString())
            let status = details.status
            print(" - Status: \(status)")
            switch status {
            case .Valid:
                print("Purchase request valid, moving on...")
                break test
            case .WaitingForVerification:
                fatalError("Purchase request invalid! Reason: \(details.invalid_reasons). Exiting")
            case .Invalid:
                fatalError("Purchase request invalid! Reason: \(details.invalid_reasons). Exiting")
            default:
                continue
            }
        }
        if Date() >= timeoutDate1 {
            fatalError("Timeout reached while waiting for purchase request to become valid")
        }

        // Step 4: Confirm purchase request (perform actual wallet transaction)
        try await sdk.confirmPurchaseRequest(env.pin, purchase_id.toString())

        // Wait 3 min while tx status becomes completed
        let timeoutDate2 = Date().addingTimeInterval(3 * 60)
        test2: while Date() < timeoutDate2 {
            try await Task.sleep(nanoseconds: 5 * 1_000_000_000)
            let status = try await sdk.getPurchaseDetails(purchase_id.toString()).status
            print(" - Status: \(status)")
            if status == .Completed {
                print("Purchase request completed, done!")
                break test2
            } else if status == .Failed {
                fatalError("Purchase request failed")
            }
        }
        if Date() >= timeoutDate2 {
            fatalError("Timeout reached while waiting for purchase request to complete")
        }

        // Check new balance
        let new_balance = try await sdk.getWalletBalance(env.pin)
        print("New Balance: \(new_balance)")

    } catch let error as RustString {
        fatalError("Send compliment example failed: \(error.toString())")
    } catch {
        fatalError("Unexpected error occurred: \(error)")
    }

    group.leave()
}

group.wait()
