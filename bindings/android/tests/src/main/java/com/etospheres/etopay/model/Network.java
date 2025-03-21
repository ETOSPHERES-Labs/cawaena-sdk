package com.etospheres.etopay.model;

import java.util.List;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

@JsonIgnoreProperties(ignoreUnknown = true)
public class Network {
    @JsonProperty("id")
    public String id;

    @JsonProperty("name")
    public String name;
}
