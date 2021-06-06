package com.sim.geography;

public class Dimension {
    public int id;
    public String name;

    public Dimension(int id, String name) {
        System.out.print("Dimension " + id + " initializing...");

        this.id = id;
        this.name = name;

        System.out.println("Done!");
    }
}
