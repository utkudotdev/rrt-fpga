import random

import cocotb
from cocotb.triggers import FallingEdge, Timer
from runner_util import gen_test_runner

GRID_WIDTH_LOG2 = 4
GRID_HEIGHT_LOG2 = 4
DATA_WIDTH = 8
ADDR_WIDTH = 8


async def generate_clock(dut):
    """Generate clock pulses."""
    while True:
        dut.clk.value = 0
        await Timer(1, unit="ns")
        dut.clk.value = 1
        await Timer(1, unit="ns")


async def reset(dut):
    dut.rst_n.value = 0
    dut.input_valid.value = 0
    dut.write_enable.value = 0
    dut.cell_x.value = 0
    dut.cell_y.value = 0
    dut.write_occupied.value = 0

    await FallingEdge(dut.clk)
    dut.rst_n.value = 1
    await FallingEdge(dut.clk)


async def wait_for_ready_for_input(dut):
    """Wait until the module is ready to accept a new input."""
    while dut.ready_for_input.value == 0:
        await FallingEdge(dut.clk)


async def wait_for_output_valid(dut):
    """Wait until the module output is ready."""
    while dut.output_valid.value == 0:
        await FallingEdge(dut.clk)


async def write_cell(dut, x, y, val):
    await wait_for_ready_for_input(dut)

    dut.cell_x.value = x
    dut.cell_y.value = y
    dut.write_occupied.value = val
    dut.write_enable.value = 1
    dut.input_valid.value = 1

    await FallingEdge(dut.clk)
    dut.input_valid.value = 0
    dut.write_enable.value = 0


async def read_cell(dut, x, y):
    await wait_for_ready_for_input(dut)

    dut.cell_x.value = x
    dut.cell_y.value = y
    dut.write_enable.value = 0
    dut.input_valid.value = 1

    await FallingEdge(dut.clk)
    dut.input_valid.value = 0

    await wait_for_output_valid(dut)

    return dut.read_occupied.value


@cocotb.test()
async def test_read_write_single(dut):
    """Test basic read/write functionality for a single cell."""
    cocotb.start_soon(generate_clock(dut))

    await reset(dut)

    # Write 1 to (1, 1)
    await write_cell(dut, 1, 1, 1)

    # Read back (1, 1)
    val = await read_cell(dut, 1, 1)
    assert val == 1, f"Expected (1,1) to be occupied, got {val}"


@cocotb.test()
async def test_random_access(dut):
    """Test random read/write access."""
    cocotb.start_soon(generate_clock(dut))
    await reset(dut)

    expected_state = {}

    # Perform 50 random writes
    for _ in range(50):
        x = random.randint(0, (1 << GRID_WIDTH_LOG2) - 1)
        y = random.randint(0, (1 << GRID_HEIGHT_LOG2) - 1)
        val = random.randint(0, 1)

        expected_state[(x, y)] = val

        await write_cell(dut, x, y, val)

    # Check modified cells
    for (x, y), val in expected_state.items():
        read_val = await read_cell(dut, x, y)
        assert read_val == val, f"Mismatch at ({x},{y}). Expected {val}, got {read_val}"


test_occupancy_grid = gen_test_runner(
    "test/wrappers/occupancy_grid_wrapper.sv",
    "test_occupancy_grid",
    "occupancy_grid_wrapper",
    parameters={
        "GRID_WIDTH_LOG2": GRID_WIDTH_LOG2,
        "GRID_HEIGHT_LOG2": GRID_HEIGHT_LOG2,
        "DATA_WIDTH": DATA_WIDTH,
        "ADDR_WIDTH": ADDR_WIDTH,
    },
)

if __name__ == "__main__":
    test_occupancy_grid()
