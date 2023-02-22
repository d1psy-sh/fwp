use image::io::Reader as ImageReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load a picture
    // open a jpg
    let image = ImageReader::open("./assets/colors.jpg")?.decode()?;
    let image2 = ImageReader::open("./assets/epicPic.jpg")?.decode()?;
    // image aspect ratio is preserved!
    let thumb = image.thumbnail(200, 200);
    let thumb2 = image2.thumbnail(200, 200);
    thumb.save("./assets/thumbColors.jpg")?;
    thumb2.save("./assets/thumbEpicPic.jpg")?;
    Ok(())
}
