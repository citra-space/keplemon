use saal::environment;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeodeticModel {
    WGS72 = environment::XF_GEOMOD_WGS72 as isize,
    WGS84 = environment::XF_GEOMOD_WGS84 as isize,
    EGM96 = environment::XF_GEOMOD_EGM96 as isize,
}
