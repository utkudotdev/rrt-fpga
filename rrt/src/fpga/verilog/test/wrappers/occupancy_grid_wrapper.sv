`include "occupancy_grid.sv"
`include "bram.sv"

module occupancy_grid_wrapper #(
    parameter GRID_WIDTH_LOG2,
    parameter GRID_HEIGHT_LOG2,
    parameter DATA_WIDTH,
    parameter ADDR_WIDTH
) (
    input logic clk,
    input logic rst_n,
    input logic [GRID_WIDTH_LOG2-1:0] cell_x_in,
    input logic [GRID_HEIGHT_LOG2-1:0] cell_y_in,
    input logic vld_in,
    output logic vld_out,
    output logic rdy,
    input logic we,
    input logic w_occupied,
    output logic r_occupied
);
    memory_bus #(.ADDR_WIDTH(ADDR_WIDTH), .DATA_WIDTH(DATA_WIDTH)) bus ();
    cell_access_bus #(.GRID_WIDTH_LOG2(GRID_WIDTH_LOG2), .GRID_HEIGHT_LOG2(GRID_HEIGHT_LOG2)) cell_bus ();
    
    // Connect wrapper ports to cell_bus interface
    assign cell_bus.cell_x = cell_x_in;
    assign cell_bus.cell_y = cell_y_in;
    assign cell_bus.vld_in = vld_in;
    assign vld_out = cell_bus.vld_out;
    assign rdy = cell_bus.rdy;
    assign cell_bus.we = we;
    assign cell_bus.w_occupied = w_occupied;
    assign r_occupied = cell_bus.r_occupied;
    
    bram #(.ADDR_WIDTH(ADDR_WIDTH), .DATA_WIDTH(DATA_WIDTH)) bram_inst (
        .clk(clk),
        .bus(bus.memory) 
    );
    
    occupancy_grid #(.GRID_WIDTH_LOG2(GRID_WIDTH_LOG2), .GRID_HEIGHT_LOG2(GRID_HEIGHT_LOG2)) uut (
        .clk(clk),
        .rst_n(rst_n),
        .cell_bus(cell_bus.client),
        .mem(bus.client)
    );
endmodule
