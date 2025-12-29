`include "membus.sv"

module bram #(
    parameter ADDR_WIDTH,
    parameter DATA_WIDTH
) (memory_bus.memory bus);
    logic [DATA_WIDTH-1:0] mem_array [0:(1'b1<<ADDR_WIDTH)-1];

    always_ff @(posedge bus.clk) begin
        if (bus.we) begin
            mem_array[bus.addr] <= bus.w_data;
        end

        bus.r_data <= mem_array[bus.addr];
    end
endmodule
