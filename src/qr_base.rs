use std::fmt;
use std::io;

use crate::encd::EncodingMode;
use crate::error_corr::ErrorCorrLevel;

// help: https://www.thonky.com/qr-code-tutorial/data-analysis
// help: https://dev.to/maxart2501/let-s-develop-a-qr-code-generator-part-iv-placing-bits-3mn1

pub fn get_ver_size(ver: usize) -> usize {
    (ver * 4) + 17
}

// get the length of the data sequence
pub fn get_ch_count(ecm: &EncodingMode, ver: &u8) -> usize {
    if ver < &10 {
        match ecm {
            EncodingMode::Numeric      => 10,
            EncodingMode::Alphanumeric =>  9,
            EncodingMode::Byte         =>  8,
            EncodingMode::Kanji        =>  8,
            EncodingMode::ECI          =>  8,
        }
    } else if ver < &27 {
        match ecm {
            EncodingMode::Numeric      => 12,
            EncodingMode::Alphanumeric => 11,
            EncodingMode::Byte         => 16,
            EncodingMode::Kanji        => 10,
            EncodingMode::ECI          => 16,
        }
    } else {
        match ecm {
            EncodingMode::Numeric      => 14,
            EncodingMode::Alphanumeric => 13,
            EncodingMode::Byte         => 16,
            EncodingMode::Kanji        => 12,
            EncodingMode::ECI          => 16,
        }
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
    pub ver_num:     u8,
    // 
    pub bit_str:     Vec<u8>
}

impl fmt::Display for QRCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{} {} {}", self.source, 
            self.pix_vals[0][0], self.ec_level.to_str()
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
        self.encode_data();
        self.print();
    }

    fn print(&self) {
        for i in 0..self.bit_str.len() {
            print!("{:1}", self.bit_str[i]);
        }
        print!("\n");
    }

    fn enc_enc_mode(&mut self) {
        let code = self.encode_mode.get_mode_indicator();
        for i in 0..3 {
            self.bit_str[i] = (code & (0b1000 >> i)) >> (3 - i);
        }
    }

    fn enc_char_count_ind(&mut self) {
        let ch_count = get_ch_count(&self.encode_mode, &self.ver_num);
        let place = 1 << (ch_count - 1);
        let mut curr: usize = 0;

        for i in 0..ch_count {
            curr = place >> i;
            if ((self.source.len() & curr) != 0) {
                self.bit_str[i + 4] = 1;
            }
        }
        println!("Character Count: {:b}", self.source.len());
    }

    fn encode_data(&mut self) {
        // encode mode indicator
        self.enc_enc_mode();
        // encode character count indicator
        self.enc_char_count_ind();
        // encode data
        // add terminator zeros
        // add to make it multiple of 8
        // pad for max
    }
}
