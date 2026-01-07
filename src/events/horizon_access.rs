use crate::elements::HorizonState;

#[derive(Debug, Clone, PartialEq)]
pub struct HorizonAccess {
    satellite_id: String,
    observatory_id: String,
    start: HorizonState,
    end: HorizonState,
}

impl HorizonAccess {
    pub fn new(satellite_id: String, observatory_id: String, start: &HorizonState, end: &HorizonState) -> Self {
        Self {
            satellite_id,
            observatory_id,
            start: *start,
            end: *end,
        }
    }

    pub fn get_satellite_id(&self) -> String {
        self.satellite_id.clone()
    }

    pub fn get_observatory_id(&self) -> String {
        self.observatory_id.clone()
    }

    pub fn get_start(&self) -> HorizonState {
        self.start
    }

    pub fn get_end(&self) -> HorizonState {
        self.end
    }

    pub fn set_start(&mut self, start: HorizonState) {
        self.start = start;
    }

    pub fn set_end(&mut self, end: HorizonState) {
        self.end = end;
    }
}
