package com.cawaena.examples;

import com.cawaena.Wallet;
public class GetWalletTxList19 {

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

            // get wallet_tx_list
            String wallet_tx_list = sdk.getWalletTransactionList(utils.PIN, 0, 10);
            System.out.println("wallet tx list: " + wallet_tx_list);

        } catch (Exception e) {
            throw new RuntimeException("Get wallet tx list example failed", e);
        }
    }
}
