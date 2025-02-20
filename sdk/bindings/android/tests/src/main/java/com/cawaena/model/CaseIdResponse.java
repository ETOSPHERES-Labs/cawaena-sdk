package com.cawaena.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;

import com.cawaena.deserializer.CaseIdResponseDeserializer;

@JsonDeserialize(using = CaseIdResponseDeserializer.class)
public class CaseIdResponse {
    @JsonProperty("case_id")
    public String caseId;
    @JsonProperty("case_url")
    public String caseUrl;

    public CaseIdResponse(String caseId, String caseUrl) {
        this.caseId = caseId;
        this.caseUrl = caseUrl;
    }
}
