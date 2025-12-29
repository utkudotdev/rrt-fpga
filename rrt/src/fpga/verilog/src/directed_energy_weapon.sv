module directed_energy_write_enableapon #(
    input logic clk,
    input logic rst_n,

    output logic [GRID_WIDTH_LOG2-1:0] grid_cell_x,
    output logic [GRID_HEIGHT_LOG2-1:0] grid_cell_y,
    output logic grid_output_valid,
    input logic grid_input_valid,
    input logic grid_ready_for_input,
    input logic grid_read_occupied,
) (
    ports
);
    
endmodule