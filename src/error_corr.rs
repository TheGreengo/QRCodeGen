// 7, 15, 25, 30
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ErrorCorrLevel {
    Low,
    Medium,
    Quartile,
    High,
}

impl ErrorCorrLevel {
    pub fn get_value(&self) -> u8 {
        match self {
            ErrorCorrLevel::Low      => 1,
            ErrorCorrLevel::Medium   => 2,
            ErrorCorrLevel::Quartile => 4,
            ErrorCorrLevel::High     => 8,
        }
    }
}
