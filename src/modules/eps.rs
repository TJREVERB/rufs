pub struct EPS {
    pub address: u16,
    pub map: [RelayName; 8],
}

pub enum RelayName {
    A = 1,
    I2C = 2,
    C = 3,
    Antenna = 4,
    Pi = 5,
    Iridium = 6,
    APRS = 7,
    H = 8,
}

impl EPS {
    pub fn new() -> Self {
        EPS {
            address: 0x57,
            map: [
                RelayName::A,
                RelayName::I2C,
                RelayName::C,
                RelayName::Antenna,
                RelayName::Pi,
                RelayName::Iridium,
                RelayName::APRS,
                RelayName::H,
            ],
        }
    }
}
