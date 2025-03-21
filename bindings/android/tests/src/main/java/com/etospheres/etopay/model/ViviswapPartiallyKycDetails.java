package com.etospheres.etopay.model;

import com.fasterxml.jackson.annotation.JsonProperty;

public class ViviswapPartiallyKycDetails {
    @JsonProperty("is_individual")
    public boolean isIndividual;

    @JsonProperty("is_pep")
    public boolean isPep;

    @JsonProperty("is_us_citizen")
    public boolean isUsCitizen;

    @JsonProperty("is_regulatory_disclosure")
    public boolean isRegulatoryDisclosure;

    @JsonProperty("country_of_residence")
    public String countryOfResidence;

    @JsonProperty("nationality")
    public String nationality;

    @JsonProperty("full_name")
    public String fullName;

    @JsonProperty("date_of_birth")
    public String dateOfBirth;
}
