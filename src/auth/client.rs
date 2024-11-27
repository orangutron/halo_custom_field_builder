use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::Config;
use crate::error::{Result, CustomError, AuthErrorKind};
use super::token::AuthToken;

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
    error_description: String,
}

#[derive(Debug, Serialize)]
struct TokenRequest {
    client_id: String,
    client_secret: String,
    grant_type: String,
}

pub struct AuthClient {
    config: Config,
    http_client: ReqwestClient,
    current_token: Arc<Mutex<Option<AuthToken>>>,
}

impl AuthClient {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            http_client: ReqwestClient::new(),
            current_token: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn get_valid_token(&self) -> Result<String> {
        let mut token_guard = self.current_token.lock().await;
        
        // Check if we have a valid token
        if let Some(token) = token_guard.as_ref() {
            if !token.is_expired() {
                return Ok(token.header_value());
            }
        }

        // If we get here, we need a new token
        let new_token = self.fetch_new_token().await?;
        *token_guard = Some(new_token);
        
        Ok(token_guard.as_ref().unwrap().header_value())
    }

    async fn fetch_new_token(&self) -> Result<AuthToken> {
        let token_request = TokenRequest {
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.clone(),
            grant_type: "client_credentials".to_string(),
        };

        let response = self.http_client
            .post(&self.config.token_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&token_request)
            .send()
            .await
            .map_err(|e| {
                if e.is_status() {
                    if let Some(status) = e.status() {
                        if status == reqwest::StatusCode::UNAUTHORIZED {
                            return CustomError::AuthError(AuthErrorKind::InvalidCredentials);
                        }
                    }
                }
                CustomError::AuthError(AuthErrorKind::TokenFetchFailed(e.to_string()))
            })?;

        // Get the status before consuming the response
        let status = response.status();
        println!("Debug - Response received: {}", if status.is_success() { "Success" } else { "Error" });

        let response_text = response.text().await.map_err(|e| 
            CustomError::AuthError(AuthErrorKind::TokenFetchFailed(format!("Failed to get response text: {}", e)))
        )?;

        // First try to parse as an error response
        if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
            return Err(CustomError::AuthError(AuthErrorKind::TokenFetchFailed(
                format!("{}: {}", error_response.error, error_response.error_description)
            )));
        }

        // If it's not an error, try to parse as a successful response
        let token_response: TokenResponse = serde_json::from_str(&response_text).map_err(|e| {
            CustomError::AuthError(AuthErrorKind::InvalidTokenResponse(format!(
                "Failed to parse response: {}", e
            )))
        })?;

        Ok(AuthToken::new(
            token_response.access_token,
            token_response.token_type,
            token_response.expires_in,
        ))
    }
}
