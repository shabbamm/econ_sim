package com.sim;

import com.fasterxml.jackson.annotation.JsonProperty;

public class Continent {
    @JsonProperty("worldId")
    public int worldId;
    @JsonProperty("id")
    public int id;
    @JsonProperty("name")
    public String name;

    public Continent() {

    }

    public Continent(int worldId, int id, String name) {
        this.worldId = worldId;
        this.id = id;
        this.name = name;
    }
}
