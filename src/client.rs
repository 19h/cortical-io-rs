use std::error::Error;

use crate::{CompareResponse, CreateCategoryFilterRequest, Fingerprint, LanguageResponse, Retina, TextEnvelope, TextSlice, TextSliceRequest};

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