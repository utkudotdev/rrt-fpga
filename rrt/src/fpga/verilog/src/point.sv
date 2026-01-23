`ifndef POINT_SV
`define POINT_SV

// TODO: maybe we should refactor this so that we can just use one type everywhere.
// And adjust the resolution accordingly so nothing really overflows.
`define POINT_BITS 32
`define POINT_MULT_BITS 64

typedef struct packed {
    logic [POINT_BITS-1:0] x;
    logic [POINT_BITS-1:0] y;
} point_t;

typedef struct packed {
    logic signed [POINT_BITS:0] x;
    logic signed [POINT_BITS:0] y;
} point_diff_t;

function point_diff_t point_sub (input point_t a, b);
	begin
        point_sub.x = signed'({1'b0, a.x}) - signed'({1'b0, b.x});
        point_sub.y = signed'({1'b0, a.y}) - signed'({1'b0, b.y});
	end
endfunction

`endif
