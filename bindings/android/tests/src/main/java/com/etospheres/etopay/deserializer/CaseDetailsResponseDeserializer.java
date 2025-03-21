package com.etospheres.etopay.deserializer;

import java.io.IOException;

import com.fasterxml.jackson.databind.deser.std.StdDeserializer;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;

import com.etospheres.etopay.model.CaseDetailsResponse;

public class CaseDetailsResponseDeserializer extends StdDeserializer<CaseDetailsResponse> {

    public CaseDetailsResponseDeserializer() {
        this(null);
    }

    public CaseDetailsResponseDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public CaseDetailsResponse deserialize(JsonParser jp, DeserializationContext ctxt)
            throws IOException, JsonProcessingException {
        JsonNode node = jp.getCodec().readTree(jp);
        String caseId = node.get("case_id").asText();
        boolean archived = node.get("archived").asBoolean();
        String status = node.get("status").asText();

        return new CaseDetailsResponse(caseId, archived, status);
    }
}
