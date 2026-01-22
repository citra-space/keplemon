from keplemon.catalogs import TLECatalog


def test_tle_catalog():
    test_tle_file = "tests/2025-04-15-celestrak.tle"
    catalog = TLECatalog.from_tle_file(test_tle_file)
    assert catalog.count == 11305
