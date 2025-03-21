package com.etospheres.etopay.examples;

import com.etospheres.etopay.ETOPaySdk;

public class MigrateWalletFromBackup05 {

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

            // create backup
            byte[] backup_bytes = sdk.createWalletBackup(utils.PIN, "backup_password");

            // delete existing wallet
            sdk.deleteWallet(utils.PIN);
            System.out.println("deleted existing wallet");

            // migrate wallet from backup
            sdk.createWalletFromBackup(utils.PIN, backup_bytes, "backup_password");
            System.out.println("wallet restored from backup");

        } catch (Exception e) {
            throw new RuntimeException("Migrate wallet from backup example failed", e);
        }
    }
}
