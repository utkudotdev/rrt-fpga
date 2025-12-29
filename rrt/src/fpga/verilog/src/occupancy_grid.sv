`ifndef OCCUPANCY_GRID_SV
`define OCCUPANCY_GRID_SV

`include "membus.sv"
`include "point.sv"
`include "cell_access_bus.sv"

module occupancy_grid #(
    parameter GRID_WIDTH_LOG2,
    parameter GRID_HEIGHT_LOG2
) (
    // Use memory clock
    input logic clk,
    input logic rst_n,

    cell_access_bus.server cell_bus,

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
    logic [GRID_WIDTH_LOG2 + GRID_HEIGHT_LOG2 - 1:0] linear_addr;
    assign linear_addr = {cell_bus.cell_y, cell_bus.cell_x};

    logic [ADDR_WIDTH-1:0] req_word_addr;
    logic [DATA_WIDTH_LOG2-1:0] req_bit_off;
    
    // Synthesis should handle optimization for power-of-2 widths
    assign req_word_addr = ADDR_WIDTH'(linear_addr / DATA_WIDTH);
    assign req_bit_off = DATA_WIDTH_LOG2'(linear_addr % DATA_WIDTH);

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
    always_ff @(posedge clk) begin
        if (!rst_n) begin
            state <= START_READ;
            mem.we <= '0;
            mem.addr <= '0;
            mem.w_data <= '0;
            req_bit_off_reg <= '0;
            w_occupied_reg <= '0;
            we_reg <= '0;
            cell_bus.vld_out <= '0;
            cell_bus.rdy <= '1;
        end else begin
            case (state)
                START_READ: begin
                    if (cell_bus.vld_in) begin
                        mem.addr <= req_word_addr;
                        mem.we <= '0;

                        we_reg <= cell_bus.we;
                        req_bit_off_reg <= req_bit_off;
                        w_occupied_reg <= cell_bus.w_occupied;

                        cell_bus.vld_out <= '0;
                        cell_bus.rdy <= '0;

                        state <= WAIT_READ;
                    end else begin
                        cell_bus.rdy <= '1;

                        state <= START_READ;
                    end
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

                    state <= START_READ;
                end
                FINISH_READ: begin
                    cell_bus.r_occupied <= mem.r_data[req_bit_off_reg];

                    cell_bus.vld_out <= '1;

                    state <= START_READ;
                end
            endcase
        end
    end

endmodule

`endif
