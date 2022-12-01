use std::collections::BTreeSet;

use num::Float;
use num_traits::{FromPrimitive, Zero};
use crate::find_peaks::PeakFinder;

pub fn gaussian(x1: f32, y1: f32, x2: f32, y2: f32, radius: f32) -> f32 {
    let pi = std::f32::consts::PI;
    let numerator = 1.0 / (2.0 * pi);
    let denominator = 0.4;

    let exponent =
        -1.0
            * (
                3.0
                * (
                    (x1 - x2).powi(2)
                    + (y1 - y2).powi(2)
                ).sqrt()
                / radius
            ).powi(2)
            / denominator;

    let exponent = exponent.exp();

    numerator * exponent
}

pub fn kde(x: f32, y: f32, points: &Vec<(f32, f32)>, radius: f32) -> f32 {
    let mut sum = 0.0;

    for point in points.iter() {
        sum += gaussian(x, y, point.0, point.1, radius);
    }

    sum
}

fn amax(
    vec: &[f32; 16384],
) -> (usize, f32) {
    let mut max = f32::zero();
    let mut pos = 0;

    for (i, val) in vec.iter().enumerate() {
        if *val > max {
            max = *val;
            pos = i;
        }
    }

    (pos, max)
}

const LOCALITY: f32 = 5.0f32;

pub struct Kde {
    pub data: [u32; 16384],
    pub points: Vec<(f32, f32)>,
    pub kde: [f32; 16384],

    pub densest_points: BTreeSet<usize>,

    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,

    pub radius: f32,
}

impl Kde {
    pub fn new(vec: &[u32; 16384]) -> Kde {
        let mut data = [0u32; 16384];

        data.copy_from_slice(vec.as_slice());


        Kde {
            data,
            points: Vec::new(),
            kde: [0.0; 16384],

            densest_points: BTreeSet::new(),

            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,

            radius: 10.0,
        }
    }

    pub fn clear(&mut self) {
        self.points = Vec::new();
        self.kde = [0.0f32; 16384];

        self.densest_points = BTreeSet::new();

        self.x_min = 0.0f32;
        self.x_max = 0.0f32;
        self.y_min = 0.0f32;
        self.y_max = 0.0f32;

        self.radius = 10.0f32;
    }

    pub fn build_points(&mut self) {
        for y in 0..16384 {
            if self.data[y] <= 0 {
                continue;
            }

            self.points
                .push(
                    (
                        (y / 128) as f32,
                        (y % 128) as f32
                    )
                );
        }
    }

    pub fn determine_kde_params(&mut self) -> Option<()> {
        // find the min and max of x and y respectively
        self.x_min = self.points.iter().map(|p| p.0).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        self.x_max = self.points.iter().map(|p| p.0).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        self.y_min = self.points.iter().map(|p| p.1).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        self.y_max = self.points.iter().map(|p| p.1).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        let dx = self.x_max - self.x_min;
        let dy = self.y_max - self.y_min;

        self.radius = dx.min(dy) / LOCALITY;

        Some(())
    }

    pub fn calculate_kde(&mut self) {
        for y in 0..128 {
            for x in 0..128 {
                self.kde[(x * 128) + y] =
                    kde(
                        x as f32,
                        y as f32,
                        &self.points,
                        self.radius,
                    );
            }
        }
    }

    pub fn determine_densest_points(&mut self) {
        let mut pf =
            PeakFinder::new(&self.kde);

        pf.with_min_prominence(2.0);

        let peaks =
            pf.find_peaks();

        let peaks =
            peaks.iter()
                .map(|p| p.clone().position.collect::<Vec<_>>())
                .flatten()
                .collect::<Vec<_>>();

        // find the densest area in kde_vec
        for peak in peaks {
            let (x, y) = (peak / 128, peak % 128);

            let cutoff = 10.0;

            // find all points within 5 pixels of the densest point in "points"
            for point in self.points.iter() {
                if (point.0 - x as f32).abs() < cutoff
                    && (point.1 - y as f32).abs() < cutoff {
                    let idx = point.0 as u32 * 128 + point.1 as u32;

                    if self.data[idx as usize] > 0 {
                        self.densest_points.insert(idx as usize);
                    }
                }
            }
        }

        dbg!(&self.densest_points);
    }

    pub fn fit_kde(&mut self) {
        let (min, max) =
            self.kde
                .iter()
                .fold(
                    (std::f32::MAX, std::f32::MIN),
                    |(min, max), &x|
                        (
                            min.min(x),
                            max.max(x)
                        ),
                );

        // fit all f32 values in kde_dev into 0..255
        self.kde
            .iter_mut()
            .for_each(|x|
                *x = ((*x - min) / (max - min)) * 255.0
            );
    }

    pub fn get_kde_data(&self) -> [u32; 16384] {
        let mut kde_data = [0u32; 16384];

        for (i, val) in self.kde.iter().enumerate() {
            kde_data[i] = *val as u32;
        }

        kde_data
    }

    pub fn run(&mut self) {
        self.build_points();
        self.determine_kde_params();
        self.calculate_kde();
        self.determine_densest_points();
        self.fit_kde();
    }
}

pub struct Density {
    pub data: [u32; 16384],
}

impl Density {
    pub fn new(vec: &[u32]) -> Self {
        assert_eq!(vec.len(), 16384);

        let mut data = [0u32; 16384];

        data.copy_from_slice(vec);

        Self {
            data,
        }
    }

    pub fn set_data(&mut self, vec: &[u32; 16384]) {
        let mut data = [0u32; 16384];

        data.copy_from_slice(vec.as_slice());

        self.data = data;
    }

    pub fn get_data(&self) -> &[u32; 16384] {
        &self.data
    }

    pub fn filter_points_min(&mut self, min: u32) {
        self.data
            .iter_mut()
            .filter(|b| **b < min)
            .for_each(|b| *b = u32::zero());
    }

    pub fn kde(&mut self) -> Kde {
        let mut kde =
            Kde::new(&self.data);

        kde.run();

        kde
    }
}