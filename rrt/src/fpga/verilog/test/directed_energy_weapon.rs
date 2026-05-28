#[cfg(test)]
mod tests {
    use std::path::Path;

    use marlin::verilator::{VerilatorRuntime, VerilatorRuntimeOptions};
    use marlin::verilog::prelude::*;
    use snafu::Whatever;

    const GRID_WIDTH_LOG2: u32 = 2;
    const GRID_HEIGHT_LOG2: u32 = 2;

    const POINT_WIDTH: u32 = 32;

    const CELL_WIDTH: u64 = (1u64 << POINT_WIDTH) >> GRID_WIDTH_LOG2;
    const CELL_HEIGHT: u64 = (1u64 << POINT_WIDTH) >> GRID_HEIGHT_LOG2;

    #[verilog(
        src = "src/fpga/verilog/test/wrappers/directed_energy_weapon_wrapper.sv",
        name = "directed_energy_weapon_wrapper",
        params = { GRID_WIDTH_LOG2: 2, GRID_HEIGHT_LOG2: 2 },
        includes = ["src/fpga/verilog/src/"]
    )]
    pub struct DewWrapper;

    impl<'ctx> DewWrapper<'ctx> {
        fn tick(&mut self) {
            self.clk = 1;
            self.eval();
            self.clk = 0;
            self.eval();
        }
    }

    struct MockGrid {
        cells: Vec<Vec<bool>>,
    }

    impl MockGrid {
        fn new() -> Self {
            let w = 1usize << GRID_WIDTH_LOG2;
            let h = 1usize << GRID_HEIGHT_LOG2;
            Self {
                cells: vec![vec![false; w]; h],
            }
        }

        fn tick(&mut self, dut: &mut DewWrapper) {
            dut.grid_ready_for_input = 1;

            if dut.grid_input_valid != 0 {
                let x = dut.grid_cell_x as usize;
                let y = dut.grid_cell_y as usize;
                if dut.grid_write_enable != 0 {
                    dut.grid_output_valid = 0;
                    self.cells[y][x] = dut.grid_write_occupied != 0;
                } else {
                    dut.grid_output_valid = 1;
                    dut.grid_read_occupied = if self.cells[y][x] { 1 } else { 0 };
                }
            }
        }
    }

    fn make_runtime() -> Result<VerilatorRuntime, Whatever> {
        VerilatorRuntime::new2(
            "build",
            &["src/fpga/verilog/test/wrappers/directed_energy_weapon_wrapper.sv"],
            &[Path::new("src/fpga/verilog/src/")],
            [],
            VerilatorRuntimeOptions::default(),
        )
    }

    fn reset(dut: &mut DewWrapper) {
        dut.rst_n = 0;
        dut.a = 0;
        dut.b = 0;
        dut.input_valid = 0;
        dut.grid_output_valid = 0;
        dut.grid_ready_for_input = 0;
        dut.grid_read_occupied = 0;
        dut.clk = 0;
        dut.eval();
        dut.tick();
        dut.rst_n = 1;
        dut.tick();
    }

    fn point_bits(x: u64, y: u64) -> u64 {
        (x << POINT_WIDTH) | y
    }

    fn cell_center(cx: u64, cy: u64) -> (u64, u64) {
        (
            cx * CELL_WIDTH + CELL_WIDTH / 2,
            cy * CELL_HEIGHT + CELL_HEIGHT / 2,
        )
    }

    fn get_occupied(dut: &mut DewWrapper, grid: &mut MockGrid, a: u64, b: u64) -> u8 {
        while dut.done == 1 {
            grid.tick(dut);
            dut.tick();
        }

        dut.a = a;
        dut.b = b;
        dut.input_valid = 1;

        while dut.done == 0 {
            grid.tick(dut);
            dut.tick();
        }

        dut.occupied
    }

    #[test]
    #[snafu::report]
    fn test_empty() -> Result<(), Whatever> {
        let runtime = make_runtime()?;
        let mut dut = runtime.create_model_simple::<DewWrapper>()?;

        let _grid = MockGrid::new();
        reset(&mut dut);

        let _a = point_bits(0, 0);
        let _b = point_bits(0, 0);
        // The python test has the assertion commented out; mirror that.

        Ok(())
    }

    #[test]
    #[snafu::report]
    fn test_single_point() -> Result<(), Whatever> {
        let runtime = make_runtime()?;
        let mut dut = runtime.create_model_simple::<DewWrapper>()?;

        let mut grid = MockGrid::new();
        grid.cells[2][2] = true;

        reset(&mut dut);

        let (cx, cy) = cell_center(2, 2);
        let a = point_bits(cx, cy);
        let b = a;

        assert_eq!(get_occupied(&mut dut, &mut grid, a, b), 1);

        Ok(())
    }
}
