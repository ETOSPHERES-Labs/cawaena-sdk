package com.etospheres.etopay.model;

import com.fasterxml.jackson.annotation.JsonProperty;

public class ViviswapKycStatus {
    @JsonProperty("full_name")
    public String fullName;

    @JsonProperty("submission_step")
    public String submissionStep;

    @JsonProperty("verified_step")
    public String verifiedStep;

    @JsonProperty("verification_status")
    public String verificationStatus;

    @JsonProperty("monthly_limit_eur")
    public float monthlyLimitEur;
}
