`include "directed_energy_weapon.sv"

module directed_energy_weapon_wrapper #(
    parameter GRID_WIDTH_LOG2,
    parameter GRID_HEIGHT_LOG2
) (
    input logic clk,
    input logic rst_n,

    input point_t a,
    input point_t b,

    output logic occupied,

    input logic input_valid,
    output logic done,

    // Grid stuff
    output logic [GRID_WIDTH_LOG2-1:0] grid_cell_x,
    output logic [GRID_HEIGHT_LOG2-1:0] grid_cell_y,
    output logic grid_input_valid,
    input logic grid_output_valid,
    input logic grid_ready_for_input,
    output logic grid_write_enable,
    output logic grid_write_occupied,
    input logic grid_read_occupied
);
    occupancy_grid_bus #(.GRID_WIDTH_LOG2(GRID_WIDTH_LOG2), .GRID_HEIGHT_LOG2(GRID_HEIGHT_LOG2)) grid_bus ();

    assign grid_cell_x = grid_bus.cell_x;
    assign grid_cell_y = grid_bus.cell_y;
    assign grid_input_valid = grid_bus.input_valid;
    assign grid_bus.output_valid = grid_output_valid;
    assign grid_bus.ready_for_input = grid_ready_for_input;
    assign grid_write_enable = grid_bus.write_enable;
    assign grid_write_occupied = grid_bus.write_occupied;
    assign grid_bus.read_occupied = grid_read_occupied;
 
    directed_energy_weapon #(.GRID_WIDTH_LOG2(GRID_WIDTH_LOG2), .GRID_HEIGHT_LOG2(GRID_HEIGHT_LOG2)) uut (
        .clk(clk),
        .rst_n(rst_n),
        .a(a),
        .b(b),
        .occupied(occupied),
        .input_valid(input_valid),
        .done(done),
        .grid_bus(grid_bus.client)
    );
endmodule

