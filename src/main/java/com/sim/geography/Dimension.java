package com.sim.geography;

public class Dimension {
    int id;
    String name;

    public Dimension(int id, String name) {
        System.out.println("Dimension " + id + " initializing...");

        this.id = id;
        this.name = name;

        System.out.print("Done!");
    }
}
