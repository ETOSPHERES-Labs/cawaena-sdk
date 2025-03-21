// The Swift Programming Language
// https://docs.swift.org/swift-book

import Foundation
import ETOPaySdk
import utils

// We use a DispatchGroup to make the program wait until async functions finish before exiting.
// Not needed in long-running applications.
let group = DispatchGroup()
group.enter()

Task {
    do {
        // Get environment variables from the Utils module
        let env = getEnvironment()
        let new_pin = "4321";

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
        
        // Reset pin
        try await sdk.resetPin(env.pin, new_pin)
        print("pin reseted")
        
        // Verify new pin
        try await sdk.verifyPin(new_pin)
        print("new pin verified")                     

    } catch let error as RustString  {
        fatalError("Reset pin example failed: \(error.toString())")
    }  catch {
        fatalError("Unexpected error occurred: \(error)")
    }
    
    group.leave()
}

group.wait()
