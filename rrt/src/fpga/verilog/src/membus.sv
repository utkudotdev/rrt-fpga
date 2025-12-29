interface memory_bus #(
    parameter ADDR_WIDTH,
    parameter DATA_WIDTH
) (input logic clk);
    logic [ADDR_WIDTH-1:0] addr;
    logic [DATA_WIDTH-1:0] r_data;
    logic [DATA_WIDTH-1:0] w_data;
    logic we;

    modport memory (input clk, addr, w_data, we, output r_data);
    modport client (input clk, r_data, output addr, w_data, we);
endinterface
