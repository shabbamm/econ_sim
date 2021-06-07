package com.sim;

import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.HashMap;

public class Galaxy {
    @JsonProperty("id")
    public int id;
    @JsonProperty("name")
    public String name;

    public Galaxy() {

    }

    public Galaxy(int id, String name) {
        this.id = id;
        this.name = name;
    }
}
