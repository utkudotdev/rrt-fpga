`ifndef MEMBUS_SV
`define MEMBUS_SV

// TODO: At some point, we probably want some bus controller that can talk to multiple clients
// This might be good so we can have one module be responsible for fast bulk loading from serial, which
// I don't want to shove in the occupancy_grid module
interface memory_bus #(
    parameter ADDR_WIDTH,
    parameter DATA_WIDTH
);
    logic [ADDR_WIDTH-1:0] addr;
    logic [DATA_WIDTH-1:0] r_data;
    logic [DATA_WIDTH-1:0] w_data;
    logic we;

    modport memory (input addr, w_data, we, output r_data);
    modport client (input r_data, output addr, w_data, we);
endinterface

`endif
