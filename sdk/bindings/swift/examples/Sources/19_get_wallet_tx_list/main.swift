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

        // Create new wallet
        try await sdk.setWalletPassword(env.pin, env.password)
        let _ = try await sdk.createNewWallet(env.pin)
        print("created new wallet")

        // Fetch networks from backend
        let networks = try await sdk.getNetworks()
        try await sdk.setNetwork(networks[0].key())
        print("retrieved available networks and set the network for the wallet")

        // Get wallet tx list
        let tx_list = try await sdk.getWalletTransactionList(env.pin, 0, 10)
        // need to properly print the list
        print("Tx list: \(tx_list)")

    } catch let error as RustString {
        fatalError("Get wallet transaction list example failed: \(error.toString())")
    } catch {
        fatalError("Unexpected error occurred: \(error)")
    }

    group.leave()
}

group.wait()
