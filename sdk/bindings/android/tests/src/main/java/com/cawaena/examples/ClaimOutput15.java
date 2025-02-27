package com.cawaena.examples;

import com.cawaena.Wallet;

public class ClaimOutput15 {

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
            System.out.println("Created and initialized new wallet.");

            // claim output
            sdk.claimOutputs(utils.PIN);
            System.out.println("claim outputs success");

        } catch (Exception e) {
            throw new RuntimeException("Claim outputs example failed", e);
        }
    }
}
