package com.cawaena.examples;

import com.cawaena.Wallet;

public class ChangePassword12 {

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

            // change password
            sdk.setPassword(utils.PIN, "StrongP@ssw0rd");
            System.out.println("Password changed");

        } catch (Exception e) {
            throw new RuntimeException("Change password example failed", e);
        }
    }
}
