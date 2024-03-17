use std::fs::File;
use crate::color::{Rgb8, Rgba8};
use crate::{Image, ImageDimensions};



#[test]
fn polish_flag() {
    let mut target_image = Image::new_uniform(Rgb8::new(255, 255, 255), ImageDimensions::new(150, 80));
    let red_bottom = Image::new_uniform(Rgb8::new(255, 64, 64), ImageDimensions::new(150, 40));
    target_image.overdraw_image(&red_bottom, ImageDimensions::new(0, 40)).expect("Failed to spill blood on flag bottom.");

    target_image.save_png(File::create("test output/Polish flag.png").expect("Failed to create Polish file.")).expect("Failed to save Polish flag.");
}



#[test]
fn puchaczewski() {
    let mut target_image = Image::new_uniform(Rgba8::new(127, 127, 127, 127), ImageDimensions::new(2048 + 256 + 64, 1024 + 256));

    let puchaczewski: Image<Rgb8> = Image::load_png(File::open("test output/Puchaczewski kompresja 5.png").expect("No Puchaczewski.")).expect("Wrong Puchaczewski.");
    target_image.overdraw_image(&puchaczewski, ImageDimensions::new(128, 128)).expect("Failed to draw Puchaczewski.");

    let mut pixel_puchaczewski = Image::new_uniform(Rgb8::BLACK, ImageDimensions::new(192, 192));
    pixel_puchaczewski.overdraw_image_rescaled(&puchaczewski, ImageDimensions::ZERO, pixel_puchaczewski.dimensions).expect("Failed to pixel Puchaczewski.");
    target_image.overdraw_image_rescaled(&pixel_puchaczewski, ImageDimensions::new(1024 + 128 + 64, 128), ImageDimensions::new(2048 + 128 + 64, 1024 + 128)).expect("Failed to draw pixel Puchaczewski.");

    target_image.save_png(File::create("test output/Puchaczewski output.png").expect("Failed to create file for new Puchaczewski.")).expect("Failed to save new Puchaczewski.");
}
