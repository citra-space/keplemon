# flake8: noqa
from enum import Enum

class AssociationConfidence(Enum):
    """
    Confidence level of an observation association

    Attributes:
        Low (AssociationConfidence): Low confidence
        Medium (AssociationConfidence): Medium confidence
        High (AssociationConfidence): High confidence
    """

    Low = ...
    Medium = ...
    High = ...

class CovarianceType(Enum):
    """
    Reference frame/element types for a covariance matrix

    Attributes:
        Inertial (CovarianceType): Cartesian TEME
        Relative (CovarianceType): Cartesian UVW
        Equinoctial (CovarianceType): Equinoctial
    """

    Inertial = ...
    Relative = ...
    Equinoctial = ...

class GeodeticModel(Enum):
    """
    Geodetic model used for Earth shape and gravity

    Attributes:
        WGS72 (GeodeticModel): WGS72 geodetic model
        WGS84 (GeodeticModel): WGS84 geodetic model
        EGM96 (GeodeticModel): EGM96 geodetic model
    """

    WGS72 = ...
    WGS84 = ...
    EGM96 = ...

class MeanEquinox(Enum):
    """
    Mean equinox used for topocentric calculations

    Attributes:
        OfDate (MeanEquinox): Mean equinox of date
        OfYear (MeanEquinox): Mean equinox of year
        J2000 (MeanEquinox): J2000 mean equinox
        B1950 (MeanEquinox): B1950 mean equinox
    """

    OfDate = ...
    OfYear = ...
    J2000 = ...
    B1950 = ...
    value: int

    def get_value(self) -> int: ...

class TimeSystem(Enum):
    """
    Attributes:
        UTC (TimeSystem): Coordinated Universal Time
        TAI (TimeSystem): International Atomic Time
        TT (TimeSystem): Terrestrial Time
        UT1 (TimeSystem): Universal Time
    """

    UTC = ...
    TAI = ...
    TT = ...
    UT1 = ...

class Classification(Enum):
    """
    Simple classification primarily used to construct single-character identifiers in SAAL data

    Attributes:
        Unclassified (Classification): Unclassified
        Confidential (Classification): Confidential
        Secret (Classification): Secret
    """

    Unclassified = ...
    Confidential = ...
    Secret = ...

class KeplerianType(Enum):
    """
    Theory used to construct the Keplerian elements

    Attributes:
        MeanKozaiGP (KeplerianType): SGP4 mean elements with Kozai mean motion
        MeanBrouwerGP (KeplerianType): SGP4 mean elements with Brouwer mean motion
        MeanBrouwerXP (KeplerianType): SGP4-XP mean elements with Brouwer mean motion
        Osculating (KeplerianType): Osculating elements with Brouwer mean motion
    """

    MeanKozaiGP = ...
    MeanBrouwerGP = ...
    MeanBrouwerXP = ...
    Osculating = ...

class ReferenceFrame(Enum):
    """
    Reference frame used for inertial elements

    Attributes:
        TEME (ReferenceFrame): True Equator Mean Equinox
        J2000 (ReferenceFrame): J2000
        EFG (ReferenceFrame): Earth Fixed Greenwich (no polar motion)
        ECR (ReferenceFrame): Earth Centered Rotating (polar motion)
    """

    TEME = ...
    J2000 = ...
    EFG = ...
    ECR = ...

class UCTObservability(Enum):
    """
    Observability status of a UCT during analysis.

    Attributes:
        Confirmed (UCTObservability): UCT was confirmed to be observable (matched orphan observations)
        Possible (UCTObservability): UCT was within sensor field of view but not confirmed
        Unavailable (UCTObservability): UCT was not observable during the analysis window
    """

    Confirmed = ...
    Possible = ...
    Unavailable = ...
    value: str

class UCTValidity(Enum):
    """
    Validity assessment of a UCT based on analysis.

    Attributes:
        LikelyReal (UCTValidity): UCT is likely a real distinct object (has confirmed observations)
        PossibleCrossTag (UCTValidity): UCT may be a cross-tag of an approved satellite (proximity events found)
        PossibleManeuver (UCTValidity): UCT may be result of a maneuver (close approaches found)
        Inconclusive (UCTValidity): Insufficient evidence to determine UCT validity
    """

    LikelyReal = ...
    PossibleCrossTag = ...
    PossibleManeuver = ...
    Inconclusive = ...
    value: str
