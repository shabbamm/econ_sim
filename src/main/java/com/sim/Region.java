package com.sim;

import com.fasterxml.jackson.annotation.JsonProperty;

public class Region {
    @JsonProperty("continentId")
    public int continentId;
    @JsonProperty("id")
    public int id;
    @JsonProperty("name")
    public String name;

    public Region() {

    }

    public Region(int continentId, int id, String name) {
        this.continentId = continentId;
        this.id = id;
        this.name = name;
    }
}
