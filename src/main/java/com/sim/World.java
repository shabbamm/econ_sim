package com.sim;

import com.fasterxml.jackson.annotation.JsonProperty;

public class World {
    @JsonProperty("galaxyId")
    public int galaxyId;
    @JsonProperty("id")
    public int id;
    @JsonProperty("name")
    public String name;

    public World() {

    }

    public World(int galaxyId, int id, String name) {
        this.galaxyId = galaxyId;
        this.id = id;
        this.name = name;
    }
}
