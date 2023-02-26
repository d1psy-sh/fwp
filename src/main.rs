use fwp::{ascii, open, scale};
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image = open::open_img("./assets/colors.jpg")?;
    let thumb1 = scale::scale(100, image);
    // save ascii to file
    let f = &mut std::fs::File::create("./assets/colors.txt")?;
    f.write_all(ascii::to_ascii(thumb1).as_bytes())?;
    Ok(())
}
