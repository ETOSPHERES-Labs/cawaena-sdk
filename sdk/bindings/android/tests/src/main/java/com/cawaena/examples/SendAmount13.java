package com.cawaena.examples;

import com.cawaena.Wallet;

public class SendAmount13 {

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
            sdk.setPassword(utils.PIN, password);
            sdk.createWalletFromMnemonic(utils.PIN, mnemonic);
            System.out.println("Created and initialized new wallet from mnemonic.");

            // generate receiver address
            String address = sdk.generateNewAddress(utils.PIN);
            System.out.println("address: " + address);

            // get balance
            double balance = sdk.getWalletBalance(utils.PIN);
            System.out.println("balance: " + balance);

            // send amount
            sdk.sendAmount(utils.PIN, address.toString(), 1, null, null, "java bindings test");
            System.out.println("send amount of 1");

            // get new balance
            double new_balance = sdk.getWalletBalance(utils.PIN);
            System.out.println("new balance: " + new_balance);

        } catch (Exception e) {
            throw new RuntimeException("Send amount example failed", e);
        }
    }
}
