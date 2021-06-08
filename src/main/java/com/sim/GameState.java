package com.sim;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.util.List;
import java.util.Scanner;

public class GameState {
    public List<World> worlds;

    public GameState() throws IOException {
        System.out.println("GameState initializing...");

        ObjectMapper objectMapper = new ObjectMapper();
        this.worlds = objectMapper.readValue(loadConfig("config/worlds.json"), new TypeReference<List<World>>() {
        });

        System.out.println(this.worlds);

        System.out.println("GameState initialized!");
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
