package sim;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

public class World {

    // id, name, popLimit, collectionPops

    @JsonProperty("worldId")
    private long worldId;
    @JsonProperty("name")
    private String name;

    private long popLimit;
    private List<Pop> pops;

    public World() throws IOException {
        String path = SaveLoader.loadConfig("config/pops.json");
        ObjectMapper objectMapper = new ObjectMapper();

        JsonNode node = objectMapper.readTree(path);

        System.out.println(node.findValue("worldId"));

        if (this.worldId == node.get("worldId").asLong()) {
            this.pops = objectMapper.readValue(path, new TypeReference<List<Pop>>() {
            });
        }

    }

    public World(long worldId, String name, long popLimit) throws IOException {
        this.worldId = worldId;
        this.name = name;
        this.popLimit = popLimit;
        this.pops = new ArrayList<Pop>();
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

    public List<Pop> getPops() {
        return this.pops;
    }

    @Override
    public String toString() {
        String result = "World\n  worldId[" + getWorldId() + "]\n  name[" + getName() + "]\n  popLimit[" + getPopLimit()
                + "]\n  " + this.pops;
        return result;
    }
}