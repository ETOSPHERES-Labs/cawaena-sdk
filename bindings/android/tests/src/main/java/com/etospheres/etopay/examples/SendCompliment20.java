package com.etospheres.etopay.examples;

import com.etospheres.etopay.ETOPaySdk;
import com.etospheres.etopay.model.Network;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.core.JsonProcessingException;
import java.util.List;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionException;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;
import com.etospheres.etopay.model.TxDetailsResponse;

public class SendCompliment20 {

    public static void main(String[] args) throws InterruptedException, ExecutionException, TimeoutException {

        // initialize the sdk
        ETOPaySdk sdk = utils.initSdk(utils.USERNAME_HANS48);

        String password = utils.getEnvVariable("PASSWORD");
        String mnemonic_hans48 = utils.getEnvVariable("MNEMONIC_HANS48");

        try {
            // create and init user
            sdk.createNewUser(utils.USERNAME_HANS48);
            sdk.initializeUser(utils.USERNAME_HANS48);
            System.out.println("Created and initialized new user.");

            // create new wallet
            sdk.setWalletPassword(utils.PIN, password);
            sdk.createWalletFromMnemonic(utils.PIN, mnemonic_hans48);

            System.out.println("Created and initialized new wallet from mnemonic.");

            // fetch networks from backend
            String networks = sdk.getNetworks();

            List<Network> networksList;
            try {
                ObjectMapper objectMapper = new ObjectMapper();
                networksList = objectMapper.readValue(networks, new TypeReference<List<Network>>() {
                });
            } catch (JsonProcessingException e) {
                throw new RuntimeException("Error processing JSON response", e);
            }

            Network iotaNetwork = networksList.get(0);
            // set the network configuration for the wallet
            sdk.setNetwork(iotaNetwork.id);

            // generate receiver address
            String address = sdk.generateNewAddress(utils.PIN);
            System.out.println("address: " + address);

            // get balance
            double balance = sdk.getWalletBalance(utils.PIN);
            System.out.println("balance: " + balance);

            // create purchase request
            String product_hash = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
            String app_data = "java example";
            String purchase_type = "CLIK";

            String purchase_id = sdk.purchaseRequestCreate("satoshi", 1, product_hash, app_data, purchase_type);
            System.out.println("purchase request id: " + purchase_id);

            CompletableFuture<Void> validFuture = CompletableFuture.runAsync(() -> {
                String valid = "Valid";
                String invalid = "Invalid";
                String waiting = "WaitingForVerification";
                try {
                    while (true) {
                        Thread.sleep(5 * 1000); // Simulate waiting
                        String purchase_details = null;
                        try {
                            purchase_details = sdk.purchaseDetails(purchase_id);
                        } catch (Exception e) {
                            throw new RuntimeException("Error fetching purchase details", e);
                        }
                        TxDetailsResponse details;
                        try {
                            details = new ObjectMapper().readValue(purchase_details, TxDetailsResponse.class);
                        } catch (JsonProcessingException e) {
                            throw new RuntimeException("Error processing JSON response", e);
                        }
                        System.out.println(" - Status: " + details.status);
                        if (details.status.equals(valid)) {
                            System.out.println("Purchase request valid, moving on...");
                            return;
                        } else if (details.status.equals(waiting)) {
                            throw new RuntimeException(
                                    "Purchase request invalid. Reason: " + details.invalid_reasons + " exiting!");
                        } else if (details.status.equals(invalid)) {
                            throw new RuntimeException(
                                    "Purchase request invalid. Reason: " + details.invalid_reasons + " exiting!");
                        }
                    }
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    throw new RuntimeException(e);
                }
            }).exceptionally(ex -> {
                handleException(ex);
                return null;
            });

            try {
                validFuture.get(3 * 60, TimeUnit.SECONDS);
            } catch (ExecutionException | TimeoutException e) {
                validFuture.cancel(true);
                throw new RuntimeException("Timeout reached while waiting for purchase request to become valid", e);
            }

            // Confirm purchase request
            sdk.purchaseRequestConfirm(utils.PIN, purchase_id);

            // Wait 3 min while tx status becomes completed
            CompletableFuture<Void> completedFuture = CompletableFuture.runAsync(() -> {
                String completed = "Completed";
                String failed = "Failed";
                try {
                    while (true) {
                        Thread.sleep(5 * 1000); // Simulate waiting
                        String purchase_details = null;
                        try {
                            purchase_details = sdk.purchaseDetails(purchase_id);
                        } catch (Exception e) {
                            throw new RuntimeException("Error fetching purchase details", e);
                        }
                        TxDetailsResponse details;
                        try {
                            details = new ObjectMapper().readValue(purchase_details, TxDetailsResponse.class);
                        } catch (JsonProcessingException e) {
                            throw new RuntimeException("Error processing JSON response", e);
                        }
                        System.out.println(" - Status: " + details.status);
                        if (details.status.equals(completed)) {
                            System.out.println("Purchase request completed, done!");
                            return;
                        } else if (details.status.equals(failed)) {
                            throw new RuntimeException("Purchase request failed");
                        }
                    }
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    throw new RuntimeException(e);
                }
            }).exceptionally(ex -> {
                handleException(ex);
                return null;
            });

            try {
                completedFuture.get(3 * 60, TimeUnit.SECONDS); // Wait for up to 3 minutes
            } catch (ExecutionException | TimeoutException e) {
                completedFuture.cancel(true);
                throw new RuntimeException("Timeout reached while waiting for purchase request to complete", e);
            }

            // Check new balance
            double new_balance = sdk.getWalletBalance(utils.PIN);
            System.out.println("new balance: " + new_balance);

        } catch (Exception e) {
            throw new RuntimeException("Send compliment example failed", e);
        }
    }

    private static Void handleException(Throwable ex) {
        if (ex instanceof CompletionException) {
            ex = ex.getCause();
        }
        if (ex instanceof RuntimeException) {
            throw (RuntimeException) ex;
        } else {
            throw new RuntimeException(ex);
        }
    }
}
