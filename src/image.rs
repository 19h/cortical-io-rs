use std::ops::{Add, Div, Mul};

use image;
use image::ImageBuffer;
use num::Integer;

use crate::Fingerprint;

const IMAGE_WIDTH: u32 = 128;
const IMAGE_HEIGHT: u32 = 128;

pub fn generate_image_from_fingerprint(
    fingerprint: &Fingerprint,
    scale: u32,
) -> image::DynamicImage {
    let fp_vec = fingerprint.expand((IMAGE_WIDTH * IMAGE_HEIGHT) as usize);

    generate_image_from_vec(&fp_vec, scale)
}

#[inline(always)]
pub fn visual_rescale_vec_by<T: Integer + Copy>(
    fp_vec: &[T],
    scale: u32,
    fn_scale: impl Fn(T) -> T,
) -> Vec<T> {
    let mut new_vec: Vec<T> =
        Vec::with_capacity(
            (fp_vec.len() * scale as usize) * scale as usize,
        );

    // when scaling a pixel by 2, make all 4 pixels the same value
    for (i, point) in fp_vec.iter().enumerate() {
        let x = i % IMAGE_WIDTH as usize;
        let y = i / IMAGE_WIDTH as usize;

        let x_start = x * scale as usize;
        let y_start = y * scale as usize;

        for x in x_start..x_start + scale as usize {
            for y in y_start..y_start + scale as usize {
                let index = (y * (IMAGE_WIDTH * scale) as usize) + x;
                new_vec[index] = fn_scale(*point);
            }
        }
    }

    new_vec
}

pub fn generate_image_from_vec(
    fp_vec: &[u8],
    scale: u32,
) -> image::DynamicImage {
    let _imgbuf =
        vec![0; (fp_vec.len() * scale as usize) * scale as usize * 3usize];

    let scaled_fp_vec =
        visual_rescale_vec_by(
            fp_vec,
            scale,
            |p| p,
        );

    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    let buf = ImageBuffer::from_raw(
        IMAGE_WIDTH * scale,
        IMAGE_HEIGHT * scale,
        scaled_fp_vec
            .into_par_iter()
            .map(|point| {
                if point == 0 {
                    [255, 255, 255]
                } else {
                    [0, 0, 0]
                }
            })
            .flatten()
            .collect::<Vec<u8>>(),
    ).unwrap();

    image::DynamicImage::ImageRgb8(buf)
}

pub fn generate_height_image_from_vec(
    fp_vec: &Vec<u32>,
    scale: u32,
) -> image::DynamicImage {
    let _imgbuf =
        vec![0; (fp_vec.len() * scale as usize) * scale as usize * 3usize];

    let fp_max = fp_vec.iter().max().unwrap();

    let scaled_fp_vec =
        visual_rescale_vec_by(
            fp_vec,
            scale,
            |p| p.div(fp_max).mul(255).min(255),
        );

    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    let buf = ImageBuffer::from_raw(
        IMAGE_WIDTH * scale,
        IMAGE_HEIGHT * scale,
        scaled_fp_vec
            .into_par_iter()
            .map(|point| {
                if point == 0 {
                    [255, 255, 255]
                } else {
                    [point, point, point]
                }
            })
            .flatten()
            .collect::<Vec<u8>>(),
    ).unwrap();

    image::DynamicImage::ImageRgb8(buf)
}