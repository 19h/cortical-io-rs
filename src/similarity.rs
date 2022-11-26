use std::collections::HashSet;
use std::error::Error;
use std::ops::{Add, Div, Mul, Sub};

use serde::{Deserialize, Serialize};

use crate::Fingerprint;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FingerprintSimilarity {
    pub exp_vec_left: Vec<f64>,
    pub exp_vec_right: Vec<f64>,
}

impl FingerprintSimilarity {
    pub fn new(left: &Fingerprint, right: &Fingerprint) -> Self {
        let exp_vec_left = left.expand_t::<f64>(128 * 128);
        let exp_vec_right = right.expand_t::<f64>(128 * 128);

        Self {
            exp_vec_left,
            exp_vec_right,
        }
    }

    #[inline(always)]
    pub fn euclidean_distance(&self) -> f64 {
        self.exp_vec_left
            .iter()
            .zip(self.exp_vec_right.iter())
            .map(|(l, r)| (l - r).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    #[inline(always)]
    pub fn jaccard_index(&self) -> f64 {
        self.exp_vec_left
            .iter()
            .zip(self.exp_vec_right.iter())
            .map(|(l, r)| (l - r).abs())
            .sum::<f64>()
            .div(2.0)
    }

    // normalized euclidean distance
    // 0.5*((norm((x-mean(x))-(y-mean(y)))^2)/(normalize(x-mean(x))^2+normalize(y-mean(y))^2))
    #[inline(always)]
    pub fn normalized_euclidean_distance(&self) -> f64 {
        let left_mean =
            self.exp_vec_left
                .iter()
                .sum::<f64>()
                .div(
                    self.exp_vec_left
                        .len() as f64,
                );

        let right_mean =
            self.exp_vec_right
                .iter()
                .sum::<f64>()
                .div(
                    self.exp_vec_right
                        .len() as f64,
                );

        let (left_norm, right_norm, sum) =
            self.exp_vec_left
                .iter()
                .zip(self.exp_vec_right.iter())
                .fold(
                    (0.0, 0.0, 0.0),
                    |(left_norm, right_norm, sum), (l, r)| {
                        (
                            left_norm.add((l - left_mean).powi(2)),
                            right_norm.add((r - right_mean).powi(2)),
                            sum.add((l - left_mean - (r - right_mean)).powi(2)),
                        )
                    },
                );

        sum / (left_norm + right_norm)
    }

    #[inline(always)]
    pub fn normalized_euclidean_similarity(&self) -> f64 {
        1.0 - self.normalized_euclidean_distance()
    }

    #[inline(always)]
    pub fn cosine_similarity(&self) -> f64 {
        let mut sum = 0.0;
        let mut left_sum = 0.0;
        let mut right_sum = 0.0;

        for i in 0..self.exp_vec_left.len() {
            sum += self.exp_vec_left[i] * self.exp_vec_right[i];
            left_sum += self.exp_vec_left[i] * self.exp_vec_left[i];
            right_sum += self.exp_vec_right[i] * self.exp_vec_right[i];
        }

        (sum / (left_sum * right_sum).sqrt()).add(1.0).div(2.0)
    }

    #[inline(always)]
    pub fn overlapping_all(&self) -> u64 {
        let left_hs = &self.exp_vec_left;
        let right_hs = &self.exp_vec_right;

        left_hs
            .iter()
            .zip(right_hs.iter())
            .filter(|(l, r)| l.eq(r) && (**l).eq(&1.0))
            .count() as u64
    }

    #[inline(always)]
    pub fn weighted_scoring(
        &self,
    ) -> f64 {
        self.cosine_similarity()
            .mul(self.normalized_euclidean_similarity())
            .sqrt()
    }

    #[inline(always)]
    pub fn pearson_r_coeff(&self) -> f64 {
        let left_mean = self.exp_vec_left.iter().sum::<f64>() / self.exp_vec_left.len() as f64;
        let right_mean = self.exp_vec_right.iter().sum::<f64>() / self.exp_vec_right.len() as f64;

        let (r1, r2) =
            self.exp_vec_left
                .iter()
                .zip(self.exp_vec_right.iter())
                .fold((0.0, 0.0), |(s1, s2), (l, r)| {
                    (
                        s2.add((l - left_mean).mul((r - right_mean))),
                        s2.add((l - left_mean).powi(2).mul((r - right_mean).powi(2))),
                    )
                });

        r1.div(r2.sqrt())
    }
}