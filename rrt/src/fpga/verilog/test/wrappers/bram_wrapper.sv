`include "bram.sv"

module bram_wrapper #(
    parameter ADDR_WIDTH,
    parameter DATA_WIDTH
) (
    input logic clk,
    input logic [ADDR_WIDTH-1:0] bus_address,
    output logic [DATA_WIDTH-1:0] bus_read_data,
    input logic [DATA_WIDTH-1:0] bus_write_data,
    input logic bus_write_enable
);
    memory_bus #(
        .ADDR_WIDTH(ADDR_WIDTH), 
        .DATA_WIDTH(DATA_WIDTH)
    ) bus (
    );

    bram #(
        .ADDR_WIDTH(ADDR_WIDTH),
        .DATA_WIDTH(DATA_WIDTH)
    ) bram_inst (
        .clk(clk),
        .bus(bus.memory) 
    );

    always_comb begin
        bus.address = bus_address;
        bus_read_data = bus.read_data;
        bus.write_data = bus_write_data;
        bus.write_enable = bus_write_enable;
    end
endmodule
