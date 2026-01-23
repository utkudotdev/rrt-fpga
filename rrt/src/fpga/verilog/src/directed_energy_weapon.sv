`ifndef DIRECTED_ENERGY_WEAPON_SV
`define DIRECTED_ENERGY_WEAPON_SV

`include "occupancy_grid.sv"
`include "point.sv"

module directed_energy_weapon #(
    parameter GRID_WIDTH_LOG2,
    parameter GRID_HEIGHT_LOG2,
    parameter T_BITS
) (
    input logic clk,
    input logic rst_n,

    input point_t a,
    input point_t b,

    output logic occupied,

    input logic input_valid,
    output logic output_valid,

    // Grid interface
    output logic [GRID_WIDTH_LOG2-1:0] grid_cell_x,
    output logic [GRID_HEIGHT_LOG2-1:0] grid_cell_y,

    input logic grid_read_occupied,

    output logic grid_input_valid,
    input logic grid_output_valid,
);
    // GRID_CELL_WIDTH = 2^(POINT_BITS) / GRID_WIDTH
    //                 = 2^(POINT_BITS - GRID_WIDTH_LOG2)
    //
    // log2(GRID_CELL_WIDTH) = POINT_BITS - GRID_WIDTH_LOG2
    localparam GRID_CELL_WIDTH_LOG2 = POINT_BITS - GRID_WIDTH_LOG2;
    localparam GRID_CELL_HEIGHT_LOG2 = POINT_BITS - GRID_HEIGHT_LOG2;

    occupancy_grid_util#(GRID_WIDTH_LOG2, GRID_HEIGHT_LOG2) grid_util();

    typedef enum logic [1:0] {
        IDLE,
        TRACING
    } state_t;

    state_t state;

    point_diff_t delta;

    logic [GRID_WIDTH_LOG2-1:0] current_cell_x;
    logic [GRID_HEIGHT_LOG2-1:0] current_cell_y;

    // Need to have one extra bit to prevent overflow
    logic [GRID_WIDTH_LOG2:0] next_int_cell_x;
    logic [GRID_HEIGHT_LOG2:0] next_int_cell_y;
    point_t next_intersection;
    point_diff_t next_int_delta;
    
    logic next_x_beyond_end;
    logic next_y_beyond_end;

    logic signed [POINT_MULT_BITS:0] intersection_t_diff;

    always_comb begin
        // Note that a and b must stay valid throughout the operation
        delta = point_sub(b, a);

        next_int_cell_x = delta.x > 0 ? current_cell_x + 1' : current_cell_x;
        next_int_cell_y = delta.y > 0 ? current_cell_y + 1' : current_cell_y;

        next_intersection.x = next_int_cell_x << GRID_CELL_WIDTH_LOG2;
        next_intersection.y = next_int_cell_y << GRID_CELL_HEIGHT_LOG2;

        next_x_beyond_end = delta.x > 0 ? (next_intersection.x >= b.x) : (next_intersection.x <= b.x);
        next_y_beyond_end = delta.y > 0 ? (next_intersection.y >= b.y) : (next_intersection.y <= b.y);

        // We now need to compute whether we will arrive at the x or y intersection first.
        // Our line can be parameterized as f(t) = a + t * delta, where t: [0, 1].
        // We want to find the t until next_intersection.x and next_intersection.y.
        // next_int = a + t * delta --> t = (next_int - a) / delta
        //
        // That division is no good! But we don't need to know t exactly for x or y.
        // We just need to know which one is bigger so we can move to the correct cell!
        //
        // t.x = (next_int.x - a.x) / delta.x, t.y = (next_int.y - a.y) / delta.y
        // t.x = k.x / delta.x                 t.y = k.y / delta.y 
        // t.x >? t.y
        // k.x / delta.x >? k.y / delta.y
        // k.x * delta.y >? k.y * delta.x
        // 
        // So no division is required. We can get away with just DSP slices.

        next_int_delta = point_sub(next_intersection, a);

        intersection_t_diff = POINT_MULT_BITS'(next_int_delta.x) * POINT_MULT_BITS'(delta.y) 
                            - POINT_MULT_BITS'(next_int_delta.y) * POINT_MULT_BITS'(delta.x);
    end

    always_ff @(posedge clk) begin
        if (!rst_n) begin
            state <= IDLE;
            ready_for_input <= '1;
            output_valid <= '0;
            current_cell_x <= '0;
            current_cell_y <= '0;
            grid_input_valid <= '0;
        end else begin
            case (state)
                IDLE: begin
                    if (input_valid) begin
                        ready_for_input <= '0;
                        output_valid <= '0;
                        grid_util.point_to_cell(a, current_cell_x, current_cell_y);
                        state <= TRACING;
                    end else begin
                        ready_for_input <= '1;
                        state <= IDLE;
                    end
                end
                TRACING: begin
                    if (!grid_output_valid) begin
                        // Wait for the occupancy grid to give us our value
                        grid_cell_x <= current_cell_x;
                        grid_cell_y <= current_cell_y;
                        grid_input_valid <= '1;
                    end else begin
                        if (grid_read_occupied) begin
                            // If the grid is occupied at the current cell, we're done
                            occupied <= '1;
                            output_valid <= '1;
                            state <= IDLE;
                        end else if (next_x_beyond_end && next_y_beyond_end) begin
                            // We made it to the end, not occupied 
                            occupied <= '0;
                            output_valid <= '1;
                            state <= IDLE;
                        end else begin
                            // We need to go to the next cell
                            // TODO: delta.y = 0 still works?
                            if (intersection_t_diff > 0)
                                current_cell_y <= current_cell_y + (delta.y > 0 ? 1 : -1);
                            else if (intersection_t_diff < 0)
                                current_cell_x <= current_cell_x + (delta.x > 0 ? 1 : -1);
                            else begin
                                // rare case, t to x intersection == t to y intersection
                                // TODO: this case actually sucks because we need to do another memory access
                                // to figure out which path we need to take. for now I'm just gonna increment x
                                current_cell_x <= current_cell_x + (delta.x > 0 ? 1 : -1);
                            end
                        end
                    end
                end
            endcase
        end
    end
endmodule

`endif
