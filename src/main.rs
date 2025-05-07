use std::io;

mod qr_base;
use qr_base::QRCode;
use qr_base::ErrorCorrLevel;
use qr_base::EncodingMode;
use qr_base::get_ver_size;
mod image_help;
use image_help::make_img;
// use encoding_rs::ISO_8859_1;

fn main() {
    // first we're going to assume that we have AlphaNumeric Encoding
    // we can work on that later
    let size: usize = get_ver_size(7);
    let mut qr = QRCode{
        source:      String::from("This is my QR code testing test"),
        pix_vals:    vec![vec![0; size]; size],
        ec_level:    ErrorCorrLevel::Medium,
        encode_mode: EncodingMode::Alphanumeric,
        ver_num:     7,
    };

    qr.all_of_the_things();

    println!("{}", qr);

    // let mut plain_text = String::new();
    // let mut out_path   = String::new();

    // println!("Welcome to the Greengo's QR Code Generator!");

    // println!("Please enter the string that you would like to encode:");

    // io::stdin()
    //     .read_line(&mut plain_text)
    //     .expect("Error with input!");

    // println!("Now please enter the name you'd like for the generated image:");

    // io::stdin()
    //     .read_line(&mut out_path)
    //     .expect("Error with input!");

    // let plan = &plain_text[..plain_text.len()-1];
    // let plin = &out_path[..out_path.len()-1];
    // println!("Your chosen plain text was: {}", plan);
    // println!("Your save path was: {}", plin);
    
    make_img(qr.pix_vals);
}
