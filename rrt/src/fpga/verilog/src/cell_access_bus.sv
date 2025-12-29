`ifndef CELL_ACCESS_BUS_SV
`define CELL_ACCESS_BUS_SV

// Interface for occupancy grid cell access
interface cell_access_bus #(
    parameter GRID_WIDTH_LOG2,
    parameter GRID_HEIGHT_LOG2
);
    logic [GRID_WIDTH_LOG2-1:0] cell_x;
    logic [GRID_HEIGHT_LOG2-1:0] cell_y;
    logic vld_in;
    logic vld_out;
    logic rdy;
    logic we;
    logic w_occupied;
    logic r_occupied;

    modport client (
        output cell_x, cell_y, vld_in, we, w_occupied,
        input vld_out, rdy, r_occupied
    );
    
    modport server (
        input cell_x, cell_y, vld_in, we, w_occupied,
        output vld_out, rdy, r_occupied
    );
endinterface

`endif
