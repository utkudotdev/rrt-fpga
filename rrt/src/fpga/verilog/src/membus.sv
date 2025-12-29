`ifndef MEMBUS_SV
`define MEMBUS_SV

// TODO: At some point, we probably want some bus controller that can talk to multiple clients
// This might be good so we can have one module be responsible for fast bulk loading from serial, which
// I don't want to shove in the occupancy_grid module
interface memory_bus #(
    parameter ADDR_WIDTH,
    parameter DATA_WIDTH
);
    logic [ADDR_WIDTH-1:0] address;
    logic [DATA_WIDTH-1:0] read_data;
    logic [DATA_WIDTH-1:0] write_data;
    logic write_enable;

    modport memory (input address, write_data, write_enable, output read_data);
    modport client (input read_data, output address, write_data, write_enable);
endinterface

`endif
