package com.cawaena.examples;

import com.cawaena.Wallet;

public class CreateNewWallet03 {

    public static void main(String[] args) {

        // initialize the sdk
        Wallet sdk = utils.initSdk(utils.USERNAME_SATOSHI);

        String password = utils.getEnvVariable("PASSWORD");

        try {
            // create and init user
            sdk.createNewUser(utils.USERNAME_SATOSHI);
            sdk.initializeUser(utils.USERNAME_SATOSHI);
            System.out.println("Created and initialized new user.");

            // create new wallet
            sdk.setPassword(utils.PIN, password);
            sdk.createNewWallet(utils.PIN);
            System.out.println("Created new wallet.");

        } catch (Exception e) {
            throw new RuntimeException("Create new wallet example failed", e);
        }
    }
}
