package com.sim.geography;

public class Province {
    int continentId;
    int id;
    String name;

    public Province(int continentId, int id, String name) {
        System.out.println("Province " + id + "initializing...");

        this.continentId = continentId;
        this.id = id;
        this.name = name;

        System.out.print("Done!");
    }
}
