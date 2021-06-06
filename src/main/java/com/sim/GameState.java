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

    public GameState() throws IOException {
        System.out.println("GameState initializing...");

        ObjectMapper objectMapper = new ObjectMapper();

        this.dimensions = objectMapper.readValue(loadConfig("config/dimensions.json"), new TypeReference<ArrayList<Dimension>>(){});

        System.out.println(dimensions.get(1).name);

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
