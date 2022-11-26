use std::ops::{Add, Mul};

use image;

use crate::Fingerprint;

const IMAGE_WIDTH: u32 = 128;
const IMAGE_HEIGHT: u32 = 128;

pub fn generate_image_from_fingerprint(
    fingerprint: &Fingerprint,
    scale: u32,
) -> image::DynamicImage {
    let fp_vec = fingerprint.expand((IMAGE_WIDTH * IMAGE_HEIGHT) as usize);

    let mut imgbuf =
        image::ImageBuffer::new(
            IMAGE_WIDTH * scale,
            IMAGE_HEIGHT * scale,
        );

    let scaled_fp_vec = {
        let mut new_vec = vec![0; (fp_vec.len() * scale as usize) * scale as usize];

        // when scaling a pixel by 2, make all 4 pixels the same value
        for (i, point) in fp_vec.iter().enumerate() {
            let x = i % IMAGE_WIDTH as usize;
            let y = i / IMAGE_WIDTH as usize;

            let x_start = x * scale as usize;
            let y_start = y * scale as usize;

            for x in x_start..x_start + scale as usize {
                for y in y_start..y_start + scale as usize {
                    let index = (y * (IMAGE_WIDTH * scale) as usize) + x;
                    new_vec[index] = *point;
                }
            }
        }

        new_vec
    };

    dbg!(scaled_fp_vec.len());
    dbg!(imgbuf.len() / 3);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index =
            y
                .mul(IMAGE_WIDTH.mul(scale))
                .add(x);

        let val = scaled_fp_vec[index as usize];

        // if val is 1, render pixel as pink
        // if val is 0, render pixel as white
        *pixel = image::Rgb([
            if val == 1 { 251 } else { 255 },
            if val == 1 { 72 } else { 255 },
            if val == 1 { 196 } else { 255 },
        ]);
    }

    image::DynamicImage::ImageRgb8(imgbuf)
}