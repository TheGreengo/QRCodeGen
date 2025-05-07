use image::{RgbImage, Rgb};

pub fn make_img(pix: Vec<Vec<u8>>) {
    // For now, assume version 4 and 9 pixels per "square"
    // so we have 33 * 9 = 297
    let scale: usize = 16;
    let width        = scale * pix.len();
    let height       = scale * pix.len();

    let mut img = RgbImage::new(width as u32, height as u32);
    let mut ind_x: usize;
    let mut ind_y: usize;

    for x in 0..width {
        for y in 0..height {
            ind_x = x / scale;
            ind_y = y / scale;
            if pix[ind_x][ind_y] == 1 {
                let pixel = Rgb([0, 0, 0]);
                img.put_pixel(x as u32, y as u32, pixel);
            } else {
                let pixel = Rgb([255, 255, 255]);
                img.put_pixel(x as u32, y as u32, pixel);
            }
        }
    }

    img.save("output.png")
        .expect("Failed to save image");
}
