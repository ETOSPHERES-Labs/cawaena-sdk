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

        // Migrate wallet
        try await sdk.setWalletPassword(env.pin, env.password)
        let _ = try await sdk.createWalletFromMnemonic(env.pin, env.mnemonic)
        print("migrated wallet from mnemonic")

        // Fetch networks from backend
        let networks = try await sdk.getNetworks()
        try await sdk.setNetwork(networks[0].key())
        print("retrieved available networks and set the network for the wallet")

        // Generate address
        let address = try await sdk.generateNewAddress(env.pin)
        print("generated new receiver address: \(address.toString())")

        // Get wallet balance
        let balance = try await sdk.getWalletBalance(env.pin)
        print("balance: \(balance)")

        // Create purchase request
        let product_hash = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        let app_data =
            "{\"imageUrl\":\"https://httpbin.org/\",\"imageId\":\"a846ad10-fc69-4b22-b442-5dd740ace361\"}"
        let purchase_type = "CLIK"

        let purchase_id = try await sdk.createPurchaseRequest(
            "alice", 2, product_hash, app_data, purchase_type)
        print("Purchase Request created: \(purchase_id.toString())")

    } catch let error as RustString {
        fatalError("Create purchase request example failed: \(error.toString())")
    } catch {
        fatalError("Unexpected error occurred: \(error)")
    }

    group.leave()
}

group.wait()
