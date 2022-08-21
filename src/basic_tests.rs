#[cfg(test)]

mod tests {
    
    use image::{GenericImageView, Pixel, RgbaImage};
    use palette::{Hsluva, IntoColor, Srgba};
    #[test]
    fn bypass_test() {
        let test_image = image::open("test-images/Lenna.png").expect("Test image can't be opened");
        let (width, height) = test_image.dimensions();
        let mut imgbuf = RgbaImage::new(width, height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let px_data = test_image.get_pixel(x, y).channels().to_owned();
            let mut temp: Hsluva = Srgba::new(
                px_data[0] as f32 / 255.0,
                px_data[1] as f32 / 255.0,
                px_data[2] as f32 / 255.0,
                px_data[3] as f32 / 255.0,
            )
            .into_linear()
            .into_color();
            temp.hue += 360.0;
            let rgba: Srgba = temp.into_color();
            *pixel = image::Rgba([
                (rgba.red * 255.0) as u8,
                (rgba.green * 255.0) as u8,
                (rgba.blue * 255.0) as u8,
                (rgba.alpha * 255.0) as u8,
            ]);
        }
        //imgbuf.save_with_format("test-images/unmodified-Lenna.png", image::ImageFormat::Png).expect("Can't save result image!");
        imageproc::assert_pixels_eq_within!(test_image, imgbuf, 1);
    }
}
