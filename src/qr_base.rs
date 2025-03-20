// help: https://dev.to/maxart2501/let-s-develop-a-qr-code-generator-part-iv-placing-bits-3mn1
// help: https://observablehq.com/@zavierhenry/encoding-qr-codes
// help: https://en.wikipedia.org/wiki/ISO/IEC_8859-1

// alpha-numeric 0-9: 0-9 A-Z: 10-35, <space>: 36, 
// $: 37, %: 38, *: 39, +: 40, -: 41, .: 42, /: 43, :: 44

use std::fmt;

pub struct QRCode {
    pub source: String,
    pub pix_vals: [[u8; 33]; 33],
    pub count: u16,
    pub error_correction: u8,
    pub encoding_mode: u8,
}

impl fmt::Display for QRCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{} {} {} {}", self.source, self.count, 
            self.error_correction, self.pix_vals[0][0]
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
