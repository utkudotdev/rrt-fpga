import random
import cocotb
from cocotb.triggers import FallingEdge, Timer
from runner_util import gen_test_runner

GRID_WIDTH_LOG2 = 4
GRID_HEIGHT_LOG2 = 4
DATA_WIDTH = 8
ADDR_WIDTH = 8

# State Enum mapping
STATE_START = 0
STATE_WAIT = 1
STATE_FINISH = 2
STATE_WRITE = 3

async def generate_clock(dut):
    """Generate clock pulses."""
    while True:
        dut.clk.value = 0
        await Timer(1, unit="ns")
        dut.clk.value = 1
        await Timer(1, unit="ns")

async def reset(dut):
    dut.rst_n.value = 0
    await FallingEdge(dut.clk)
    dut.rst_n.value = 1
    await FallingEdge(dut.clk)

async def wait_for_start(dut):
    """Wait until the state machine is in START state."""
    while dut.uut.state.value != STATE_START:
        await FallingEdge(dut.clk)

@cocotb.test()
async def test_read_write_single(dut):
    """Test basic read/write functionality for a single cell."""
    cocotb.start_soon(generate_clock(dut))
    
    dut.we.value = 0
    dut.cell_x_in.value = 0
    dut.cell_y_in.value = 0
    dut.w_occupied.value = 0
    
    await reset(dut)
    await wait_for_start(dut)
    
    # Write 1 to (1, 1)
    dut.cell_x_in.value = 1
    dut.cell_y_in.value = 1
    dut.w_occupied.value = 1
    dut.we.value = 1
    
    # 3 cycles for Write: START->WAIT, WAIT->WRITE, WRITE->START
    await FallingEdge(dut.clk) # START -> WAIT
    await FallingEdge(dut.clk) # WAIT -> WRITE
    await FallingEdge(dut.clk) # WRITE -> START
    
    dut.we.value = 0
    
    # We should be back in START now (or transitioning to it)
    # Actually, previous FallingEdge was when state BECAME START (transition from WRITE).
    # So we are in START.
    
    # Read back (1, 1)
    dut.cell_x_in.value = 1
    dut.cell_y_in.value = 1
    dut.we.value = 0
    
    # 3 cycles for Read: START->WAIT, WAIT->FINISH, FINISH->START
    await FallingEdge(dut.clk) # START -> WAIT
    await FallingEdge(dut.clk) # WAIT -> FINISH
    await FallingEdge(dut.clk) # FINISH -> START. r_occupied updated.
    
    assert dut.r_occupied.value == 1, f"Expected (1,1) to be occupied, got {dut.r_occupied.value}"
    
    # Read another address (2, 2) - should be 0
    dut.cell_x_in.value = 2
    dut.cell_y_in.value = 2
    
    await FallingEdge(dut.clk)
    await FallingEdge(dut.clk)
    await FallingEdge(dut.clk)
    
    assert dut.r_occupied.value == 0, "Expected (2,2) to be empty"

@cocotb.test()
async def test_random_access(dut):
    """Test random read/write access."""
    cocotb.start_soon(generate_clock(dut))
    await reset(dut)
    await wait_for_start(dut)
    
    expected_state = {}
    
    # Perform 50 random writes
    for _ in range(50):
        x = random.randint(0, (1 << GRID_WIDTH_LOG2) - 1)
        y = random.randint(0, (1 << GRID_HEIGHT_LOG2) - 1)
        val = random.randint(0, 1)
        
        expected_state[(x, y)] = val
        
        dut.cell_x_in.value = x
        dut.cell_y_in.value = y
        dut.w_occupied.value = val
        dut.we.value = 1
        
        await FallingEdge(dut.clk)
        await FallingEdge(dut.clk)
        await FallingEdge(dut.clk)
        
    dut.we.value = 0
    
    # Check modified cells
    for (x, y), val in expected_state.items():
        dut.cell_x_in.value = x
        dut.cell_y_in.value = y
        
        await FallingEdge(dut.clk)
        await FallingEdge(dut.clk)
        await FallingEdge(dut.clk)
        
        assert dut.r_occupied.value == val, f"Mismatch at ({x},{y}). Expected {val}, got {dut.r_occupied.value}"

    # Check some unmodified cells
    for _ in range(20):
        x = random.randint(0, (1 << GRID_WIDTH_LOG2) - 1)
        y = random.randint(0, (1 << GRID_HEIGHT_LOG2) - 1)
        if (x, y) not in expected_state:
            dut.cell_x_in.value = x
            dut.cell_y_in.value = y
            
            await FallingEdge(dut.clk)
            await FallingEdge(dut.clk)
            await FallingEdge(dut.clk)
            
            assert dut.r_occupied.value == 0, f"Expected 0 at ({x},{y}), got {dut.r_occupied.value}"

test_occupancy_grid = gen_test_runner(
    "test/wrappers/occupancy_grid_wrapper.sv",
    "test_occupancy_grid",
    "occupancy_grid_wrapper",
    parameters={
        "GRID_WIDTH_LOG2": GRID_WIDTH_LOG2,
        "GRID_HEIGHT_LOG2": GRID_HEIGHT_LOG2,
        "DATA_WIDTH": DATA_WIDTH,
        "ADDR_WIDTH": ADDR_WIDTH
    },
)

if __name__ == "__main__":
    test_occupancy_grid()
