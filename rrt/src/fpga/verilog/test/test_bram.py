import cocotb
from cocotb.triggers import FallingEdge, Timer
from runner_util import gen_test_runner


async def generate_clock(dut):
    """Generate clock pulses."""

    while True:
        dut.bus_clk.value = 0
        await Timer(1, unit="ns")
        dut.bus_clk.value = 1
        await Timer(1, unit="ns")


@cocotb.test()
async def test_nothing(dut):
    cocotb.start_soon(generate_clock(dut))

    await FallingEdge(dut.bus_clk)


test_bram = gen_test_runner(
    "test/wrappers/bram_wrapper.sv",
    "test_bram",
    "bram_wrapper",
    parameters={"ADDR_WIDTH": 8, "DATA_WIDTH": 8},
)


if __name__ == "__main__":
    test_bram()
