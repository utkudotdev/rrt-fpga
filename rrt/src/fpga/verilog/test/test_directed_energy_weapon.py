import cocotb
from cocotb.triggers import FallingEdge
from util import gen_test_runner, generate_clock

GRID_WIDTH_LOG2 = 2
GRID_HEIGHT_LOG2 = 2

POINT_WIDTH = 32

CELL_WIDTH = 2**POINT_WIDTH // 2**GRID_WIDTH_LOG2
CELL_HEIGHT = 2**POINT_WIDTH // 2**GRID_HEIGHT_LOG2


class MockOccupancyGrid:
    def __init__(self, dut, cells: list[list[bool]]) -> None:
        self.dut = dut
        self.cells = cells

    def tick(self) -> None:
        self.dut.grid_ready_for_input.value = True

        if self.dut.grid_input_valid.value:
            if self.dut.grid_write_enable.value:
                self.dut.grid_output_valid.value = False
                self.cells[self.dut.grid_cell_y.value][
                    self.dut.grid_cell_x.value
                ] = self.dut.grid_write_occupied.value
            else:
                self.dut.grid_output_valid.value = True
                self.dut.grid_read_occupied.value = self.cells[
                    self.dut.grid_cell_y.value
                ][self.dut.grid_cell_x.value]


async def reset(dut):
    dut.rst_n.value = 0
    dut.a.value = 0
    dut.b.value = 0
    dut.grid_output_valid.value = 0
    dut.grid_ready_for_input.value = 0
    dut.grid_read_occupied.value = 0

    await FallingEdge(dut.clk)
    dut.rst_n.value = 1


async def get_occupied(dut, grid: MockOccupancyGrid, a, b) -> int:
    cocotb.log.info("get_occupied")

    while dut.done.value == 1:
        grid.tick()
        await FallingEdge(dut.clk)

    dut.a.value = a
    dut.b.value = b
    dut.input_valid.value = 1
    cocotb.log.info(a)
    cocotb.log.info(dut.a.value)
    cocotb.log.info(dut.input_valid.value)

    while dut.done.value == 0:
        grid.tick()
        await FallingEdge(dut.clk)

    return dut.occupied.value


def get_cell_center(cell: tuple[int, int]) -> tuple[int, int]:
    x, y = cell
    return x * CELL_WIDTH + (CELL_WIDTH // 2), y * CELL_HEIGHT + (CELL_HEIGHT // 2)


def convert_point_to_bits(point: tuple[int, int]) -> int:
    x, y = point
    return (x << POINT_WIDTH) | y


@cocotb.test()
async def test_empty(dut):
    cocotb.start_soon(generate_clock(dut))

    test_grid = [[False] * 2**GRID_WIDTH_LOG2] * 2**GRID_HEIGHT_LOG2

    grid = MockOccupancyGrid(dut, test_grid)

    await reset(dut)

    # TODO: more reasonable line
    a = convert_point_to_bits((0, 0))
    b = convert_point_to_bits((0, 0))

    # assert await get_occupied(dut, grid, a, b) == 0


@cocotb.test()
async def test_single_point(dut):
    cocotb.start_soon(generate_clock(dut))

    test_grid = [[False] * 2**GRID_WIDTH_LOG2] * 2**GRID_HEIGHT_LOG2
    test_grid[2][2] = True

    grid = MockOccupancyGrid(dut, test_grid)

    await reset(dut)

    # a = convert_point_to_bits((0, 0))
    # b = a

    # assert await get_occupied(dut, grid, a, b) == 0

    a = convert_point_to_bits(get_cell_center((2, 2)))
    b = a

    cocotb.log.info("START COOKED")

    assert await get_occupied(dut, grid, a, b) == 1


test_directed_energy_weapon = gen_test_runner(
    "test/wrappers/directed_energy_weapon_wrapper.sv",
    "test_directed_energy_weapon",
    "directed_energy_weapon_wrapper",
    parameters={
        "GRID_WIDTH_LOG2": GRID_WIDTH_LOG2,
        "GRID_HEIGHT_LOG2": GRID_HEIGHT_LOG2,
    },
)

if __name__ == "__main__":
    test_directed_energy_weapon()
