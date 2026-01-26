# flake8: noqa
from typing import Optional

from keplemon.elements import TLE, OrbitPlotData

class TLECatalog:
    count: int
    name: Optional[str]
    def __init__(self) -> None: ...
    @classmethod
    def from_tle_file(cls, filename: str) -> TLECatalog: ...
    def add(self, tle: TLE) -> None: ...
    def get(self, satellite_id: str) -> TLE: ...
    def remove(self, satellite_id: str) -> None: ...
    def keys(self) -> list[str]: ...
    def get_count(self) -> int: ...
    def clear(self) -> None: ...
    def __getitem__(self, satellite_id: str) -> TLE: ...
    def get_plot_data(self) -> OrbitPlotData: ...
    def fit_best_tle(
        self,
        srp_coefficient: Optional[float] = None,
        drag_coefficient: Optional[float] = None,
    ) -> TLE:
        """
        Fit the best TLE from all TLEs in this catalog using batch least squares.

        Uses the first TLE as the a priori estimate and fits to all TLE observations.
        Automatically enables drag and SRP estimation based on orbital altitude,
        unless explicit coefficients are provided.

        Args:
            srp_coefficient: Optional SRP coefficient. If provided, SRP will not be estimated.
            drag_coefficient: Optional drag coefficient. If provided, drag will not be estimated.

        Returns:
            The best-fit TLE
        """
        ...
