package com.etospheres.etopay.deserializer;

import java.io.IOException;

import com.fasterxml.jackson.databind.deser.std.StdDeserializer;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;

import com.etospheres.etopay.model.CaseIdResponse;

public class CaseIdResponseDeserializer extends StdDeserializer<CaseIdResponse> {

    public CaseIdResponseDeserializer() {
        this(null);
    }

    public CaseIdResponseDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public CaseIdResponse deserialize(JsonParser jp, DeserializationContext ctxt)
            throws IOException, JsonProcessingException {
        JsonNode node = jp.getCodec().readTree(jp);
        String caseId = node.get("case_id").asText();
        String caseUrl = node.get("case_url").asText();

        return new CaseIdResponse(caseId, caseUrl);
    }
}
