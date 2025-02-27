package com.cawaena.examples;

import com.cawaena.Wallet;

public class CreateNewUser01 {

    public static void main(String[] args) {

        // Initialize SDK
        Wallet sdk = utils.initSdk(utils.USERNAME_SATOSHI);

        try {
            sdk.createNewUser(utils.USERNAME_SATOSHI);
            sdk.initializeUser(utils.USERNAME_SATOSHI);
            System.out.println("Created and initialized new user.");
        } catch (Exception e) {
            throw new RuntimeException("Create new user example failed", e);
        }
    }
}
