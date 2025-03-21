package com.etospheres.etopay.examples;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.github.cdimascio.dotenv.Dotenv;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.net.HttpURLConnection;
import java.net.URL;
import java.util.HashMap;
import java.util.Map;

import java.nio.file.Path;
import java.nio.file.FileSystems;
import java.nio.file.Files;

import com.etospheres.etopay.ETOPaySdk;

public class utils {
    public static String USERNAME_SATOSHI = "satoshi";
    public static String PIN = "1234";
    public static String NEW_PIN = "4321";
    public static String USERNAME_ARCHIVEME = "archiveme";
    public static String USERNAME_HANS48 = "hans48";

    public static ETOPaySdk initSdk(String username) {
        ETOPaySdk sdk = new ETOPaySdk();

        // config sdk
        try {
            // on CI we want to store the logs as artifacts, so we force the location
            Path directory;
            if (getEnvVariable("CI") != null) {
                Path path = FileSystems.getDefault().getPath("logs");
                Files.createDirectories(path);
                directory = Files.createTempDirectory(path, "etopay_examples");
            } else {
                directory = Files.createTempDirectory("etopay_examples");
            }

            System.out.println("Setting storage path to temporary directory: " + directory.toString());

            String url = getEnvVariable("EXAMPLES_BACKEND_URL");

            sdk.setConfig("""
                    {
                    	"backend_url": "%s",
                    	"storage_path": "%s",
                    	"log_level": "info",
                    	"auth_provider": "standalone"
                    }
                    """.formatted(url, directory.toString()));

            System.out.println("SDK environment set to development and validated.");

            // get the access token
            String access_token = generateAccessToken(username);
            sdk.refreshAccessToken(access_token);
            System.out.println("retrieved access token");
        } catch (Exception e) {
            throw new RuntimeException("Failed to initialize SDK", e);
        }

        return sdk;
    }

    // Enum with possible error cases which might happen during the generation of
    // the access token call.
    enum TokenError {
        MISSING_ENVIRONMENT_VARIABLE,
        INVALID_URL,
        PARSING_ERROR,
        ACCESS_TOKEN_NOT_FOUND
    }

    // Generate an access token by making a call to the KC API
    public static String generateAccessToken(String username) throws IOException {

        // get from env vars
        String kcURL = getEnvVariable("KC_URL");
        String kcRealm = getEnvVariable("KC_REALM");
        String clientId = getEnvVariable("KC_CLIENT_ID");
        String clientSecret = getEnvVariable("KC_CLIENT_SECRET");
        String password = getEnvVariable("PASSWORD");

        if (kcURL == null || kcRealm == null || clientId == null || clientSecret == null || password == null) {
            throw new RuntimeException(TokenError.MISSING_ENVIRONMENT_VARIABLE.name());
        }

        String urlString = kcURL + "/realms/" + kcRealm + "/protocol/openid-connect/token";
        @SuppressWarnings("deprecation")
        URL url = new URL(urlString);
        HttpURLConnection con = (HttpURLConnection) url.openConnection();
        con.setRequestMethod("POST");
        con.setRequestProperty("Content-Type", "application/x-www-form-urlencoded");

        // Construct body parameters
        Map<String, String> bodyParameters = new HashMap<>();
        bodyParameters.put("grant_type", "password");
        bodyParameters.put("scope", "profile email openid");
        bodyParameters.put("client_id", clientId);
        bodyParameters.put("client_secret", clientSecret);
        bodyParameters.put("username", username);
        bodyParameters.put("password", password);

        StringBuilder postData = new StringBuilder();
        for (Map.Entry<String, String> param : bodyParameters.entrySet()) {
            if (postData.length() != 0)
                postData.append('&');
            postData.append(param.getKey());
            postData.append('=');
            postData.append(param.getValue());
        }
        byte[] postDataBytes = postData.toString().getBytes("UTF-8");

        con.setDoOutput(true);
        con.getOutputStream().write(postDataBytes);

        // Read response
        int responseCode = con.getResponseCode();
        if (responseCode == HttpURLConnection.HTTP_OK) {
            // Parse JSON response using Jackson ObjectMapper
            InputStream inputStream = con.getInputStream();
            ObjectMapper mapper = new ObjectMapper();
            JsonNode jsonResponse = mapper.readTree(inputStream);

            // Check if access_token exists in JSON response
            if (jsonResponse.has("access_token")) {
                // System.out.println(jsonResponse.get("access_token").asText());
                return jsonResponse.get("access_token").asText();
            } else {
                throw new RuntimeException(TokenError.ACCESS_TOKEN_NOT_FOUND.name());
            }
        } else {
            throw new RuntimeException("Failed to get access token: " + responseCode);
        }
    }

    public static String getEnvVariable(String varName) {
        // Check if running in CI environment
        if (System.getenv("CI") != null) {
            // Use CI environment variables directly
            return System.getenv(varName);
        } else {
            // Load environment variables from .env file for local development
            Dotenv dotenv = Dotenv.configure().load();
            return dotenv.get(varName);
        }
    }
}
