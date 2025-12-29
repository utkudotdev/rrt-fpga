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
    input logic [GRID_WIDTH_LOG2-1:0] cell_x,
    input logic [GRID_HEIGHT_LOG2-1:0] cell_y,
    input logic input_valid,
    output logic output_valid,
    output logic ready_for_input,
    input logic write_enable,
    input logic write_occupied,
    output logic read_occupied
);
    memory_bus #(.ADDR_WIDTH(ADDR_WIDTH), .DATA_WIDTH(DATA_WIDTH)) bus ();
    
    bram #(.ADDR_WIDTH(ADDR_WIDTH), .DATA_WIDTH(DATA_WIDTH)) bram_inst (
        .clk(clk),
        .bus(bus.memory) 
    );
    
    occupancy_grid #(.GRID_WIDTH_LOG2(GRID_WIDTH_LOG2), .GRID_HEIGHT_LOG2(GRID_HEIGHT_LOG2)) uut (
        .clk(clk),
        .rst_n(rst_n),
        .cell_x(cell_x),
        .cell_y(cell_y),
        .input_valid(input_valid),
        .output_valid(output_valid),
        .ready_for_input(ready_for_input),
        .write_enable(write_enable),
        .write_occupied(write_occupied),
        .read_occupied(read_occupied),
        .mem(bus.client)
    );
endmodule
