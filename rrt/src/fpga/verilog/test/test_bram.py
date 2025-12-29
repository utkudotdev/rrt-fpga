import random

import cocotb
from cocotb.triggers import FallingEdge, Timer
from runner_util import gen_test_runner

ADDR_WIDTH = 8
DATA_WIDTH = 8


async def generate_clock(dut):
    """Generate clock pulses."""
    while True:
        dut.bus_clk.value = 0
        await Timer(1, unit="ns")
        dut.bus_clk.value = 1
        await Timer(1, unit="ns")


@cocotb.test()
async def test_read_write(dut):
    """Test basic read/write functionality."""
    cocotb.start_soon(generate_clock(dut))

    await FallingEdge(dut.bus_clk)

    # Write 0x42 to address 0x01
    dut.bus_addr.value = 0x01
    dut.bus_w_data.value = 0x42
    dut.bus_we.value = True

    # Wait for value to be written
    await FallingEdge(dut.bus_clk)
    dut.bus_we.value = False

    # Read back (address is still 0x01)
    await FallingEdge(dut.bus_clk)

    assert dut.bus_r_data.value == 0x42, f"Expected 0x42, got {dut.bus_r_data.value}"


@cocotb.test()
async def test_multiple_locations(dut):
    """Test writing and reading from multiple random locations."""
    cocotb.start_soon(generate_clock(dut))

    data_dict = {}

    await FallingEdge(dut.bus_clk)

    for _ in range(20):
        addr = random.randint(0, (1 << ADDR_WIDTH) - 1)
        data = random.randint(0, (1 << DATA_WIDTH) - 1)

        data_dict[addr] = data

        dut.bus_addr.value = addr
        dut.bus_w_data.value = data
        dut.bus_we.value = True
        await FallingEdge(dut.bus_clk)

    dut.bus_we.value = False

    for addr, expected_data in data_dict.items():
        dut.bus_addr.value = addr

        await FallingEdge(dut.bus_clk)

        assert (
            dut.bus_r_data.value == expected_data
        ), f"Addr {hex(addr)}: Expected {hex(expected_data)}, got {dut.bus_r_data.value}"


@cocotb.test()
async def test_simultaneous_read_write(dut):
    """
    Test simultaneous read and write (Read-During-Write).
    Expectation: The read operation returns the OLD value at the address
    while the new value is being written (Read-Before-Write).
    """
    cocotb.start_soon(generate_clock(dut))

    await FallingEdge(dut.bus_clk)

    addr = 0x10
    val1 = 0xAA
    val2 = 0xBB

    # 1. Initialize address with val1
    dut.bus_addr.value = addr
    dut.bus_w_data.value = val1
    dut.bus_we.value = True
    await FallingEdge(dut.bus_clk)

    # 2. Setup Simultaneous Write (val2) and Read
    # We are currently at FallingEdge.
    # We keep we=1, addr=addr. Change w_data to val2.
    dut.bus_w_data.value = val2

    # Wait for the clock edge where both read and write happen
    await FallingEdge(dut.bus_clk)

    # The read output (r_data) should reflect the value BEFORE the write (val1)
    # because of non-blocking assignment order in RTL.
    read_val = dut.bus_r_data.value
    assert (
        read_val == val1
    ), f"Read-During-Write failed. Expected old value {hex(val1)}, got {read_val}. (New val was {hex(val2)})"

    # 3. Verify that the write actually happened
    dut.bus_we.value = False
    await FallingEdge(dut.bus_clk)  # One more cycle to read the NEW value

    read_val_new = dut.bus_r_data.value
    assert (
        read_val_new == val2
    ), f"Subsequent read failed. Expected new value {hex(val2)}, got {read_val_new}"


test_bram = gen_test_runner(
    "test/wrappers/bram_wrapper.sv",
    "test_bram",
    "bram_wrapper",
    parameters={"ADDR_WIDTH": ADDR_WIDTH, "DATA_WIDTH": DATA_WIDTH},
)


if __name__ == "__main__":
    test_bram()
