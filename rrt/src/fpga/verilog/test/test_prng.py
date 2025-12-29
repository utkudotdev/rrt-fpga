import cocotb
from cocotb.triggers import FallingEdge, Timer
from runner_util import gen_test_runner


async def generate_clock(dut):
    """Generate clock pulses."""

    while True:
        dut.clk.value = 0
        await Timer(1, unit="ns")
        dut.clk.value = 1
        await Timer(1, unit="ns")


@cocotb.test()
async def test_en_low(dut):
    cocotb.start_soon(generate_clock(dut))

    await FallingEdge(dut.clk)
    dut.rst_n.value = 0
    dut.seed.value = 123
    dut.en.value = 0

    await FallingEdge(dut.clk)
    dut.rst_n.value = 1

    await FallingEdge(dut.clk)
    val = dut.out.value
    assert val == 123

    for _ in range(10):
        await FallingEdge(dut.clk)
        assert dut.out.value == val


@cocotb.test()
async def test_en_high_changes(dut):
    cocotb.start_soon(generate_clock(dut))

    await FallingEdge(dut.clk)
    dut.rst_n.value = 0
    dut.seed.value = 123
    dut.en.value = 1

    await FallingEdge(dut.clk)
    dut.rst_n.value = 1

    last_val = dut.out.value

    for _ in range(10):
        await FallingEdge(dut.clk)

        curr_val = dut.out.value
        cocotb.log.info("PRNG current value is %d", curr_val)

        assert curr_val != last_val
        last_val = curr_val


@cocotb.test()
async def test_reset_seed(dut):
    cocotb.start_soon(generate_clock(dut))

    await FallingEdge(dut.clk)

    dut.rst_n.value = 1
    dut.en.value = 0
    dut.seed.value = 10

    await FallingEdge(dut.clk)

    dut.rst_n.value = 0

    await FallingEdge(dut.clk)

    assert dut.out.value == 10


test_prng = gen_test_runner("src/prng.sv", "test_prng", "prng64")

if __name__ == "__main__":
    test_prng()
