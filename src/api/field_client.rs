use reqwest::Client as ReqwestClient;
use crate::models::field::Field;
use crate::error::{Result, CustomError, ApiErrorKind};
use crate::config::Config;
use crate::transformers::json_transformer::JsonTransformer;
use log::debug;

pub struct FieldClient {
    config: Config,
    http_client: ReqwestClient,
}

impl FieldClient {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            http_client: ReqwestClient::new(),
        }
    }

    pub async fn create_field(&self, field: &Field, auth_token: &str) -> Result<()> {
        let field_json = JsonTransformer::transform_fields(&[field.clone()]);
        let url = format!("{}/fieldinfo", self.config.api_url);

        debug!("Sending request to: {}", url);
        debug!("Request payload: {}", serde_json::to_string_pretty(&field_json)?);

        let response = self.http_client
            .post(&url)
            .header("Authorization", auth_token)
            .header("Content-Type", "application/json")
            .json(&field_json)  // Send the entire array, not just the first element
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await
                .unwrap_or_else(|_| "Failed to get error response".to_string());
            
            return Err(CustomError::ApiError(ApiErrorKind::FieldCreationFailed(
                field.label.clone(),
                format!("Status: {}, Error: {}", status, error_text)
            )));
        }

        Ok(())
    }
}