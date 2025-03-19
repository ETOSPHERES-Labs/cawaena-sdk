package com.cawaena.model;

import java.util.List;

import com.fasterxml.jackson.annotation.JsonProperty;

public class TxDetailsResponse {

    @JsonProperty("system_address")
    public String systemAddress;

    @JsonProperty("amount")
    public double amount;

    @JsonProperty("status")
    public String status;

    @JsonProperty("network_key")
    public String network_key;

    @JsonProperty("invalid_reasons")
    public String[] invalid_reasons;
}
