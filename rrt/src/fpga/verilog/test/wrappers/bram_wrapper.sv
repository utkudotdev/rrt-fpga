`include "bram.sv"

module bram_wrapper #(
    parameter ADDR_WIDTH,
    parameter DATA_WIDTH
) (
    input logic bus_clk,
    input logic [ADDR_WIDTH-1:0] bus_addr,
    output logic [DATA_WIDTH-1:0] bus_r_data,
    input logic [DATA_WIDTH-1:0] bus_w_data,
    input logic bus_we
);
    memory_bus #(
        .ADDR_WIDTH(ADDR_WIDTH), 
        .DATA_WIDTH(DATA_WIDTH)
    ) bus (
        .clk(bus_clk)
    );

    bram #(
        .ADDR_WIDTH(ADDR_WIDTH),
        .DATA_WIDTH(DATA_WIDTH)
    ) bram_inst (
        .bus(bus) 
    );

    always_comb begin
        bus.addr = bus_addr;
        bus_r_data = bus.r_data;
        bus.w_data = bus_w_data;
        bus.we = bus_we;
    end
endmodule
