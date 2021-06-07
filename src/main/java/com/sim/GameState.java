package com.sim;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Scanner;

public class GameState {
    public ArrayList<Galaxy> galaxies;
    public ArrayList<World> worlds;
    public ArrayList<Continent> continents;
    public ArrayList<Region> regions;
    public ArrayList<Province> provinces;
    public ArrayList<Pops> communities;

    public GameState() throws IOException {
        System.out.println("GameState initializing...");

        ObjectMapper objectMapper = new ObjectMapper();

        this.galaxies = objectMapper.readValue(loadConfig("config/galaxies.json"), new TypeReference<ArrayList<Galaxy>>() {});
        this.worlds = objectMapper.readValue(loadConfig("config/worlds.json"), new TypeReference<ArrayList<World>>() {});
        this.continents = objectMapper.readValue(loadConfig("config/continents.json"), new TypeReference<ArrayList<Continent>>() {});
        this.regions = objectMapper.readValue(loadConfig("config/regions.json"), new TypeReference<ArrayList<Region>>() {});
        this.provinces = objectMapper.readValue(loadConfig("config/provinces.json"), new TypeReference<ArrayList<Province>>() {});
        this.communities = objectMapper.readValue(loadConfig("config/communities.json"), new TypeReference<ArrayList<Pops>>() {});

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

    public void addGalaxy(String name) {
        this.galaxies.add(new Galaxy(this.galaxies.size() + 1, name));
    }

    public void removeGalaxy(int id) {
        this.galaxies.remove(id);
    }

    public void addWorld(int galaxyId, String name) {
        this.worlds.add(new World(galaxyId, this.worlds.size() + 1, name));
    }

    public void removeWorld(int id) {
        this.worlds.remove(id);
    }

    public void addContinent(int worldId, String name) {
        this.continents.add(new Continent(worldId, this.continents.size() + 1, name));
    }

    public void removeContinent(int id) {
        this.continents.remove(id);
    }

    public void addRegion(int continentId, String name) {
        this.regions.add(new Region(continentId, this.regions.size() + 1, name));
    }

    public void removeRegion(int id) {
        this.regions.remove(id);
    }

    public void addProvince(int regionId, String name) {
        this.provinces.add(new Province(regionId, this.provinces.size() + 1, name));
    }

    public void removeProvince(int id) {
        this.provinces.remove(id);
    }

    public void addCommunity(int provinceId, int size, long money) {
        this.communities.add(new Pops(provinceId, this.communities.size() + 1, size, money));
    }

    public void removeCommunity(int id) {
        this.communities.remove(id);
    }
}
