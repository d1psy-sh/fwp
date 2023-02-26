use fwp::open;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load a picture
    // open a jpg
    let image = open::open_img("./assets/colors.jpg")?;
    let image2 = open::open_img("./assets/epicPic.jpg")?;
    // image aspect ratio is preserved!
    let thumb = image.thumbnail(200, 200);
    let thumb2 = image2.thumbnail(200, 200);
    thumb.save("./assets/thumbColors.jpg")?;
    thumb2.save("./assets/thumbEpicPic.jpg")?;
    Ok(())
}
