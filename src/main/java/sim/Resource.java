package sim;

import com.fasterxml.jackson.annotation.JsonProperty;

// A Resource acts as what is available for Pop communities to interact with to generate n amounts of value
// Example: there are 100,000 farmers in a Pop, therefore that creates 100 wheat per turn w/ any increase/deacreas in pop affecting the value
public class Resource {
    @JsonProperty("name")
    private String name;

    @JsonProperty("resourceId")
    private long resourceId;

    @JsonProperty("productivity")
    private float productivity;

    public Resource() {

    }

    public String getName() {
        return name;
    }

    public long getResourceId() {
        return resourceId;
    }

    public float getProductivity() {
        return productivity;
    }

    public void setProductivity(float productivity) {
        this.productivity = productivity;
    }
}
