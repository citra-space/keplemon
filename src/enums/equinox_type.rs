use saal::astro;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeanEquinox {
    OfDate = astro::YROFEQNX_OBTIME,
    OfYear = astro::YROFEQNX_CURR,
    J2000 = astro::YROFEQNX_2000,
    B1950 = astro::YROFEQNX_1950,
}

impl MeanEquinox {
    pub fn get_value(&self) -> i32 {
        match self {
            MeanEquinox::OfYear => astro::YROFEQNX_CURR as i32,
            MeanEquinox::J2000 => astro::YROFEQNX_2000 as i32,
            MeanEquinox::B1950 => astro::YROFEQNX_1950 as i32,
            MeanEquinox::OfDate => astro::YROFEQNX_OBTIME as i32,
        }
    }
}
