#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::Path};

    use marlin::verilator::{VerilatorRuntime, VerilatorRuntimeOptions};
    use marlin::verilog::prelude::*;
    use rand::RngExt;
    use snafu::Whatever;

    const GRID_WIDTH_LOG2: u32 = 4;
    const GRID_HEIGHT_LOG2: u32 = 4;

    #[verilog(
        src = "src/fpga/verilog/test/wrappers/occupancy_grid_wrapper.sv",
        name = "occupancy_grid_wrapper",
        params = { GRID_WIDTH_LOG2: 4, GRID_HEIGHT_LOG2: 4, DATA_WIDTH: 8, ADDR_WIDTH: 8 },
        includes = ["src/fpga/verilog/src/"]
    )]
    pub struct OccupancyGridWrapper;

    impl<'ctx> OccupancyGridWrapper<'ctx> {
        fn tick(&mut self) {
            self.clk = 1;
            self.eval();
            self.clk = 0;
            self.eval();
        }

        fn reset(&mut self) {
            self.rst_n = 0;
            self.input_valid = 0;
            self.write_enable = 0;
            self.cell_x = 0;
            self.cell_y = 0;
            self.write_occupied = 0;
            self.clk = 0;
            self.eval();

            self.tick();
            self.rst_n = 1;
            self.tick();
        }

        fn wait_for_ready(&mut self) {
            while self.ready_for_input == 0 {
                self.tick();
            }
        }

        fn wait_for_output(&mut self) {
            while self.output_valid == 0 {
                self.tick();
            }
        }

        fn write_cell(&mut self, x: u8, y: u8, val: u8) {
            self.wait_for_ready();
            self.cell_x = x;
            self.cell_y = y;
            self.write_occupied = val;
            self.write_enable = 1;
            self.input_valid = 1;
            self.tick();
            self.input_valid = 0;
            self.write_enable = 0;
        }

        fn read_cell(&mut self, x: u8, y: u8) -> u8 {
            self.wait_for_ready();
            self.cell_x = x;
            self.cell_y = y;
            self.write_enable = 0;
            self.input_valid = 1;
            self.tick();
            self.input_valid = 0;
            self.wait_for_output();
            self.read_occupied
        }
    }

    fn make_runtime() -> Result<VerilatorRuntime, Whatever> {
        VerilatorRuntime::new2(
            "build",
            &["src/fpga/verilog/test/wrappers/occupancy_grid_wrapper.sv"],
            &[Path::new("src/fpga/verilog/src/")],
            [],
            VerilatorRuntimeOptions::default(),
        )
    }

    #[test]
    #[snafu::report]
    fn test_read_write_single() -> Result<(), Whatever> {
        let runtime = make_runtime()?;
        let mut dut = runtime.create_model_simple::<OccupancyGridWrapper>()?;

        dut.reset();
        dut.write_cell(1, 1, 1);
        let val = dut.read_cell(1, 1);
        assert_eq!(val, 1, "expected (1,1) occupied, got {}", val);

        Ok(())
    }

    #[test]
    #[snafu::report]
    fn test_random_access() -> Result<(), Whatever> {
        let runtime = make_runtime()?;
        let mut dut = runtime.create_model_simple::<OccupancyGridWrapper>()?;

        dut.reset();

        let mut rng = rand::rng();
        let mut expected: HashMap<(u8, u8), u8> = HashMap::new();

        for _ in 0..50 {
            let x: u8 = rng.random_range(0..(1u32 << GRID_WIDTH_LOG2)) as u8;
            let y: u8 = rng.random_range(0..(1u32 << GRID_HEIGHT_LOG2)) as u8;
            let val: u8 = rng.random_range(0..2u32) as u8;

            expected.insert((x, y), val);
            dut.write_cell(x, y, val);
        }

        for (&(x, y), &val) in &expected {
            let read = dut.read_cell(x, y);
            assert_eq!(
                read, val,
                "mismatch at ({},{}): expected {}, got {}",
                x, y, val, read
            );
        }

        Ok(())
    }
}
