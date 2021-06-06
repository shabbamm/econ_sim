package com.sim;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;
import java.util.HashMap;
import java.util.Scanner;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.sim.geography.Community;
import com.sim.geography.Continent;
import com.sim.geography.Dimension;
import com.sim.geography.Galaxy;
import com.sim.geography.Province;
import com.sim.geography.Region;
import com.sim.geography.World;

public class GameState {
    public HashMap<Integer, Dimension> dimensions;
    public HashMap<Integer, Galaxy> universes;
    public HashMap<Integer, World> worlds;
    public HashMap<Integer, Continent> continents;
    public HashMap<Integer, Region> regions;
    public HashMap<Integer, Province> provinces;
    public HashMap<Integer, Community> communities;

    public GameState() throws IOException {
        System.out.println("GameState initializing...");

        ObjectMapper objectMapper = new ObjectMapper();

        /*
         * this.dimensions =
         * objectMapper.readValue(loadConfig("config/dimensions.json"), new
         * TypeReference<HashMap<Integer, Dimension>>() { }); String jsonObject =
         * "{\"brand\":\"ford\", \"doors\":5}";
         * 
         * ObjectMapper objectMapper = new ObjectMapper(); Map<String, Object> jsonMap =
         * objectMapper.readValue(, );
         */

        this.dimensions = new HashMap<>();
        this.dimensions.put(0, new Dimension(0, "alpha"));

        Dimension test = this.dimensions.get(0);

        objectMapper.writeValueAsString(test);

        loadConfig("config/galaxies.json");
        loadConfig("config/worlds.json");
        loadConfig("config/continents.json");
        loadConfig("config/regions.json");
        loadConfig("config/provinces.json");
        loadConfig("config/communities.json");

        System.out.println("GameState done!");
    }

    public String loadConfig(String filename) throws FileNotFoundException {
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
