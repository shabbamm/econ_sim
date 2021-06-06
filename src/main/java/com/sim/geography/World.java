package com.sim.geography;

public class World {
    int universeId;
    int id;
    String name;

    public World(int universeId, int id, String name) {
        System.out.println("World " + id + " initializing...");

        this.universeId = universeId;
        this.id = id;
        this.name = name;

        System.out.print("Done!");
    }
}
