package com.sim;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.sim.geography.*;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Scanner;

public class GameState {
    public ArrayList<Dimension> dimensions;
    public ArrayList<Galaxy> galaxies;
    public ArrayList<World> worlds;
    public ArrayList<Continent> continents;
    public ArrayList<Region> regions;
    public ArrayList<Province> provinces;
    public ArrayList<Community> communities;


    public GameState() throws IOException {
        System.out.println("GameState initializing...");

        ObjectMapper objectMapper = new ObjectMapper();

        this.dimensions = objectMapper.readValue(loadConfig("config/dimensions.json"), new TypeReference<ArrayList<Dimension>>(){});
        this.galaxies = objectMapper.readValue(loadConfig("config/galaxies.json"), new TypeReference<ArrayList<Galaxy>>(){});
        this.worlds = objectMapper.readValue(loadConfig("config/worlds.json"), new TypeReference<ArrayList<World>>(){});
        this.continents = objectMapper.readValue(loadConfig("config/continents.json"), new TypeReference<ArrayList<Continent>>(){});
        this.regions = objectMapper.readValue(loadConfig("config/regions.json"), new TypeReference<ArrayList<Region>>(){});
        this.provinces = objectMapper.readValue(loadConfig("config/provinces.json"), new TypeReference<ArrayList<Province>>(){});
        this.communities = objectMapper.readValue(loadConfig("config/communities.json"), new TypeReference<ArrayList<Community>>(){});


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
