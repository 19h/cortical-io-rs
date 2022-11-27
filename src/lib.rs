use std::error::Error;

use serde::{Deserialize, Serialize};

#[cfg(feature = "client")]
pub use client::Cortical;

use crate::similarity::FingerprintSimilarity;

pub mod similarity;

#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "client")]
pub mod client;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Retina {
    #[serde(rename = "retinaName")]
    pub retina_name: String,
    #[serde(rename = "numberOfColumns")]
    pub number_of_columns: u32,
    #[serde(rename = "numberOfTermsInRetina")]
    pub number_of_terms_in_retina: u64,
    pub description: String,
    #[serde(rename = "numberOfRows")]
    pub number_of_rows: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fingerprint {
    pub positions: Vec<u32>,
}

impl Fingerprint {
    pub fn expand(&self, len: usize) -> Vec<u8> {
        let mut expanded = vec![0; len];

        for pos in self.positions.iter() {
            expanded[*pos as usize] = 1;
        }

        expanded
    }

    pub fn expand_t<T: num::Float>(&self, len: usize) -> Vec<T> {
        let zero = T::from(0).unwrap();
        let one = T::from(1).unwrap();

        let mut expanded: Vec<T> = vec![zero; len];

        for pos in self.positions.iter() {
            expanded[*pos as usize] = one;
        }

        expanded
    }

    pub fn compare(&self, other: &Fingerprint) -> FingerprintSimilarity {
        FingerprintSimilarity::new(self, other)
    }
}

impl From<Fingerprint> for Vec<f64> {
    fn from(fingerprint: Fingerprint) -> Self {
        fingerprint
            .expand_t::<f64>(128 * 128)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextSlice {
    pub text: String,
    pub fingerprint: Option<Fingerprint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextSliceRequest {
    pub retina_name: String,
    pub start_index: usize,
    pub max_results: usize,
    pub get_fingerprint: bool,
}

impl Default for TextSliceRequest {
    fn default() -> Self {
        Self {
            retina_name: "en_general".to_string(),
            start_index: 0,
            max_results: 10,
            get_fingerprint: false,
        }
    }
}

impl TextSliceRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_retina_name(mut self, retina_name: &str) -> Self {
        self.retina_name = retina_name.to_string();
        self
    }

    pub fn with_start_index(mut self, start_index: usize) -> Self {
        self.start_index = start_index;
        self
    }

    pub fn with_max_results(mut self, max_results: usize) -> Self {
        self.max_results = max_results;
        self
    }

    pub fn with_get_fingerprint(mut self, get_fingerprint: bool) -> Self {
        self.get_fingerprint = get_fingerprint;
        self
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextEnvelope {
    pub text: String,
}

impl TextEnvelope {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCategoryFilterRequest {
    #[serde(rename = "categoryName")]
    pub category_name: Option<String>,
    #[serde(rename = "positiveExamples")]
    pub positive_examples: Vec<TextEnvelope>,
    #[serde(rename = "negativeExamples")]
    pub negative_examples: Vec<TextEnvelope>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateCategoryFilterResponse {
    #[serde(rename = "categoryName")]
    pub category_name: String,
    pub positions: Vec<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageResponse {
    pub language: Option<String>,
    pub iso_tag: Option<String>,
    pub wiki_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompareResponse {
    #[serde(rename = "sizeLeft")]
    pub size_left: u32,
    #[serde(rename = "sizeRight")]
    pub size_right: u32,
    #[serde(rename = "weightedScoring")]
    pub weighted_scoring: f64,
    #[serde(rename = "euclideanDistance")]
    pub euclidean_distance: f64,
    #[serde(rename = "jaccardDistance")]
    pub jaccard_distance: f64,
    #[serde(rename = "overlappingAll")]
    pub overlapping_all: u32,
    #[serde(rename = "overlappingLeftRight")]
    pub overlapping_left_right: f64,
    #[serde(rename = "overlappingRightLeft")]
    pub overlapping_right_left: f64,
    #[serde(rename = "cosineSimilarity")]
    pub cosine_similarity: f64,
}