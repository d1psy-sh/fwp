use image::DynamicImage;

/// keeps the aspect ratio and scales the pic down to the given pixels
pub fn scale(max: u32, img: DynamicImage) -> DynamicImage {
    img.thumbnail(max, max)
}
