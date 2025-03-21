package com.etospheres.etopay.examples;

import com.etospheres.etopay.ETOPaySdk;
import com.etospheres.etopay.model.Network;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.core.JsonProcessingException;
import java.util.List;
import com.etospheres.etopay.ETOPaySdk;

public class GetWalletTxList19 {

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

            // get wallet_tx_list
            String wallet_tx_list = sdk.getWalletTransactionList(utils.PIN, 0, 10);
            System.out.println("wallet tx list: " + wallet_tx_list);

        } catch (Exception e) {
            throw new RuntimeException("Get wallet tx list example failed", e);
        }
    }
}
