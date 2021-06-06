package com.sim.geography;

import com.fasterxml.jackson.annotation.JsonProperty;

public class Dimension {
    @JsonProperty("id")
    public int id;
    @JsonProperty("name")
    public String name;

    //allows deserializing json into an Object
    public Dimension() {

    }

    //for creating new objects while running
    public Dimension(int id, String name) {
        System.out.print("Dimension " + id + " initializing...");

        this.id = id;
        this.name = name;

        System.out.println("Done!");
    }
}
