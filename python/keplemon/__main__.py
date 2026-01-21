from __future__ import annotations
import click
from pathlib import Path
from importlib import resources
from keplemon.time import request_time_constants_update


@click.command()
@click.option(
    "--update-eop",
    help="Update time constants and EOP data (global or path/to/output/file)",
    type=click.Path(exists=False),
)
def cli(update_eop: Path | None) -> None:
    if update_eop is not None:
        if update_eop == "global":
            time_constants_path = resources.files("keplemon") / "time_constants.dat"
        print("Requesting time constants and EOP data from USNO...")
        request_time_constants_update(str(time_constants_path))
        print(f"Updated time constants at {update_eop}")
