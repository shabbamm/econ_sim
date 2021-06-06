package aylah.geography;


public class Region {
    int continentId;
    int id;
    String name;

    public Region(int continentId, int id, String name) {
        System.out.println("Continent " + id + " initializing...");

        this.continentId = continentId;
        this.id = id;
        this.name = name;

        System.out.print("Done!");
    }
}
