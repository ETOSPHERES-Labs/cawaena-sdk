package com.etospheres.etopay.examples;

import com.etospheres.etopay.ETOPaySdk;

public class DeleteUser18 {

    public static void main(String[] args) {

        // initialize the sdk
        ETOPaySdk sdk = utils.initSdk(utils.USERNAME_ARCHIVEME);

        String password = utils.getEnvVariable("PASSWORD");

        try {
            // create and init user
            sdk.createNewUser(utils.USERNAME_ARCHIVEME);
            sdk.initializeUser(utils.USERNAME_ARCHIVEME);
            System.out.println("Created and initialized new user.");

            // create and init new wallet
            sdk.setWalletPassword(utils.PIN, password);
            sdk.createNewWallet(utils.PIN);
            System.out.println("Created and initialized new wallet.");

            // Delete user and wallet
            sdk.deleteUser(utils.PIN);

            // check verification after deletion. Should be false
            boolean verified = sdk.isKycVerified(utils.USERNAME_ARCHIVEME);
            System.out.println("is verified: " + verified);

        } catch (Exception e) {
            throw new RuntimeException("Delete user example failed", e);
        }
    }
}
