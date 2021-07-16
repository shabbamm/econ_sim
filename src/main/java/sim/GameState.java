package sim;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.io.IOException;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

// GameState is where we begin to actually call methods and hold our top level collections of Objects/global data
public class GameState {

    // # TODO figure out if we want: a nested set of Objects where the collection of
    // the largest Object is in GameState: List<World> worlds[ World: List<Pop>
    // pops[ Pop, Pop, Pop ] ], World: List<Pop> pops[ Pop, Pop, Pop ] ] or if it
    // would be better to store all collections at the GameState level like so:
    // List<World> worlds[ World, World, World ]
    //
    // List<Pop> pops [ Pop, Pop, Pop ]

    private List<World> worlds;

    public GameState() throws IOException {
        System.out.println("GameState initializing...");

        // Using Jackson framework to serialize/deserialize json data as the save files
        ObjectMapper objectMapper = new ObjectMapper();

        this.worlds = objectMapper.readValue(SaveHandler.loadConfig(), new TypeReference<ArrayList<World>>() {
        });

        System.out.println("GameState initialized!");

        createWorld("Mars", 0);

        objectMapper.writeValue(Paths.get(SaveHandler.getFilePath()).toFile(), worlds);
    }

    public void createWorld(String name, long popLimit) throws IOException {
        long worldId = this.worlds.size();

        for (World world : worlds) {
            if (world.getWorldId() == worldId) {
                worldId++;
            }
        }

        this.worlds.add(new World(worldId, name, popLimit));
    }
}
