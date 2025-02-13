use image::{DynamicImage, ImageReader};

use crate::{image::BGRImage, interface::WrappedPixels, locator::locate};
struct ImageWarpper {
    width: usize,
    height: usize,
    channels: usize,
    pixels: Vec<u8>,
}
fn read_image(image: &DynamicImage) -> ImageWarpper {
    let width = image.width() as usize;
    let height = image.height() as usize;
    let mut pixels = image.to_rgb8().into_raw();
    for i in 0..width {
        for j in 0..height {
            let index = (j * width + i) * 3;
            let r = pixels[index];
            let g = pixels[index + 1];
            let b = pixels[index + 2];
            pixels[index] = b;
            pixels[index + 1] = g;
            pixels[index + 2] = r;
        }
    }
    ImageWarpper {
        width,
        height,
        channels: 3,
        pixels,
    }
}
#[test]
fn test_read_image() {
    let image = ImageReader::open("tests/images/3.png")
        .unwrap()
        .decode()
        .unwrap();
    let image = read_image(&image);
    let image = BGRImage::from_wrapped_pixels(WrappedPixels {
        width: image.width,
        height: image.height,
        channels: image.channels,
        pixels: &image.pixels,
    });
    let result = locate(&image);
    println!("{:?}", result);
}
