from keplemon.enums import (
    TimeSystem,
    KeplerianType,
    Classification,
    ReferenceFrame,
)


def test_time_system():
    assert TimeSystem.UTC.value == "UTC"
    assert TimeSystem.TAI.value == "TAI"
    assert TimeSystem.TT.value == "TT"
    assert TimeSystem.UT1.value == "UT1"


def test_classification():
    assert Classification.Unclassified.value == "U"
    assert Classification.Confidential.value == "C"
    assert Classification.Secret.value == "S"


def test_tle_type():
    assert KeplerianType.MeanKozaiGP.value == 0
    assert KeplerianType.MeanBrouwerGP.value == 2
    assert KeplerianType.MeanBrouwerXP.value == 4


def test_reference_frame():
    assert ReferenceFrame.TEME.value == "TEME"
    assert ReferenceFrame.J2000.value == "J2000"
    assert ReferenceFrame.EFG.value == "EFG"
    assert ReferenceFrame.ECR.value == "ECR"
