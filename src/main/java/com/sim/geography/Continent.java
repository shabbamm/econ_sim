package com.sim.geography;

public class Continent {
    int worldId;
    int id;
    String name;

    public Continent(int worldId, int id, String name) {
        System.out.println("Continent " + id + " initializing...");

        this.worldId = worldId;
        this.id = id;
        this.name = name;

        System.out.print("Done!");
    }
}
