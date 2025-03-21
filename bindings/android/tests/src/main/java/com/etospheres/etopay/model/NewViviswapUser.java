package com.etospheres.etopay.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;

import com.etospheres.etopay.deserializer.NewViviswapUserDeserializer;

@JsonDeserialize(using = NewViviswapUserDeserializer.class)
public class NewViviswapUser {

    @JsonProperty("username")
    public String username;

    public NewViviswapUser(String username) {
        this.username = username;
    }

}
