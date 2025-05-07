use std::fmt;

// help: https://www.thonky.com/qr-code-tutorial/data-analysis
// help: https://dev.to/maxart2501/let-s-develop-a-qr-code-generator-part-iv-placing-bits-3mn1
// help: https://observablehq.com/@zavierhenry/encoding-qr-codes
// help: https://en.wikipedia.org/wiki/ISO/IEC_8859-1

// alpha-numeric 0-9: 0-9 A-Z: 10-35, <space>: 36, 
// $: 37, %: 38, *: 39, +: 40, -: 41, .: 42, /: 43, :: 44

// Level of error correction

// Step  0: Initialize w/ error correction level & text
// Step  1: Determine encoding type --- SKIP FOR NOW
// Step  2: Determine number of things needed
// Step  3: Start binary string with mode indicator
// Step  4: Add character count indicator
// Step  5: Encode data (create binary string)
// Step  6: Break into 8s
// Step  7: Determine number of codewords and add padding 0s 
// Step  8: If necessary, add padding pattern
// Step  9: Determine number of groups and blocks
// Step 10: 

// TODO: make this a thing

pub fn get_ver_size(ver: usize) -> usize {
    (ver * 4) + 17
}

// get the length of the data sequence
pub fn get_ch_count(ecm: EncodingMode, ver: u8) {
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
    pub pix_vals:    Vec<Vec<u8>>,
    // Low, Medium, Quartile, High
    pub ec_level:    ErrorCorrLevel,
    // numeric, alphanumeric, binary, kanji
    pub encode_mode: EncodingMode,
    //
    pub ver_num:     u8
}

impl fmt::Display for QRCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{} {}", self.source, self.pix_vals[0][0]
        )
    }
}

impl QRCode {
    fn draw_square(&mut self, x_strt: usize, y_strt: usize, size: usize) {
        let mut curr_x: usize = x_strt;
        let mut curr_y: usize = y_strt;

        for _ in 0..size - 1 {
            self.pix_vals[curr_x][curr_y] = 1;
            curr_y += 1;
        }
        for _ in 0..size - 1 {
            self.pix_vals[curr_x][curr_y] = 1;
            curr_x += 1;
        }
        for _ in 0..size - 1 {
            self.pix_vals[curr_x][curr_y] = 1;
            curr_y -= 1;
        }
        for _ in 0..size - 1 {
            self.pix_vals[curr_x][curr_y] = 1;
            curr_x -= 1;
        }
    }

    fn encode_finders(&mut self) {
        let siz: usize = self.pix_vals.len();
        self.draw_square(0, 0, 7);
        self.draw_square(2, 2, 3);
        self.pix_vals[3][3] = 1;

        self.draw_square(siz - 7, 0, 7);
        self.draw_square(siz - 5, 2, 3);
        self.pix_vals[siz - 4][3] = 1;

        self.draw_square(0, siz - 7, 7);
        self.draw_square(2, siz - 5, 3);
        self.pix_vals[3][siz - 4] = 1;
    }

    fn encode_timing(&mut self) {
        // get the size of the "tracks"
        let siz: usize = self.pix_vals.len() - 16;
        for i in 0..siz {
            self.pix_vals[6][8 + i] = ((i as u8) + 1) & 1;
        }
        for i in 0..siz {
            self.pix_vals[8 + i][6] = ((i as u8) + 1) & 1;
        }
    }

    fn encode_version(&self) {
    }

    fn encode_dark_module(&mut self) {
        let siz: usize = self.pix_vals.len();
        self.pix_vals[8][siz - 8] = 1;
    }

    pub fn all_of_the_things(&mut self) {
        self.encode_finders();
        self.encode_timing();
        self.encode_version();
        self.encode_dark_module();
    }

    fn encode_data(&mut self) {
        // encode mode indicator
        // encode character count indicator
        // encode data
        // add terminator zeros
        // add to make it multiple of 8
        // pad for max
    }
}
