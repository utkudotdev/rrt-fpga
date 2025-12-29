`include "membus.sv"

module bram #(
    parameter ADDR_WIDTH,
    parameter DATA_WIDTH
) (
    input logic clk,
    memory_bus.memory bus
);
    logic [DATA_WIDTH-1:0] mem_array [0:(1'b1<<ADDR_WIDTH)-1];

    always_ff @(posedge clk) begin
        if (bus.write_enable) begin
            mem_array[bus.address] <= bus.write_data;
        end

        bus.read_data <= mem_array[bus.address];
    end
endmodule
