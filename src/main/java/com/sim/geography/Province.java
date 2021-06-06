package com.sim.geography;

import com.fasterxml.jackson.annotation.JsonProperty;

public class Province {
    @JsonProperty("continentId")
    public int continentId;
    @JsonProperty("id")
    public int id;
    @JsonProperty("name")
    public String name;

    public Province() {
        
    }

    public Province(int continentId, int id, String name) {
        this.continentId = continentId;
        this.id = id;
        this.name = name;
    }
}
