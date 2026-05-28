#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::Path};

    use marlin::verilator::{VerilatorRuntime, VerilatorRuntimeOptions};
    use marlin::verilog::prelude::*;
    use rand::RngExt;
    use snafu::Whatever;

    const ADDR_WIDTH: u32 = 8;
    const DATA_WIDTH: u32 = 8;

    #[verilog(
        src = "src/fpga/verilog/test/wrappers/bram_wrapper.sv",
        name = "bram_wrapper",
        params = { ADDR_WIDTH: 8, DATA_WIDTH: 8 },
        includes = ["src/fpga/verilog/src/"]
    )]
    pub struct BramWrapper;

    impl<'ctx> BramWrapper<'ctx> {
        fn tick(&mut self) {
            self.clk = 1;
            self.eval();
            self.clk = 0;
            self.eval();
        }
    }

    fn make_runtime() -> Result<VerilatorRuntime, Whatever> {
        VerilatorRuntime::new2(
            "build",
            &["src/fpga/verilog/test/wrappers/bram_wrapper.sv"],
            &[Path::new("src/fpga/verilog/src/")],
            [],
            VerilatorRuntimeOptions::default(),
        )
    }

    #[test]
    #[snafu::report]
    fn test_read_write() -> Result<(), Whatever> {
        let runtime = make_runtime()?;
        let mut dut = runtime.create_model_simple::<BramWrapper>()?;

        dut.clk = 0;
        dut.eval();

        dut.bus_address = 0x01;
        dut.bus_write_data = 0x42;
        dut.bus_write_enable = 1;
        dut.tick();

        dut.bus_write_enable = 0;
        dut.tick();

        assert_eq!(dut.bus_read_data, 0x42);

        Ok(())
    }

    #[test]
    #[snafu::report]
    fn test_multiple_locations() -> Result<(), Whatever> {
        let runtime = make_runtime()?;
        let mut dut = runtime.create_model_simple::<BramWrapper>()?;

        dut.clk = 0;
        dut.eval();

        let mut rng = rand::rng();
        let mut data: HashMap<u8, u8> = HashMap::new();

        for _ in 0..20 {
            let address: u8 = rng.random_range(0..(1u32 << ADDR_WIDTH)) as u8;
            let value: u8 = rng.random_range(0..(1u32 << DATA_WIDTH)) as u8;

            data.insert(address, value);

            dut.bus_address = address;
            dut.bus_write_data = value;
            dut.bus_write_enable = 1;
            dut.tick();
        }

        dut.bus_write_enable = 0;

        for (&address, &expected) in &data {
            dut.bus_address = address;
            dut.tick();
            assert_eq!(
                dut.bus_read_data, expected,
                "addr {:#x}: expected {:#x}, got {:#x}",
                address, expected, dut.bus_read_data
            );
        }

        Ok(())
    }

    #[test]
    #[snafu::report]
    fn test_simultaneous_read_write() -> Result<(), Whatever> {
        let runtime = make_runtime()?;
        let mut dut = runtime.create_model_simple::<BramWrapper>()?;

        dut.clk = 0;
        dut.eval();

        let address: u8 = 0x10;
        let val1: u8 = 0xAA;
        let val2: u8 = 0xBB;

        dut.bus_address = address;
        dut.bus_write_data = val1;
        dut.bus_write_enable = 1;
        dut.tick();

        dut.bus_write_data = val2;
        dut.tick();

        assert_eq!(
            dut.bus_read_data, val1,
            "read-during-write should return old value"
        );

        dut.bus_write_enable = 0;
        dut.tick();

        assert_eq!(dut.bus_read_data, val2);

        Ok(())
    }
}
