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
    
    bram #(.ADDR_WIDTH(ADDR_WIDTH), .DATA_WIDTH(DATA_WIDTH)) bram_inst (
        .clk(clk),
        .bus(bus.memory) 
    );
    
    occupancy_grid #(.GRID_WIDTH_LOG2(GRID_WIDTH_LOG2), .GRID_HEIGHT_LOG2(GRID_HEIGHT_LOG2)) uut (
        .clk(clk),
        .rst_n(rst_n),
        .cell_x_in(cell_x_in),
        .cell_y_in(cell_y_in),
        .vld_in(vld_in),
        .vld_out(vld_out),
        .rdy(rdy),
        .we(we),
        .w_occupied(w_occupied),
        .r_occupied(r_occupied),
        .mem(bus.client)
    );
endmodule
