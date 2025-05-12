use std::io;

mod vers_size_table;
use vers_size_table::get_version;
mod encd;
use encd::EncodingMode;
mod error_corr;
use error_corr::ErrorCorrLevel;
mod qr_base;
use qr_base::QRCode;
use qr_base::get_ver_size;
mod image_help;
use image_help::make_img;
// use encoding_rs::ISO_8859_1;

// Nexts:
// - get version smartly

fn main() {
    // first we're going to assume that we have AlphaNumeric Encoding
    // we can work on that later
    let mut plain_text = String::new();

    println!("Welcome to the Greengo's QR Code Generator!");

    println!("Please enter the string that you would like to encode:");

    io::stdin()
        .read_line(&mut plain_text)
        .expect("Error with input!");

    println!("Please enter the level of error correction:");

    let mut rrrrrr = String::new();
    io::stdin()
        .read_line(&mut rrrrrr)
        .expect("Error with input!");
    let rrr = &rrrrrr[..rrrrrr.len() - 1];

    let (vers, maxx) = get_version(
       plain_text.len() - 1, 
       EncodingMode::Alphanumeric, 
       ErrorCorrLevel::from_str(rrr)
   );
    println!("version: {}", vers);
    
    let size: usize = get_ver_size(vers);

    let mut qr = QRCode{
        source:      String::from(plain_text),
        pix_vals:    vec![vec![0; size]; size],
        ec_level:    ErrorCorrLevel::from_str(rrr),
        encode_mode: EncodingMode::Alphanumeric,
        ver_num:     7,
        bit_str:     vec![0; maxx]
    };

    qr.all_of_the_things();

    // let mut out_path   = String::new();
    let mut curr_ind: usize = 0;

    println!("{}", qr);

    // let plan = &plain_text[..plain_text.len()-1];
    // let plin = &out_path[..out_path.len()-1];
    // println!("Your chosen plain text was: {}", plan);
    // println!("Your save path was: {}", plin);
    
    make_img(qr.pix_vals);
}
