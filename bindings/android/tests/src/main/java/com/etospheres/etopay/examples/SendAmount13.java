package com.etospheres.etopay.examples;

import com.etospheres.etopay.ETOPaySdk;
import com.etospheres.etopay.model.Network;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.core.JsonProcessingException;
import java.util.List;
import com.etospheres.etopay.ETOPaySdk;

public class SendAmount13 {

    public static void main(String[] args) {
        // initialize the sdk
        ETOPaySdk sdk = utils.initSdk(utils.USERNAME_SATOSHI);

        String password = utils.getEnvVariable("PASSWORD");
        String mnemonic = utils.getEnvVariable("MNEMONIC");

        try {
            // create and init user
            sdk.createNewUser(utils.USERNAME_SATOSHI);
            sdk.initializeUser(utils.USERNAME_SATOSHI);
            System.out.println("Created and initialized new user.");

            // create new wallet
            sdk.setWalletPassword(utils.PIN, password);
            sdk.createWalletFromMnemonic(utils.PIN, mnemonic);
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

            // send amount
            String tx_id = sdk.sendAmount(utils.PIN, address.toString(), 1, "java bindings test".getBytes());
            System.out.println("send amount of 1 with transaction " + tx_id);

            // get new balance
            double new_balance = sdk.getWalletBalance(utils.PIN);
            System.out.println("new balance: " + new_balance);

            // print the details
            String details = sdk.getWalletTransaction(utils.PIN, tx_id);
            System.out.println("details: " + details);

        } catch (Exception e) {
            throw new RuntimeException("Send amount example failed", e);
        }
    }
}
