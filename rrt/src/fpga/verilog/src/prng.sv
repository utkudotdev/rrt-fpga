module prng64 (
    input wire clk,
    input wire rst_n,
    input wire en,
    input wire [63:0] seed,
    output logic [63:0] out
);
    always_ff @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            out <= seed;
        end else if (en) begin
            // Algorithm source: https://en.wikipedia.org/wiki/Xorshift#Example_implementation
            logic [63:0] x0;
            logic [63:0] x1;
            logic [63:0] x2;
            logic [63:0] x3;

            x0 = out;
            x1 = x0 ^ (x0 << 13);
            x2 = x1 ^ (x1 >> 7);
            x3 = x2 ^ (x2 << 17);

            out <= x3;
        end
    end
endmodule
