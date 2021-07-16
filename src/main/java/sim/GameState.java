package sim;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.io.IOException;
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

    public List<World> worlds;

    public GameState() throws IOException {
        System.out.println("GameState initializing...");

        // Using Jackson framework to serialize/deserialize json data as the save files
        ObjectMapper objectMapper = new ObjectMapper();

        this.worlds = objectMapper.readValue(SaveLoader.loadConfig("config/save_file.json"),
                new TypeReference<ArrayList<World>>() {
                });

        // For visualizing
        // # TODO create method meant for visualizing and move into there

        for (World world : worlds) {
            System.out.println(world.toString());
        }

        System.out.println("GameState initialized!");
    }
}
