package com.sim;

import com.fasterxml.jackson.annotation.JsonProperty;

public class World {
    @JsonProperty("id")
    public int id;
    @JsonProperty("name")
    public String name;

    public World() {

    }

    public World(int id, String name) {
        this.id = id;
        this.name = name;
    }
}
