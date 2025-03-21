package com.etospheres.etopay.examples;

import com.etospheres.etopay.ETOPaySdk;

public class VerifyPin10 {

    public static void main(String[] args) {

        // initialize the sdk
        ETOPaySdk sdk = utils.initSdk(utils.USERNAME_SATOSHI);

        String password = utils.getEnvVariable("PASSWORD");

        try {
            // create and init user
            sdk.createNewUser(utils.USERNAME_SATOSHI);
            sdk.initializeUser(utils.USERNAME_SATOSHI);
            System.out.println("Created and initialized new user.");

            // create new wallet
            sdk.setWalletPassword(utils.PIN, password);
            sdk.createNewWallet(utils.PIN);
            System.out.println("Created and initialized new wallet.");

            // verify pin
            sdk.pinVerify(utils.PIN);
            System.out.println("Pin verified");

        } catch (Exception e) {
            throw new RuntimeException("Verify pin example failed", e);
        }
    }
}
