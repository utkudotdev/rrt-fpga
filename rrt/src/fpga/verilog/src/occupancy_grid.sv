`ifndef OCCUPANCY_GRID_SV
`define OCCUPANCY_GRID_SV

`include "mem.sv"
`include "point.sv"

module occupancy_grid #(
    parameter GRID_WIDTH_LOG2,
    parameter GRID_HEIGHT_LOG2
) (
    // Use memory clock
    input logic rst_n,

    input logic [GRID_WIDTH_LOG2-1:0] cell_x_in,
    input logic [GRID_HEIGHT_LOG2-1:0] cell_y_in,

    input logic we,
    input logic w_occupied,
    output logic r_occupied,

    memory_bus.client mem
);
    localparam DATA_WIDTH = $bits(mem.r_data);
    localparam ADDR_WIDTH = $bits(mem.addr);
    localparam DATA_WIDTH_LOG2 = $clog2(DATA_WIDTH);
    
    // Address calculation
    logic [GRID_WIDTH_LOG2 + GRID_HEIGHT_LOG2 - 1:0] linear_addr;
    assign linear_addr = {cell_y_in, cell_x_in};

    logic [ADDR_WIDTH-1:0] req_word_addr;
    logic [DATA_WIDTH_LOG2-1:0] req_bit_off;
    
    // Synthesis should handle optimization for power-of-2 widths
    assign req_word_addr = linear_addr / DATA_WIDTH;
    assign req_bit_off = linear_addr % DATA_WIDTH;

    logic [DATA_WIDTH_LOG2-1:0] req_bit_off_reg;
    logic w_occupied_reg;
    logic we_reg;

    typedef enum logic [1:0] {
        START_READ,
        WAIT_READ,
        FINISH_READ,
        WRITE_BACK
    } state_t;

    state_t state;

    // TODO: I think the latency here can be lower?
    always_ff @(posedge mem.clk) begin
        if (!rst_n) begin
            state <= START;
            mem.we <= '0;
            mem.addr <= '0;
            mem.w_data <= '0;
            req_bit_off_reg <= '0;
            w_occupied_reg <= '0;
            we_reg <= '0;
        end else begin
            case (state)
                START: begin
                    mem.addr <= req_word_addr;
                    mem.we <= '0;
                    we_reg <= we;
                    req_bit_off_reg <= req_bit_off;
                    w_occupied_reg <= w_occupied;
                    state <= WAIT_READ;
                end
                WAIT_READ: begin
                    // We have to wait one cycle for the read data to actually become available.
                    if (we_reg)
                        state <= WRITE_BACK;
                    else
                        state <= FINISH_READ;
                end
                WRITE_BACK: begin
                    mem.we <= '1;

                    if (w_occupied_reg)
                        mem.w_data <= mem.r_data | (DATA_WIDTH'(1) << req_bit_off_reg);
                    else
                        mem.w_data <= mem.r_data & ~(DATA_WIDTH'(1) << req_bit_off_reg);
                        
                    state <= START;
                end
                FINISH_READ: begin
                    r_occupied <= mem.r_data[req_bit_off_reg];

                    state <= START;
                end
            endcase
        end
    end

endmodule

`endif
