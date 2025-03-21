use std::io;

mod qr_base;
use qr_base::QRCode;
use qr_base::ErrorCorrLevel;
use qr_base::EncodingMode;

fn main() {
    let mut qr = QRCode{
        source:        String::from("Test"),
        pix_vals:      [[0; 33];33],
        count:         12,
        ec_level:      ErrorCorrLevel::Medium,
        encode_mode: EncodingMode::Alphanumeric,
    };
    qr.all_of_the_things();
    qr.encode_mode = EncodingMode::Numeric;
    qr.ec_level    = ErrorCorrLevel::High;

    println!("{}", qr);

    let mut plain_text = String::new();
    let mut out_path   = String::new();

    println!("Welcome to the Greengo's QR Code Generator!");

    println!("Please enter the string that you would like to encode:");

    io::stdin()
        .read_line(&mut plain_text)
        .expect("Error with input!");

    println!("Now please enter the name you'd like for the generated image:");

    io::stdin()
        .read_line(&mut out_path)
        .expect("Error with input!");

    let plan = &plain_text[..plain_text.len()-1];
    let plin = &out_path[..out_path.len()-1];
    println!("Your chosen plain text was: {}", plan);
    println!("Your save path was: {}", plin);
}
