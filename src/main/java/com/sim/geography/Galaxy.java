package com.sim.geography;

import com.fasterxml.jackson.annotation.JsonProperty;

public class Galaxy {
    @JsonProperty("dimensionId")
    public int dimensionId;
    @JsonProperty("id")
    public int id;
    @JsonProperty("name")
    public String name;

    public Galaxy() {

    }

    public Galaxy(int dimensionId, int id, String name) {
        this.dimensionId = dimensionId;
        this.id = id;
        this.name = name;
    }
}
