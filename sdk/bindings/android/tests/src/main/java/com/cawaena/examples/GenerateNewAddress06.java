package com.cawaena.examples;

import com.cawaena.Wallet;import com.cawaena.model.Network;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.core.JsonProcessingException;
import java.util.List;
import com.cawaena.Wallet;
public class GenerateNewAddress06 {

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
            sdk.setWalletPassword(utils.PIN, password);
            sdk.createWalletFromMnemonic(utils.PIN, mnemonic);
            System.out.println("Created and initialized new wallet from mnemonic.");

            
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

            // generate receiver address
            String address = sdk.generateNewAddress(utils.PIN);
            System.out.println("address: " + address);

        } catch (Exception e) {
            throw new RuntimeException("Generate new address example failed", e);
        }
    }
}
