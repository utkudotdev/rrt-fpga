import os
from pathlib import Path

from cocotb_tools.runner import get_runner


def gen_test_runner(src: str, hdl_toplevel: str, module_name: str):
    def test_runner():
        sim = os.getenv("SIM", "verilator")

        proj_path = Path(__file__).resolve().parent.parent

        sources = [proj_path / "src" / src]

        runner = get_runner(sim)
        runner.build(
            sources=sources,
            hdl_toplevel=hdl_toplevel,
            timescale=("1ns", "1ns"),
            always=True,
        )
        runner.test(hdl_toplevel=hdl_toplevel, test_module=module_name)

    return test_runner
