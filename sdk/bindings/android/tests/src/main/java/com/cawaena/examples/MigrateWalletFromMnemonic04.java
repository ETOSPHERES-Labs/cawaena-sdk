package com.cawaena.examples;

import com.cawaena.Wallet;
public class MigrateWalletFromMnemonic04 {

    public static void main(String[] args) {

        // initialize the sdk
        Wallet sdk = utils.initSdk(utils.USERNAME_SATOSHI);

        String password = utils.getEnvVariable("PASSWORD");
        String mnemonic = utils.getEnvVariable("MNEMONIC");

        try {
            // create and init user
            sdk.createNewUser(utils.USERNAME_SATOSHI);
            sdk.initializeUser(utils.USERNAME_SATOSHI);
            System.out.println("Created and initialized new user.");

            // create new wallet
            sdk.setWalletPassword(utils.PIN, password);

            // fetch networks from backend
            sdk.getNetworks();
            // set the network configuration for the wallet
            sdk.setNetwork(utils.IOTA_NETWORK_ID);

            sdk.createWalletFromMnemonic(utils.PIN, mnemonic);
            System.out.println("Created new wallet from mnemonic.");

        } catch (Exception e) {
            throw new RuntimeException("Migrate wallet from mnemonic example failed", e);
        }
    }
}
