# flake8: noqa
from __future__ import annotations

from typing import Optional

from keplemon.elements import CartesianState, GeodeticPosition, KeplerianElements, KeplerianState, TLE
from keplemon.time import Epoch

class ForceProperties:
    srp_coefficient: float
    drag_coefficient: float
    mass: float
    srp_area: float
    drag_area: float
    mean_motion_dot: float
    mean_motion_dot_dot: float
    def __init__(
        self,
        srp_coefficient: float,
        srp_area: float,
        drag_coefficient: float,
        drag_area: float,
        mass: float,
        mean_motion_dot: float,
        mean_motion_dot_dot: float,
    ) -> None: ...

class InertialPropagator:
    @staticmethod
    def from_tle(tle: TLE) -> InertialPropagator: ...

    def get_cartesian_state_at_epoch(self, epoch: Epoch) -> Optional[CartesianState]: ...
    def get_keplerian_state_at_epoch(self, epoch: Epoch) -> Optional[KeplerianState]: ...

    keplerian_state: KeplerianState
    force_properties: ForceProperties

class SGP4Output:
    cartesian_state: CartesianState
    mean_elements: KeplerianElements
    osculating_elements: KeplerianElements
    geodetic_position: GeodeticPosition

def b_star_to_drag_coefficient(b_star: float) -> float:
    """Convert B* to drag coefficient."""
    ...

def drag_coefficient_to_b_star(drag_coefficient: float) -> float:
    """Convert drag coefficient to B*."""
    ...
