use std::fmt;

// help: https://dev.to/maxart2501/let-s-develop-a-qr-code-generator-part-iv-placing-bits-3mn1
// help: https://observablehq.com/@zavierhenry/encoding-qr-codes
// help: https://en.wikipedia.org/wiki/ISO/IEC_8859-1

// alpha-numeric 0-9: 0-9 A-Z: 10-35, <space>: 36, 
// $: 37, %: 38, *: 39, +: 40, -: 41, .: 42, /: 43, :: 44

// Level of error correction

pub fn get_ver_size(ver: u8) -> u8 {
    (ver * 4) + 17
}

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

pub enum EncodingMode {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
}

impl EncodingMode {
    pub fn get_nm(&self) -> u8 {
        match self {
            EncodingMode::Numeric      => 1,
            EncodingMode::Alphanumeric => 2,
            EncodingMode::Byte         => 4,
            EncodingMode::Kanji        => 8,
        }
    }
}

// get the length of the data sequence
pub fn getLength(ecm: EncodingMode, ver: u8) {
    if ver < 10 {
        match ecm {
            EncodingMode::Numeric      => 10,
            EncodingMode::Alphanumeric =>  9,
            EncodingMode::Byte         =>  8,
            EncodingMode::Kanji        =>  8,
        };
    } else if ver < 27 {
        match ecm {
            EncodingMode::Numeric      => 12,
            EncodingMode::Alphanumeric => 11,
            EncodingMode::Byte         => 16,
            EncodingMode::Kanji        => 10,
        };
    } else {
        match ecm {
            EncodingMode::Numeric      => 14,
            EncodingMode::Alphanumeric => 13,
            EncodingMode::Byte         => 16,
            EncodingMode::Kanji        => 12,
        };
    }
}

pub struct QRCode {
    // text to be encoded
    pub source:      String,
    // values of the actual pixels to be set
    // TODO: make variable size
    pub pix_vals:    Vec<Vec<u8>>,
    // Low, Medium, Quartile, High
    pub ec_level:    ErrorCorrLevel,
    // numeric, alphanumeric, binary, kanji
    pub encode_mode: EncodingMode,
}

impl fmt::Display for QRCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{} {}", self.source, self.pix_vals[0][0]
        )
    }
}

impl QRCode {
    fn encode_finders(&self) {
    }
    fn encode_timing(&self) {
    }
    fn encode_version(&self) {
    }
    fn encode_dark_module(&self) {
    }
    pub fn all_of_the_things(&self) {
        self.encode_finders();
        self.encode_timing();
        self.encode_version();
        self.encode_dark_module();
    }
}
