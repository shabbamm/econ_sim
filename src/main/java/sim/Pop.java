package sim;

import com.fasterxml.jackson.annotation.JsonProperty;

public class Pop {
    @JsonProperty("worldId")
    private long worldId;
    @JsonProperty("popId")
    private long popId;
    @JsonProperty("size")
    private long size;
    @JsonProperty("money")
    private long money;

    public Pop() {

    }

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

    @Override
    public String toString() {
        String result = "Pop:worldId[" + getWorldId() + "]popId[" + getPopId() + "]size[" + getSize() + "]" + "money["
                + getMoney() + "]";
        return result;
    }
}
