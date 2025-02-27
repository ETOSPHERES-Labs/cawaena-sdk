package com.cawaena.examples;

import com.cawaena.Wallet;

public class CreateCustomer17 {

    public static void main(String[] args) {

        // initialize the sdk
        Wallet sdk = utils.initSdk(utils.USERNAME_SATOSHI);

        try {
            // create and init user
            sdk.createNewUser(utils.USERNAME_SATOSHI);
            sdk.initializeUser(utils.USERNAME_SATOSHI);
            System.out.println("Created and initialized new user.");

            // Create sap customer if not exists
            try {
                sdk.customerGet();
                System.out.println("sap customer exists.");
            } catch (Exception e) {
                sdk.customerCreate("DE");
                System.out.println("created new sap customer");
            }

        } catch (Exception e) {
            throw new RuntimeException("Create new customer example failed", e);
        }
    }
}
