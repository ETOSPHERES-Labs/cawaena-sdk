package com.cawaena.examples;

import com.cawaena.Wallet;import com.cawaena.model.Network;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.core.JsonProcessingException;
import java.util.List;
import com.cawaena.Wallet;

public class GetExchangeRate14 {

    public static void main(String[] args) {

        // initialize the sdk
        Wallet sdk = utils.initSdk(utils.USERNAME_SATOSHI);

        try {

            // create and init user
            sdk.createNewUser(utils.USERNAME_SATOSHI);
            sdk.initializeUser(utils.USERNAME_SATOSHI);
            System.out.println("Created and initialized new user.");
            
            // fetch networks from backend
            String networks = sdk.getNetworks();

            List<Network> networksList;
            try {
                ObjectMapper objectMapper = new ObjectMapper();
                networksList = objectMapper.readValue(networks, new TypeReference<List<Network>>() {});
            } catch (JsonProcessingException e) {
                throw new RuntimeException("Error processing JSON response", e);
            }

            Network iotaNetwork = networksList.get(0);
            // set the network configuration for the wallet
            sdk.setNetwork(iotaNetwork.key);

            // get exchange rate
            double exchange_rate = sdk.getExchangeRate();
            System.out.println("Exchange rate: " + exchange_rate);

        } catch (Exception e) {
            throw new RuntimeException("Get exchange rate example failed", e);
        }
    }
}
