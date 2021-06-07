package com.sim;

import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.List;

public class Province {
    String name;
    List<Pops> pops;
    List<Producer> producers;
}
