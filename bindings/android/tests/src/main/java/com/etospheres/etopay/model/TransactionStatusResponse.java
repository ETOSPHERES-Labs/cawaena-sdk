package com.etospheres.etopay.model;

import com.fasterxml.jackson.annotation.JsonProperty;

public class TransactionStatusResponse {

    @JsonProperty("status")
    public String status;

    public TransactionStatusResponse(String status) {
        this.status = status;
    }

}
