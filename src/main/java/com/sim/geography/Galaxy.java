package aylah.geography;


public class Galaxy {
    int dimensionId;
    int id;
    String name;

    public Galaxy(int dimensionId, int id, String name) {
        System.out.println("Galaxy " + id + " initializing...");

        this.dimensionId = dimensionId;
        this.id = id;
        this.name = name;

        System.out.print("Done!");
    }
}
