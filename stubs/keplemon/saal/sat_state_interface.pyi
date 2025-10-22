def get_relative_state(state_1: list[float], state_2: list[float], utc_ds50: float) -> list[float]:
    """Calculate the relative state between two satellites

    Args:
        state_1: primary satellite TEME state vector [x, y, z, vx, vy, vz] in km and km/s
        state_2: secondary satellite TEME state vector [x, y, z, vx, vy, vz] in km and km/s
        utc_ds50: UTC time in days since 1950
    """
    ...
