package sim;

import java.io.IOException;
import java.util.List;

import com.fasterxml.jackson.annotation.JsonProperty;

public class World {

    // World's represent are largest level of abstraction outside the
    // universe/GameState.
    // They hold Pop's and Resource's and produce value reports for the GameState
    // each turn

    @JsonProperty("worldId")
    private long worldId;

    @JsonProperty("name")
    private String name;

    @JsonProperty("popLimit")
    private long popLimit;

    @JsonProperty("pops")
    private List<Pop> pops;

    @JsonProperty("resources")
    private List<Resource> resources;

    public World() throws IOException {

    }

    public World(long worldId, String name, long popLimit) throws IOException {
        this.worldId = worldId;
        this.name = name;
        this.popLimit = popLimit;
    }

    public long getWorldId() {
        return this.worldId;
    }

    public String getName() {
        return this.name;
    }

    public long getPopLimit() {
        return this.popLimit;
    }
}