use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Retina {
    #[serde(rename = "retinaName")]
    pub retina_name: String,
    #[serde(rename = "numberOfColumns")]
    pub number_of_columns: i64,
    #[serde(rename = "numberOfTermsInRetina")]
    pub number_of_terms_in_retina: i64,
    pub description: String,
    #[serde(rename = "numberOfRows")]
    pub number_of_rows: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fingerprint {
    pub positions: Vec<i32>,
}

impl Fingerprint {
    pub fn expand(&self, len: usize) -> Vec<u8> {
        let mut expanded = vec![0; len];

        for pos in &self.positions {
            expanded[*pos as usize] = 1;
        }

        expanded
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
    pub category_name: Option<String>,
    pub positive_examples: Vec<TextEnvelope>,
    pub negative_examples: Vec<TextEnvelope>,
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
    pub size_left: i64,
    #[serde(rename = "sizeRight")]
    pub size_right: i64,
    #[serde(rename = "weightedScoring")]
    pub weighted_scoring: f64,
    #[serde(rename = "euclideanDistance")]
    pub euclidean_distance: f64,
    #[serde(rename = "jaccardDistance")]
    pub jaccard_distance: f64,
    #[serde(rename = "overlappingAll")]
    pub overlapping_all: i64,
    #[serde(rename = "overlappingLeftRight")]
    pub overlapping_left_right: f64,
    #[serde(rename = "overlappingRightLeft")]
    pub overlapping_right_left: f64,
    #[serde(rename = "cosineSimilarity")]
    pub cosine_similarity: f64,
}

pub struct Cortical {
    pub client: reqwest::Client,
    pub base_url: String,
}

impl Cortical {
    pub fn new() -> Cortical {
        Cortical {
            client: reqwest::Client::new(),
            base_url: std::env::var("CORTICAL_API_URL").unwrap_or("https://languages.cortical.io".to_string()),
        }
    }

    pub async fn get_retinas(&self) -> Result<Vec<Retina>, Box<dyn Error>> {
        let response =
            self.client
                .get(format!("{}{}", &self.base_url, "/rest/retinas"))
                .send()
                .await?;

        Ok(
            response
                .json()
                .await?
        )
    }

    pub async fn get_text_analysis(
        &self,
        text: &str,
        retina_name: Option<&str>,
    ) -> Result<Vec<Fingerprint>, Box<dyn Error>> {
        let retina_name = retina_name.unwrap_or("en_general");

        let response =
            self.client
                .post(format!("{}/rest/text?retina_name={}", &self.base_url, retina_name))
                .header("Accept", "application/json")
                .header("Referer", "")
                .header("Content-Type", "application/json")
                .body(text.to_string())
                .send()
                .await?;

        Ok(
            response
                .json()
                .await?
        )
    }

    pub async fn get_text_keywords(
        &self,
        text: &str,
        retina_name: Option<&str>,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let retina_name = retina_name.unwrap_or("en_general");

        let response =
            self.client
                .post(format!("{}/rest/text/keywords?retina_name={}", &self.base_url, retina_name))
                .header("Accept", "application/json")
                .header("Referer", "")
                .header("Content-Type", "text/plain;charset=UTF-8")
                .body(text.to_string())
                .send()
                .await?;

        Ok(
            response
                .json()
                .await?
        )
    }

    pub async fn get_text_slices(
        &self,
        text: &str,
        params: Option<TextSliceRequest>,
    ) -> Result<Vec<TextSlice>, Box<dyn Error>> {
        let params = params.unwrap_or_default();

        let response =
            self.client
                .post(
                    format!(
                        "{}/rest/text/slices?retina_name={}&start_index={}&max_results={}&get_fingerprint={}",
                        &self.base_url,
                        params.retina_name,
                        params.start_index,
                        params.max_results,
                        params.get_fingerprint
                    )
                )
                .header("Accept", "application/json")
                .header("Referer", "")
                .header("Content-Type", "application/json")
                .body(text.to_string())
                .send()
                .await?;

        Ok(
            response
                .json()
                .await?
        )
    }

    pub async fn get_text_detect_language(
        &self,
        text: &str,
    ) -> Result<LanguageResponse, Box<dyn Error>> {
        let response =
            self.client
                .post(format!("{}/rest/text/detect_language", &self.base_url))
                .header("Accept", "application/json")
                .header("Referer", "")
                .header("Content-Type", "application/json")
                .body(text.to_string())
                .send()
                .await?;

        Ok(
            response
                .json()
                .await?
        )
    }

    pub async fn create_category_filter(
        &self,
        positive_examples: Vec<String>,
        negative_examples: Vec<String>,
        retina_name: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        let retina_name = retina_name.unwrap_or("en_general");

        let positive_examples = positive_examples
            .into_iter()
            .map(|text| TextEnvelope { text })
            .collect();

        let negative_examples = negative_examples
            .into_iter()
            .map(|text| TextEnvelope { text })
            .collect();

        let request = CreateCategoryFilterRequest {
            category_name: None,
            positive_examples,
            negative_examples,
        };

        Ok(
            self.client
                .post(
                    format!(
                        "{}/rest/classify/create_category_filter?retina_name={}&filter_name={}",
                        &self.base_url,
                        retina_name,
                        "filter_name"
                    )
                )
                .header("Accept", "application/json")
                .header("Referer", "")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&request)?)
                .send()
                .await?
                .json()
                .await?
        )
    }

    pub async fn get_compare(
        &self,
        (text1, text2): (&str, &str),
        retina_name: Option<&str>,
    ) -> Result<CompareResponse, Box<dyn Error>> {
        let retina_name = retina_name.unwrap_or("en_general");

        Ok(
            self.client
                .post(format!("{}/rest/compare?retina_name={}", &self.base_url, retina_name))
                .header("Accept", "application/json")
                .header("Referer", "")
                .header("Content-Type", "application/json")
                .body(
                    serde_json::to_string(
                        &vec![
                            TextEnvelope::new(text1),
                            TextEnvelope::new(text2),
                        ]
                    )?
                )
                .send()
                .await?
                .json()
                .await?
        )
    }
}