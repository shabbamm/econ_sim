package sim;

import com.fasterxml.jackson.annotation.JsonProperty;

public class Pop {
    // Pop's are going to be the representation of like groups of people based on demographics
    // Ideally they not just hold the data about a particular group, but produce 'reports' that bubble up to the world they inhabit
    // TODO create methods that tally value produced by Pop each 'turn'

    // This associates the field w/ the json field for the ObjectMapper in GameState
    @JsonProperty("worldId")
    private long worldId;
    @JsonProperty("popId")
    private long popId;
    @JsonProperty("size")
    private long size;
    @JsonProperty("money")
    private long money;

    // Interestingly, Jackson requires and uses a constructor w/ no parameters in order to serialize/deserialize json
    public Pop() {

    }

    // This was created first w/ parameters I imagined being required each time a World created a new Pop
    // TODO if this is not required and can be removed, do so
    public Pop(long worldId, long popId, long size, long money) {
        this.worldId = worldId;
        this.popId = popId;
        this.size = size;
        this.money = money;
    }

    public long getWorldId() {
        return this.worldId;
    }

    public long getPopId() {
        return this.popId;
    }

    public long getSize() {
        return this.size;
    }

    public long getMoney() {
        return this.money;
    }

    // TODO find a better way of forming data as a string
    @Override
    public String toString() {
        String result = "Pop:worldId[" + getWorldId() + "]popId[" + getPopId() + "]size[" + getSize() + "]" + "money["
                + getMoney() + "]";
        return result;
    }
}
