/**
    * Do manual postident verification at https://postident-itu.deutschepost.de/testapp
*/

package com.etospheres.etopay.examples;

import com.etospheres.etopay.ETOPaySdk;

public class OnboardUserPostident02 {

    public static void main(String[] args) {

        // Initialize SDK
        ETOPaySdk sdk = utils.initSdk(utils.USERNAME_SATOSHI);

        try {
            // create and init new user
            sdk.createNewUser(utils.USERNAME_SATOSHI);
            sdk.initializeUser(utils.USERNAME_SATOSHI);
            System.out.println("Created and initialized new user.");

            // check if user is kyc verified
            boolean is_verified = sdk.isKycVerified(utils.USERNAME_SATOSHI);
            if (is_verified == true) {
                System.out.println("User is already verified, please run the delete_user example first.");
                return;
            }

            // Start KYC verification for postident
            String new_case = sdk.startKycVerificationForPostident();
            System.out.println("New postident user with case: " + new_case);

            // Do manual postident verification at
            // https://postident-itu.deutschepost.de/testapp

            // Finish KYC verification for postident
            sdk.updateKycStatusForPostident("new case id");

            // Check that the user is verified
            boolean is_verified_after = sdk.isKycVerified(utils.USERNAME_SATOSHI);
            System.out.println("IsVerified: " + is_verified_after);

        } catch (

        Exception e) {
            throw new RuntimeException("Onboard user postident example failed", e);
        }
    }
}
