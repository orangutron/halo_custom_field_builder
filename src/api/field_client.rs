use tokio::time::sleep;
use std::time::Duration;
use reqwest::Client as ReqwestClient;
use crate::models::field::Field;
use crate::error::{Result, CustomError, ApiErrorKind};
use crate::config::Config;
use crate::transformers::json_transformer::JsonTransformer;
use log::debug;

pub struct FieldClient {
    config: Config,
    http_client: ReqwestClient,
    auth_token: String,
}

impl FieldClient {
    pub fn new(config: Config, auth_token: String) -> Self {
        Self {
            config,
            http_client: ReqwestClient::new(),
            auth_token,
        }
    }

    pub async fn create_field(&self, field: &Field) -> Result<()> {
        // Rate limiting: 500ms delay between requests
        // Max 120 requests/minute, staying under the 700/5min limit
        sleep(Duration::from_millis(500)).await;

        let endpoint = format!("{}/fieldinfo", self.config.api_url);
        let json = JsonTransformer::to_json(&[field.clone()])?;
        debug!("Sending field creation request for: {}", field.label);
        
        let response = self.http_client
            .post(&endpoint)
            .header("Authorization", &self.auth_token)
            .header("Content-Type", "application/json")
            .body(json)
            .send()
            .await
            .map_err(|e| CustomError::ApiError(ApiErrorKind::NetworkError(e.to_string())))?;

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