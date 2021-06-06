package com.sim.geography;

public class Community {
    int provinceId;
    int id;
    int size;
    int money;

    public Community(int provinceId, int id, int size, int money) {
        System.out.println("Community " + id + " initializing...");

        this.provinceId = provinceId;
        this.id = id;
        this.size = size;
        this.money = money;
        System.out.print("Done!");
    }
}
