package com.cawaena.model;

import java.util.List;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

@JsonIgnoreProperties(ignoreUnknown = true)
public class Network {
    @JsonProperty("key")
    public String key;

    @JsonProperty("display_name")
    public String name;
}
