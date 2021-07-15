package sim;

import java.io.IOException;

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

    private long popLimit;

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

    @Override
    public String toString() {
        String result = "World:\n  worldId[" + getWorldId() + "]\n  name[" + getName() + "]\n  popLimit["
                + getPopLimit() + "]";
        return result;
    }
}