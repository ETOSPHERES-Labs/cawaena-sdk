package com.etospheres.etopay.examples;

import com.etospheres.etopay.ETOPaySdk;

public class ResetPin11 {

    public static void main(String[] args) {

        // initialize the sdk
        ETOPaySdk sdk = utils.initSdk(utils.USERNAME_SATOSHI);

        String password = utils.getEnvVariable("PASSWORD");

        try {
            // create and init user
            sdk.createNewUser(utils.USERNAME_SATOSHI);
            sdk.initializeUser(utils.USERNAME_SATOSHI);
            System.out.println("Created and initialized new user.");

            // create and init new wallet
            sdk.setWalletPassword(utils.PIN, password);
            sdk.createNewWallet(utils.PIN);
            System.out.println("Created and init new wallet.");

            // reset pin
            sdk.pinReset(utils.PIN, utils.NEW_PIN);

            // verify new pin
            sdk.pinVerify(utils.NEW_PIN);
            System.out.println("New pin verified");

        } catch (Exception e) {
            throw new RuntimeException("Reset pin example failed", e);
        }
    }
}
