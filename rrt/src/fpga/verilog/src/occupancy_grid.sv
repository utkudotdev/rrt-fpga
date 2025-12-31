`ifndef OCCUPANCY_GRID_SV
`define OCCUPANCY_GRID_SV

`include "membus.sv"
`include "point.sv"

module occupancy_grid #(
    parameter GRID_WIDTH_LOG2,
    parameter GRID_HEIGHT_LOG2
) (
    // Use memory clock
    input logic clk,
    input logic rst_n,

    input logic [GRID_WIDTH_LOG2-1:0] cell_x,
    input logic [GRID_HEIGHT_LOG2-1:0] cell_y,

    input logic input_valid,
    output logic output_valid,
    output logic ready_for_input,

    input logic write_enable,
    input logic write_occupied,
    output logic read_occupied,

    memory_bus.client mem
);
    function automatic void point_to_cell(
        input point p,
        output logic [GRID_WIDTH_LOG2-1:0] cx,
        output logic [GRID_HEIGHT_LOG2-1:0] cy
    );
        cx = p.x[31 -: GRID_WIDTH_LOG2];
        cy = p.y[31 -: GRID_HEIGHT_LOG2];
    endfunction

    localparam DATA_WIDTH = mem.DATA_WIDTH;
    localparam ADDR_WIDTH = mem.ADDR_WIDTH;
    localparam DATA_WIDTH_LOG2 = $clog2(DATA_WIDTH);
    
    // Address calculation
    logic [GRID_WIDTH_LOG2 + GRID_HEIGHT_LOG2 - 1:0] linear_address;
    assign linear_address = {cell_y, cell_x};

    logic [ADDR_WIDTH-1:0] req_word_address;
    logic [DATA_WIDTH_LOG2-1:0] req_bit_off;
    
    // Synthesis should handle optimization for powrite_enabler-of-2 widths
    assign req_word_address = ADDR_WIDTH'(linear_address / DATA_WIDTH);
    assign req_bit_off = DATA_WIDTH_LOG2'(linear_address % DATA_WIDTH);

    logic [DATA_WIDTH_LOG2-1:0] req_bit_off_reg;
    logic write_occupied_reg;
    logic write_enable_reg;

    typedef enum logic [1:0] {
        START_READ,
        WAIT_READ,
        FINISH_READ,
        WRITE_BACK
    } state_t;

    state_t state;

    // TODO: I think the latency here can be lowrite_enabler?
    always_ff @(posedge clk) begin
        if (!rst_n) begin
            state <= START_READ;
            mem.write_enable <= '0;
            mem.address <= '0;
            mem.write_data <= '0;
            req_bit_off_reg <= '0;
            write_occupied_reg <= '0;
            write_enable_reg <= '0;
            output_valid <= '0;
            ready_for_input <= '1;
        end else begin
            case (state)
                START_READ: begin
                    if (input_valid) begin
                        mem.address <= req_word_address;
                        mem.write_enable <= '0;

                        write_enable_reg <= write_enable;
                        req_bit_off_reg <= req_bit_off;
                        write_occupied_reg <= write_occupied;

                        output_valid <= '0;
                        ready_for_input <= '0;

                        state <= WAIT_READ;
                    end else begin
                        ready_for_input <= '1;

                        state <= START_READ;
                    end
                end
                WAIT_READ: begin
                    // write_enable have to wait one cycle for the read data to actually become available.
                    if (write_enable_reg)
                        state <= WRITE_BACK;
                    else
                        state <= FINISH_READ;
                end
                WRITE_BACK: begin
                    mem.write_enable <= '1;

                    if (write_occupied_reg)
                        mem.write_data <= mem.read_data | (DATA_WIDTH'(1) << req_bit_off_reg);
                    else
                        mem.write_data <= mem.read_data & ~(DATA_WIDTH'(1) << req_bit_off_reg);

                    state <= START_READ;
                end
                FINISH_READ: begin
                    read_occupied <= mem.read_data[req_bit_off_reg];

                    output_valid <= '1;

                    state <= START_READ;
                end
            endcase
        end
    end

endmodule

module occupancy_grid_util #(
    parameter GRID_WIDTH_LOG2,
    parameter GRID_HEIGHT_LOG2
) ();
    function automatic void point_to_cell(
        input point p,
        output logic [GRID_WIDTH_LOG2-1:0] cx,
        output logic [GRID_HEIGHT_LOG2-1:0] cy
    );
        cx = p.x[31 -: GRID_WIDTH_LOG2];
        cy = p.y[31 -: GRID_HEIGHT_LOG2];
    endfunction
endmodule

`endif
