package sim;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

public class GameState {
    public List<World> worlds;

    public GameState() throws IOException {
        System.out.println("GameState initializing...");

        ObjectMapper objectMapper = new ObjectMapper();
        this.worlds = objectMapper.readValue(SaveLoader.loadConfig("config/worlds.json"),
                new TypeReference<ArrayList<World>>() {
                });

        // for (World world : worlds) {
        // System.out.println(world.toString());
        // }

        System.out.println("GameState initialized!");
    }

}
