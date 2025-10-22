from keplemon.saal import astro_func_interface


class TestAstroFuncInterface:
    def test_get_jpl_sun_and_moon_position(self):
        epoch = 24000.0  # Example Julian Date
        sun_pos, moon_pos = astro_func_interface.get_jpl_sun_and_moon_position(epoch)

        # Check that the returned positions are tuples of length 3
        assert sun_pos[0] == -149257535.84066284
        assert moon_pos[0] == -375197.53303902777
