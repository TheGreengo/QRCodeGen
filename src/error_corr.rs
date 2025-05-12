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

    pub fn from_str(s: &str) -> Self {
        println!("{}", s);
        match s {
            "Low"      => ErrorCorrLevel::Low,
            "Medium"   => ErrorCorrLevel::Medium,
            "Quartile" => ErrorCorrLevel::Quartile,
            "High"     => ErrorCorrLevel::High,
            other      => ErrorCorrLevel::Medium,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            ErrorCorrLevel::Low      => "Low",
            ErrorCorrLevel::Medium   => "Medium",
            ErrorCorrLevel::Quartile => "Quartile",
            ErrorCorrLevel::High     => "High",
        }
    }
}
