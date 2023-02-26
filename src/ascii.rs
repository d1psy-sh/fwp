use image::{GenericImageView, Pixel};

// converte the DynamicImage to ascii
pub fn to_ascii(img: image::DynamicImage) -> String {
    let mut ascii = String::new();
    let (width, height) = img.dimensions();
    let mut pixels = img.pixels();
    for _ in 0..height {
        let mut row = String::new();
        for _ in 0..width {
            let pixel = pixels.next().unwrap();
            let tmp = pixel.2.to_rgb();
            let rgb: &[u8] = tmp.channels();
            // cuz 255 + 255 + 255 = 765 and 765 / 3 not > 255 we can savely typecast here
            let brightness = ((rgb[0] as u16 + rgb[1] as u16 + rgb[2] as u16) / 3) as u8;
            let ascii_char = match brightness {
                0..=25 => '#',
                26..=50 => 'X',
                51..=75 => 'x',
                76..=100 => '-',
                101..=125 => '=',
                126..=150 => '+',
                151..=175 => ':',
                176..=200 => '.',
                201..=225 => ' ',
                226..=255 => ' ',
            };
            row.push(ascii_char);
        }
        row.push('\n');
        ascii.push_str(&row);
    }
    ascii
}
