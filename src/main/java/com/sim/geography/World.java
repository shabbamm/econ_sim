package com.sim.geography;

import com.fasterxml.jackson.annotation.JsonProperty;

public class World {
    @JsonProperty("universeId")
    public int universeId;
    @JsonProperty("id")
    public int id;
    @JsonProperty("name")
    public String name;

    public World() {

    }

    public World(int universeId, int id, String name) {
        this.universeId = universeId;
        this.id = id;
        this.name = name;
    }
}
