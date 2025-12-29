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
    input logic we,
    input logic w_occupied,
    output logic r_occupied
);
    memory_bus #(.ADDR_WIDTH(ADDR_WIDTH), .DATA_WIDTH(DATA_WIDTH)) bus (.clk(clk));
    
    bram #(.ADDR_WIDTH(ADDR_WIDTH), .DATA_WIDTH(DATA_WIDTH)) bram_inst (
        .bus(bus.memory) 
    );
    
    occupancy_grid #(.GRID_WIDTH_LOG2(GRID_WIDTH_LOG2), .GRID_HEIGHT_LOG2(GRID_HEIGHT_LOG2)) uut (
        .rst_n(rst_n),
        .cell_x_in(cell_x_in),
        .cell_y_in(cell_y_in),
        .we(we),
        .w_occupied(w_occupied),
        .r_occupied(r_occupied),
        .mem(bus.client)
    );
endmodule
