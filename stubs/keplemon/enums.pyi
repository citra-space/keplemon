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

class SAALKeyMode(Enum):
    """
    How data is referenced in SAAL memory

    Attributes:
        NoDuplicates (SAALKeyMode): Keys are constructed from unique fields such as epoch or satellite ID
        DirectMemoryAccess (SAALKeyMode): Keys are constructed from the memory address of the data
    """

    NoDuplicates = ...
    DirectMemoryAccess = ...

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
