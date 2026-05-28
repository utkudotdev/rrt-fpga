#[cfg(test)]
mod tests {
    use std::path::Path;

    use marlin::verilator::{VerilatorRuntime, VerilatorRuntimeOptions};
    use marlin::verilog::prelude::*;
    use snafu::Whatever;

    #[verilog(src = "src/fpga/verilog/src/prng.sv", name = "prng64")]
    pub struct PRNG64;

    impl<'ctx> PRNG64<'ctx> {
        fn tick(&mut self) {
            self.clk = 1;
            self.eval();
            self.clk = 0;
            self.eval();
        }
    }

    fn reset(dut: &mut PRNG64) {
        dut.rst_n = 0;
        dut.seed = 123;
        dut.enable = 0;
        dut.clk = 0;

        dut.eval();

        dut.tick();

        dut.rst_n = 1;
    }

    #[test]
    #[snafu::report]
    fn test_en_low() -> Result<(), Whatever> {
        let runtime = VerilatorRuntime::new2(
            "build",
            &["src/fpga/verilog/src/prng.sv"],
            &[] as &[&Path],
            [],
            VerilatorRuntimeOptions::default(),
        )?;

        let mut dut = runtime.create_model_simple::<PRNG64>()?;

        reset(&mut dut);

        dut.enable = 0;

        dut.tick();
        dut.tick();
        dut.tick();

        assert_eq!(dut.out, 123);

        Ok(())
    }

    #[test]
    #[snafu::report]
    fn test_en_high_changes() -> Result<(), Whatever> {
        let runtime = VerilatorRuntime::new2(
            "build",
            &["src/fpga/verilog/src/prng.sv"],
            &[] as &[&Path],
            [],
            VerilatorRuntimeOptions::default(),
        )?;

        let mut dut = runtime.create_model_simple::<PRNG64>()?;

        reset(&mut dut);

        dut.enable = 1;

        let mut last_val = dut.out;

        for _ in 0..10 {
            dut.tick();
            let curr_val = dut.out;
            assert_ne!(curr_val, last_val);
            last_val = curr_val;
        }

        Ok(())
    }

    #[test]
    #[snafu::report]
    fn test_reset_seed() -> Result<(), Whatever> {
        let runtime = VerilatorRuntime::new2(
            "build",
            &["src/fpga/verilog/src/prng.sv"],
            &[] as &[&Path],
            [],
            VerilatorRuntimeOptions::default(),
        )?;

        let mut dut = runtime.create_model_simple::<PRNG64>()?;

        // Manual reset sequence to test specific seed behavior
        dut.rst_n = 1;
        dut.enable = 0;
        dut.seed = 10;
        dut.clk = 0;
        dut.eval();

        dut.tick();

        dut.rst_n = 0;
        dut.eval();

        dut.tick();

        assert_eq!(dut.out, 10);

        Ok(())
    }
}
