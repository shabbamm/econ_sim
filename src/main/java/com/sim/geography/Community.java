package com.sim.geography;

import com.fasterxml.jackson.annotation.JsonProperty;

public class Community {
    @JsonProperty("provinceId")
    public int provinceId;
    @JsonProperty("id")
    public int id;
    @JsonProperty("size")
    public int size;
    @JsonProperty("money")
    public int money;

    public Community() {

    }

    public Community(int provinceId, int id, int size, int money) {
        this.provinceId = provinceId;
        this.id = id;
        this.size = size;
        this.money = money;
    }
}
