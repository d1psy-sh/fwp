use image::io::Reader as ImageReader;

pub fn open_img(path: &str) -> Result<image::DynamicImage, image::ImageError> {
    let img = ImageReader::open(path)?.decode()?;
    Ok(img)
}

