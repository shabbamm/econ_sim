package aylah;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.HashMap;
import java.util.Scanner;

import main.java.aylah.geography.Community;
import main.java.aylah.geography.Continent;
import main.java.aylah.geography.Dimension;
import main.java.aylah.geography.Galaxy;
import main.java.aylah.geography.Province;
import main.java.aylah.geography.Region;
import main.java.aylah.geography.World;


public class GameState {
    public HashMap<Integer, Dimension> dimensions;
    public HashMap<Integer, Galaxy> universes;
    public HashMap<Integer, World> worlds;
    public HashMap<Integer, Continent> continents;
    public HashMap<Integer, Region> regions;
    public HashMap<Integer, Province> provinces;
    public HashMap<Integer, Community> communities;


    public GameState() throws FileNotFoundException {
        System.out.println("GameState initializing...");
        //ObjectMapper objectMapper = new ObjectMapper();

        loadConfig("config/dimensions.json");
        loadConfig("config/galaxies.json");
        loadConfig("config/worlds.json");
        loadConfig("config/continents.json");
        loadConfig("config/regions.json");
        loadConfig("config/provinces.json");
        loadConfig("config/communities.json");

        System.out.println("GameState initialized!");
    }

    public String loadConfig(String filename) throws FileNotFoundException{
        File file = new File(filename);
        Scanner scanner = new Scanner(file);
        String data = new String();

        while (scanner.hasNextLine()) {
            data += scanner.nextLine();
        }
        
        scanner.close();
        return data;
    }
}
