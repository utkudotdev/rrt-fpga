`ifndef POINT_SV
`define POINT_SV

`define POINT_BITS 32

typedef struct packed {
    logic [POINT_BITS-1:0] x;
    logic [POINT_BITS-1:0] y;
} point;

`endif
