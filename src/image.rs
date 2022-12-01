use std::ops::{Div, Mul};

use image;
use image::ImageBuffer;
use num::Integer;
use num_traits::cast::FromPrimitive;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::Fingerprint;

const IMAGE_WIDTH: u32 = 128;
const IMAGE_HEIGHT: u32 = 128;

pub fn generate_image_from_fingerprint(
    fingerprint: &Fingerprint,
    scale: u32,
) -> Option<image::DynamicImage> {
    let fp_vec = fingerprint.expand((IMAGE_WIDTH * IMAGE_HEIGHT) as usize);

    generate_image_from_vec(&fp_vec, scale)
}

#[inline(always)]
pub fn visual_rescale_vec_by<T: Integer + Copy, P: Integer + Copy>(
    fp_vec: &[T],
    scale: u32,
    fn_scale: impl Fn(T) -> P,
) -> (Vec<P>, Vec<usize>) {
    let mut new_vec: Vec<P> =
        vec![P::zero(); (fp_vec.len() * scale as usize) * scale as usize];

    let mut ref_vec: Vec<usize> =
        vec![0usize; (fp_vec.len() * scale as usize) * scale as usize];

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
                ref_vec[index] = i;
            }
        }
    }

    (new_vec, ref_vec)
}

pub fn generate_image_from_vec(
    fp_vec: &[u8],
    scale: u32,
) -> Option<image::DynamicImage> {
    let (scaled_fp_vec, _) =
        visual_rescale_vec_by::<u8, u8>(
            fp_vec,
            scale,
            |p| p,
        );

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
    )?;

    Some(image::DynamicImage::ImageRgb8(buf))
}

pub fn generate_height_image_from_vec(
    fp_vec: &[u32],
    scale: u32,
    fn_color: impl Fn(u8, usize) -> [u8; 3] + Sync,
) -> Option<image::DynamicImage> {
    let fp_max = fp_vec.iter().max()?;

    let (scaled_fp_vec, ref_vec) =
        visual_rescale_vec_by::<u32, u8>(
            fp_vec,
            scale,
            |p|
                p
                    .mul(255)
                    .div(*fp_max.max(&1))
                    .min(255) as u8,
        );

    let buf = ImageBuffer::from_raw(
        IMAGE_WIDTH * scale,
        IMAGE_HEIGHT * scale,
        scaled_fp_vec
            .into_par_iter()
            .enumerate()
            .map(|(i, point)| {
                fn_color(point, ref_vec[i])
            })
            .flatten()
            .collect::<Vec<u8>>(),
    )?;

    Some(image::DynamicImage::ImageRgb8(buf))
}