package com.etospheres.etopay.examples;

import com.etospheres.etopay.ETOPaySdk;

public class GetTxList16 {

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

            // get tx list
            String tx_list = sdk.txList(0, 10);
            System.out.println("tx list: " + tx_list);

        } catch (Exception e) {
            throw new RuntimeException("Get tx list example failed", e);
        }
    }
}
