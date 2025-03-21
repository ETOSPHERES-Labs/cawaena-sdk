package com.etospheres.etopay.deserializer;

import java.io.IOException;

import com.fasterxml.jackson.databind.deser.std.StdDeserializer;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;

import com.etospheres.etopay.model.NewViviswapUser;

public class NewViviswapUserDeserializer extends StdDeserializer<NewViviswapUser> {

    public NewViviswapUserDeserializer() {
        this(null);
    }

    public NewViviswapUserDeserializer(Class<?> vc) {
        super(vc);
    }

    @Override
    public NewViviswapUser deserialize(JsonParser jp, DeserializationContext ctxt)
            throws IOException, JsonProcessingException {
        JsonNode node = jp.getCodec().readTree(jp);
        String username = node.get("username").asText();

        return new NewViviswapUser(username);
    }
}
