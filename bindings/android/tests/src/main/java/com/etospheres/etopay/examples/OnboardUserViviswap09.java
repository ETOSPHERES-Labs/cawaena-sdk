/**
    * This example will not run until the end because the user already exists in Viviswap db and it will not create a new one.
*/

package com.etospheres.etopay.examples;

import com.etospheres.etopay.ETOPaySdk;

public class OnboardUserViviswap09 {

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

            // Start KYC verification for viviswap
            // The user already exists in viviswap db. Therefore, the test will fail here.
            String new_user = sdk
                    .startViviswapKyc("javaexamples@gmail.com", true);
            System.out.println("New Viviswap user: " + new_user);

            // Get KYC status for viviswap
            String details = sdk.getViviswapKyc();
            System.out.println("Viviswap KYC details: " + details);

        } catch (

        Exception e) {
            throw new RuntimeException("Onboard user viviswap example failed", e);
        }
    }
}
