package com.etospheres.etopay;

import org.junit.Test;
import org.junit.BeforeClass;

import org.junit.AfterClass;
import org.junit.Ignore;
import org.junit.runner.OrderWith;
import org.junit.runner.manipulation.Alphanumeric;
import static org.junit.Assert.*;
import static com.github.tomakehurst.wiremock.client.WireMock.*;
import com.github.tomakehurst.wiremock.junit.WireMockRule;
import org.junit.Rule;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import java.io.Reader;
import java.io.StringReader;

import java.nio.file.Path;
import java.nio.file.Files;

import com.etospheres.etopay.model.CaseDetailsResponse;
import com.etospheres.etopay.model.CaseIdResponse;
import com.etospheres.etopay.model.NewViviswapUser;
import com.etospheres.etopay.model.TransactionStatusResponse;
import com.etospheres.etopay.model.TxDetailsResponse;
import com.etospheres.etopay.model.ViviswapKycStatus;
import com.etospheres.etopay.model.ViviswapPartiallyKycDetails;

@OrderWith(Alphanumeric.class)
public class SdkUnitTest {
    private static ETOPaySdk sdk;
    private static final Logger logger = LoggerFactory.getLogger(SdkUnitTest.class);
    private static final String token = "access_token";

    private static final String USERNAME = "satoshi";
    private static final String PIN = "1234";
    private static final String PASSWORD = "StrongP@55word";
    private static final String BACKUP_PASSWORD = "BackupStrongP@55word";
    private static final String AUTH_PROVIDER = "standalone";
    private static final String CURRENCY = "Iota";
    private static final String EMAIL = "useremail@gmail.com";
    private static final String TOKEN_HEADER_VALUE = String.format("Bearer %s", token);
    private static final String CASE_ID = "123456789ABC";
    private static final String PURCHASE_ID = "3be5b0c8-e292-4957-bc70-923e13d01423";
    private static final String IOTA_NETWORK_ID = "67a1f08edf55756bae21e7eb";

    private double initBalance;
    private static byte[] backupBytes;
    private static String mnemonic;
    private String receiverAddr;

    @Rule
    public WireMockRule wireMockRule = new WireMockRule(1080);

    @BeforeClass
    public static void setUpClass() {
        sdk = new ETOPaySdk();

        try {
            // create a random directory for the tests to use
            Path directory = Files.createTempDirectory("etopay_tests");

            sdk.setConfig("""
                    {
                        "backend_url": "http://localhost:1080/api",
                        "storage_path": "%s",
                        "log_level": "debug",
                        "auth_provider": "standalone"
                    }
                    """.formatted(directory.toString()));
            sdk.refreshAccessToken(token);
        } catch (Exception e) {
            fail(e.getMessage());
        }

    }

    @AfterClass
    public static void tearDownClass() {
        try {
            // we cannot delete wallet since we cannot mock the API requests here, so we
            // first remove the access token
            sdk.refreshAccessToken("");
            sdk.deleteWallet(PIN);
            sdk.close();
        } catch (Exception e) {
            fail(e.getMessage());
        }
    }

    @Test
    public void AshouldCreateNewUser() throws Exception {
        sdk.createNewUser(USERNAME);
        logger.debug(String.format("User %s created", USERNAME));
    }

    @Test
    public void BAshouldInitializeUser() throws Exception {
        final String body = "{" + System.lineSeparator() +
                String.format("\"username\":\"%s\",", USERNAME)
                + System.lineSeparator() + "\"is_verified\":false"
                + System.lineSeparator() + "}";

        wireMockRule.stubFor(get(urlPathEqualTo("/api/kyc/check-status"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)
                        .withHeader("Content-Type", "application/json")
                        .withBody(body)));
        sdk.initializeUser(USERNAME);
    }

    @Test
    public void BBshouldGetNetworks() throws Exception {
        String body = "{\"networks\":[{\"id\":\"67a1f08edf55756bae21e7eb\",\"name\":\"IOTA\",\"currency\":\"IOTA\",\"block_explorer_url\":\"https://explorer.shimmer.network/testnet/\",\"enabled\":true,\"network_identifier\":\"iota_mainnet\",\"network_type\":{\"Stardust\":{\"node_urls\":[\"https://api.testnet.iotaledger.net\"]}}},{\"id\":\"67a2080ddf55756bae21e7f5\",\"name\":\"Eth Sepolia\",\"currency\":\"ETH\",\"block_explorer_url\":\"https://sepolia.explorer.mode.network\",\"enabled\":true,\"network_identifier\":\"ethereum_mainnet\",\"network_type\":{\"Evm\":{\"node_urls\":[\"https://sepolia.mode.network\"],\"chain_id\":31337}}}]}";

        wireMockRule.stubFor(get(urlPathEqualTo("/api/config/networks"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)
                        .withHeader("Content-Type", "application/json")
                        .withBody(body)));
        sdk.getNetworks();
        logger.debug(String.format("Get networks"));
    }

    @Test
    public void BCshouldSetNetwork() throws Exception {
        sdk.setNetwork(IOTA_NETWORK_ID);
        logger.debug(String.format("Set network %s", IOTA_NETWORK_ID));
    }

    @Test
    public void CshouldCreateNewWallet() throws Exception {
        final String share_body_request = "{" + "\"share\":\"${json-unit.any-string}\"" + "}";

        wireMockRule.stubFor(put(urlPathEqualTo("/api/user/shares/backup"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .withRequestBody(equalToJson(share_body_request))
                .willReturn(aResponse()
                        .withStatus(200)));

        wireMockRule.stubFor(put(urlPathEqualTo("/api/user/shares/recovery"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .withRequestBody(equalToJson(share_body_request))
                .willReturn(aResponse()
                        .withStatus(200)));

        sdk.setWalletPassword(PIN, PASSWORD);

        this.mnemonic = sdk.createNewWallet(PIN);
        logger.debug(String.format("Wallet created. Mnemonic: %s", this.mnemonic));
    }

    @Ignore("No healthy node available")
    @Test
    public void EshouldGenerateReceiverAddress() throws Exception {
        final String body = "{" + "\"address\":\"${json-unit.any-string}\"" + "}";
        wireMockRule.stubFor(put(urlPathEqualTo("/api/user/address"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .withQueryParam("network_id", equalTo(IOTA_NETWORK_ID))
                .withRequestBody(equalToJson(body))
                .willReturn(aResponse()
                        .withStatus(200)
                        .withHeader("Content-Type", "application/json")
                        .withBody(body)));
        this.receiverAddr = sdk.generateNewAddress(PIN);
        logger.debug(String.format("Generated address %s", this.receiverAddr));
    }

    @Test
    public void FshouldCheckKycStatus() throws Exception {
        String body = "{" + System.lineSeparator() + "\"username\":\"satoshi\","
                + System.lineSeparator() + "\"is_verified\":false"
                + System.lineSeparator() + "}";

        wireMockRule.stubFor(get(urlPathEqualTo("/api/kyc/check-status"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)
                        .withHeader("Content-Type", "application/json")
                        .withBody(body)));

        var isVerified = sdk.isKycVerified(USERNAME);
        assertEquals(isVerified, false);
    }

    @Test
    public void GshouldStartKycVerificationPostident() throws Exception {
        final String caseUrl = "https://postident.deutschepost.de/user/start/?caseId=" + this.CASE_ID;
        final String body = "{" + System.lineSeparator() +
                String.format("\"case_id\":\"%s\",", this.CASE_ID) + System.lineSeparator() +
                String.format("\"case_url\":\"%s\"", caseUrl) + System.lineSeparator() + "}";

        wireMockRule.stubFor(get(urlPathEqualTo("/api/postident/get-new-case-id"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)
                        .withHeader("Content-Type", "application/json")
                        .withBody(body)));

        String json = sdk.startKycVerificationForPostident();
        logger.info("started" + json);
        Reader reader = new StringReader(json);
        CaseIdResponse caseResponse = new ObjectMapper().readValue(reader, CaseIdResponse.class);
        assertEquals(caseResponse.caseId, "123456789ABC");
        assertEquals(caseResponse.caseUrl, caseUrl);
    }

    @Test
    public void HshouldGetKycVerificationPostident() throws Exception {
        final String body = "{" + System.lineSeparator() +
                String.format("\"case_id\":\"%s\",", this.CASE_ID) + System.lineSeparator() +
                "\"archived\":false," + System.lineSeparator() +
                "\"status\":\"COMPLETED\"" + "}";

        wireMockRule.stubFor(get(urlPathEqualTo("/api/postident/get-case-details"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)
                        .withHeader("Content-Type", "application/json")
                        .withBody(body)));

        ObjectMapper mapper = new ObjectMapper();
        String json = sdk.getKycDetailsForPostident();
        CaseDetailsResponse details = mapper.readValue(json,
                CaseDetailsResponse.class);
        assertEquals(details.caseId, this.CASE_ID);
        assertEquals(details.archived, false);
        assertEquals(details.status, "COMPLETED");
    }

    @Test
    public void IshouldUpdateKycPostident() throws Exception {
        wireMockRule.stubFor(post(urlEqualTo("/api/postident/update-case-status"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse().withStatus(202)));
        sdk.updateKycStatusForPostident(this.CASE_ID);
    }

    @Test
    public void JshouldCreateWalletFromMnemonic() throws Exception {
        final String share_body_request = "{" + "\"share\":\"${json-unit.any-string}\"" + "}";

        wireMockRule.stubFor(put(urlPathEqualTo("/api/user/shares/backup"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .withRequestBody(equalToJson(share_body_request))
                .willReturn(aResponse()
                        .withStatus(200)));

        wireMockRule.stubFor(put(urlPathEqualTo("/api/user/shares/recovery"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .withRequestBody(equalToJson(share_body_request))
                .willReturn(aResponse()
                        .withStatus(200)));

        sdk.createWalletFromMnemonic(PIN, this.mnemonic);
        logger.info("Wallet created from mnemonic");
    }

    @Test
    public void KshouldCreateWalletBackup() throws Exception {
        this.backupBytes = sdk.createWalletBackup(PIN, BACKUP_PASSWORD);
        logger.info(String.format("Wallet backup with %d bytes", this.backupBytes.length));
    }

    @Test
    public void LshouldCreateWalletFromBackup() throws Exception {
        final String share_body_request = "{" + "\"share\":\"${json-unit.any-string}\"" + "}";

        wireMockRule.stubFor(put(urlPathEqualTo("/api/user/shares/backup"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .withRequestBody(equalToJson(share_body_request))
                .willReturn(aResponse()
                        .withStatus(200)));

        wireMockRule.stubFor(put(urlPathEqualTo("/api/user/shares/recovery"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .withRequestBody(equalToJson(share_body_request))
                .willReturn(aResponse()
                        .withStatus(200)));

        sdk.createWalletFromBackup(PIN, this.backupBytes, BACKUP_PASSWORD);
        logger.info("Wallet created using backup");
    }

    @Test
    public void NshouldVerifyMnemonic() throws Exception {
        sdk.verifyMnemonic(PIN, this.mnemonic);
        logger.info("Mnemonic verified");
    }

    @Ignore("Deleting would cause problems with other tests")
    @Test
    public void OshouldDeleteWallet() throws Exception {
        wireMockRule.stubFor(delete(urlPathEqualTo("/api/user/shares"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)));
        sdk.deleteWallet(PIN);
        logger.info("Wallet deleted");
    }

    @Test
    public void PshouldVerifyPin() throws Exception {
        sdk.pinVerify(PIN);
        logger.info("Pin verified");
    }

    @Test
    public void QshouldResetPin() throws Exception {
        sdk.pinReset(PIN, PIN);
        logger.info("Pin reseted");
    }

    @Test
    public void RshouldChangePassword() throws Exception {
        final String share_body_request = "{" + "\"share\":\"${json-unit.any-string}\"" + "}";

        wireMockRule.stubFor(put(urlPathEqualTo("/api/user/shares/backup"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .withRequestBody(equalToJson(share_body_request))
                .willReturn(aResponse()
                        .withStatus(200)));

        wireMockRule.stubFor(put(urlPathEqualTo("/api/user/shares/recovery"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .withRequestBody(equalToJson(share_body_request))
                .willReturn(aResponse()
                        .withStatus(200)));

        sdk.setWalletPassword(PIN, PASSWORD);
        logger.info("Password changed");
    }

    @Ignore("No healthy node available")
    @Test
    public void ShouldCreatePurchaseRequest() throws Exception {
        this.initBalance = sdk.getWalletBalance(PIN);
        double amount = 50000.5;
        final String reqBody = "{" + System.lineSeparator()
                + String.format("\"amount\":\"%d\",", amount) + System.lineSeparator()
                + String.format("\"network_id\":\"%s\",", IOTA_NETWORK_ID) + System.lineSeparator()
                + String.format("\"receiver\":\"%s\"", receiverAddr) + System.lineSeparator()
                + "}";
        final String resBody = "{" + String.format("\"index\":\"%s\"",
                this.PURCHASE_ID) + "}";

        wireMockRule.stubFor(post(urlEqualTo("/api/transactions/create"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .withQueryParam("option", equalTo("C2C"))
                .withRequestBody(equalToJson(reqBody))
                .willReturn(aResponse()
                        .withStatus(201)
                        .withHeader("Content-Type", "application/json")
                        .withBody(resBody)));

        String purchaseId = sdk.purchaseRequestCreate(receiverAddr, amount, "product_hash", "app_data",
                "purchase_type");
        assertEquals(this.PURCHASE_ID, purchaseId);
        logger.info(String.format("Purchase request created. Id: %s", purchaseId));
    }

    @Test
    public void UshouldReturnPurchaseDetails() throws Exception {
        String mainAddress = "rms1qz8jdgvrerzv35s43pkdkawdr9x4t6xfnhcrt5tlgsyltgpwyx9ks4c5kct";
        double amount = 5.5;
        String status = "Pending";
        String network = "{\"id\":\"67a1f08edf55756bae21e7eb\",\"name\":\"IOTA\",\"currency\":\"IOTA\",\"block_explorer_url\":\"https://explorer.shimmer.network/testnet/\",\"enabled\":true,\"network_identifier\":\"iota_mainnet\",\"network_type\":{\"Stardust\":{\"node_urls\":[\"https://api.testnet.iotaledger.net\"]}}}";

        final String body = "{" + System.lineSeparator()
                + String.format("\"system_address\":\"%s\",", mainAddress) +
                System.lineSeparator()
                + String.format("\"amount\":\"%f\",", amount) + System.lineSeparator()
                + String.format("\"network\":%s,", network) + System.lineSeparator()
                + String.format("\"status\":\"%s\",", status) + System.lineSeparator()
                + String.format("\"invalid_reasons\": []", status) + System.lineSeparator()
                + "}";

        wireMockRule.stubFor(get(urlPathEqualTo("/api/transactions/details"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .withQueryParam("index", equalTo(this.PURCHASE_ID))
                .willReturn(aResponse()
                        .withStatus(200)
                        .withHeader("Content-Type", "application/json")
                        .withBody(body)));

        String json = sdk.purchaseDetails(this.PURCHASE_ID);
        System.out.println(json);
        ObjectMapper mapper = new ObjectMapper();
        TxDetailsResponse details = mapper.readValue(json, TxDetailsResponse.class);

        assertEquals(details.systemAddress, mainAddress);
        assertEquals(details.network_id, IOTA_NETWORK_ID);
        assertEquals(details.amount, amount, 0.0001);
        assertEquals(details.status, status);
    }

    @Ignore("No healthy node available")
    @Test
    public void VshouldConfirmPurchaseRequest() throws Exception {
        final String body = "{" + System.lineSeparator()
                + String.format("\"index\":\"%s\",", this.PURCHASE_ID) +
                System.lineSeparator()
                + String.format("\"transaction_id\":\"${json-unit.any-string}\"") +
                System.lineSeparator() + "}";

        wireMockRule.stubFor(post(urlPathEqualTo("/api/transactions/commit"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .withRequestBody(equalToJson(body))
                .willReturn(aResponse()
                        .withStatus(202)));

        sdk.purchaseRequestConfirm(PIN, this.PURCHASE_ID);
        logger.info("Tx confirmed");
    }

    @Ignore("No healthy node available")
    @Test
    public void shouldGetBalance() throws Exception {
        double balance = sdk.getWalletBalance(PIN);
        assertEquals((double) initBalance - 50000.5, balance, 0.0001);
    }

    @Test
    public void WshouldStartKycVerificationViviswap() throws Exception {
        final String reqBody = "{" + String.format("\"mail\":\"%s\",", EMAIL)
                + String.format("\"terms_accepted\":false") + "}";
        final String resBody = "{" + String.format("\"username\":\"%s\"", USERNAME) +
                "}";

        wireMockRule.stubFor(post(urlPathEqualTo("/api/viviswap/users"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .withRequestBody(equalToJson(reqBody))
                .willReturn(aResponse()
                        .withStatus(200)
                        .withHeader("Content-Type", "application/json")
                        .withBody(resBody)));

        String json = sdk.startViviswapKyc(EMAIL, false);
        ObjectMapper mapper = new ObjectMapper();
        NewViviswapUser user = mapper.readValue(json, NewViviswapUser.class);
        assertEquals(user.username, USERNAME);
    }

    @Ignore("Unhandled error")
    @Test
    public void XshouldGetKycDetailsViviswap() throws Exception {
        final String body = "{" + System.lineSeparator()
                + String.format("\"is_verified\":false,") + System.lineSeparator()
                + String.format("\"is_individual\":false,") + System.lineSeparator()
                + String.format("\"full_name\":\"Werner Karl Heisenberg\",") +
                System.lineSeparator()
                + String.format("\"submission_step\":\"General\",") + System.lineSeparator()
                + String.format("\"verified_step\":\"General\",") + System.lineSeparator()
                + String.format("\"verification_status\":\"Unverified\",") +
                System.lineSeparator()
                + String.format("\"monthly_limit_eur\":\"100.0\"")
                + System.lineSeparator() + "}";

        wireMockRule.stubFor(get(urlPathEqualTo("/api/viviswap/kyc/status"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)
                        .withHeader("Content-Type", "application/json")
                        .withBody(body)));

        String json = sdk.getViviswapKyc();
        ObjectMapper object = new ObjectMapper();
        ViviswapKycStatus status = object.readValue(json, ViviswapKycStatus.class);
        assertEquals(status.fullName, "Werner Karl Heisenberg");
        assertEquals(status.monthlyLimitEur, 100.0, 0.0001);
        assertEquals(status.verifiedStep, "General");
        assertEquals(status.submissionStep, "General");
        assertEquals(status.verificationStatus, "Unverified");
    }

    @Ignore("Unhandled error")
    @Test
    public void YshouldUpdateKycPartiallyViviswap() throws Exception {
        String json = sdk.updateViviswapKycPartial(true, false, false, false, "PL",
                "PL", "Werner Karl Heisenberg",
                "1901-12-05");
        ObjectMapper mapper = new ObjectMapper();
        ViviswapPartiallyKycDetails details = mapper.readValue(json,
                ViviswapPartiallyKycDetails.class);
        assertEquals(details.isIndividual, true);
        assertEquals(details.isPep, false);
        assertEquals(details.isUsCitizen, false);
        assertEquals(details.isRegulatoryDisclosure, false);
        assertEquals(details.countryOfResidence, "PL");
        assertEquals(details.nationality, "PL");
        assertEquals(details.fullName, "Werner Karl Heisenberg");
        assertEquals(details.dateOfBirth, "1901-12-05");
    }

    @Test
    public void ZAshouldCallSetViviswapKycIdentityDetails() throws Exception {
        // a simple test just to make sure that the binding works, not checking the
        // content or anything
        wireMockRule.stubFor(post(urlPathEqualTo("/api/viviswap/kyc/identity"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)));

        sdk.setViviswapKycIdentityDetails("Id", "2023-01-01", "123456",
                new byte[] {},
                "front.png", null, null, new byte[] {},
                "personal_video.mp4");
    }

    @Test
    public void ZBshouldCallSetViviswapKycResidenceDetails() throws Exception {
        // a simple test just to make sure that the binding works, not checking the
        // content or anything
        wireMockRule.stubFor(post(urlPathEqualTo("/api/viviswap/kyc/residence"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)));

        sdk.setViviswapKycResidenceDetails("DE", "Baden-Wurttemberg", "123456", "City", "Street1", "", false,
                "", false,
                new byte[] {},
                "document.jpg");
    }

    @Test
    public void ZCshouldCallGetViviswapKycAmlaOpenQuestions() throws Exception {
        // a simple test just to make sure that the binding works, not checking the
        // content or anything

        String body = """
                {
                    "questions": [
                        {
                            "id": "q1",
                            "question": "What is this?",
                            "possible_answers": ["one", "two", "three"],
                            "is_free_text": false,
                            "min_answers": 1,
                            "max_answers": 2
                        },
                        {
                            "id": "q2",
                            "question": "What is this?",
                            "possible_answers": ["one", "two", "three"],
                            "is_free_text": false,
                            "min_answers": 1,
                            "max_answers": 2
                        }
                    ]
                }
                """;

        wireMockRule.stubFor(get(urlPathEqualTo("/api/viviswap/kyc/questions"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)
                        .withHeader("Content-Type", "application/json")
                        .withBody(body)));

        sdk.getViviswapKycAmlaOpenQuestions();
    }

    @Test
    public void ZDshouldSetViviswapKycAmlaAnswer() throws Exception {
        // a simple test just to make sure that the binding works, not checking the
        // content or anything
        wireMockRule.stubFor(post(urlPathEqualTo("/api/viviswap/kyc/questions"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)));

        sdk.setViviswapKycAmlaAnswer("question_id", new String[] { "one", "two" }, "long free text answer");

    }

    @Test
    public void ZEshouldCallGetViviswapKycOpenDocuments() throws Exception {
        // a simple test just to make sure that the binding works, not checking the
        // content or anything

        String body = """
                {
                    "documents": [
                        {
                            "id": "doc1",
                            "is_back_image_required": false,
                            "type": "Id",
                            "description": "Identification document"
                        },
                        {
                            "id": "doc2",
                            "is_back_image_required": true,
                            "type": "DrivingLicense",
                            "description": "Driving license document"
                        }
                    ]
                }
                """;

        wireMockRule.stubFor(get(urlPathEqualTo("/api/viviswap/kyc/documents"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)
                        .withHeader("Content-Type", "application/json")
                        .withBody(body)));

        sdk.getViviswapKycOpenDocuments();
    }

    @Test
    public void ZFshouldSetViviswapKycDocument() throws Exception {
        // a simple test just to make sure that the binding works, not checking the
        // content or anything
        wireMockRule.stubFor(post(urlPathEqualTo("/api/viviswap/kyc/documents"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)));

        sdk.setViviswapKycDocument("document_id", "2023-01-01", "123456",
                new byte[] {},
                "front.png", null, null);
    }

    @Test
    public void ZGshouldGetRecoveryShare() throws Exception {
        String share = sdk.getRecoveryShare();
        assertNotEquals(share, null);
        sdk.setRecoveryShare(share);
    }

    @Test
    public void ZHshouldGetPreferredNetwork() throws Exception {

        String body = """
                    {"network_id": ""}
                """;
        wireMockRule.stubFor(get(urlPathEqualTo("/api/user/network"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)
                        .withHeader("Content-Type", "application/json")
                        .withBody(body)));
        String network = sdk.getPreferredNetwork();
        assertEquals(network, "");
    }

    @Test
    public void ZIshouldSetPreferredNetwork() throws Exception {
        wireMockRule.stubFor(put(urlPathEqualTo("/api/user/network"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(202)));
        sdk.setPreferredNetwork(IOTA_NETWORK_ID);

    }

    @Test
    public void ZJshouldGetPreferredNetwork() throws Exception {

        String body = """
                    {"network_id": "67a1f08edf55756bae21e7eb"}
                """;
        wireMockRule.stubFor(get(urlPathEqualTo("/api/user/network"))
                .withHeader("Authorization", equalTo(TOKEN_HEADER_VALUE))
                .withHeader("X-APP-NAME", equalTo(AUTH_PROVIDER))
                .willReturn(aResponse()
                        .withStatus(200)
                        .withHeader("Content-Type", "application/json")
                        .withBody(body)));
        String network = sdk.getPreferredNetwork();
        assertEquals(network, IOTA_NETWORK_ID);
    }

}
