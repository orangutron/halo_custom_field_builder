use dotenv::dotenv;
use std::env;
use url::Url;
use crate::error::{Result, CustomError, ConfigErrorKind};

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub tenant: String,
    pub token_url: String,
    pub api_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub source_file_name: String,
}

impl Config {
    fn get_env_var(key: &str, allow_empty: bool) -> Result<String> {
        let value = env::var(key)
            .map_err(|_| CustomError::ConfigError(ConfigErrorKind::MissingEnvVar(key.to_string())))?;
        
        if !allow_empty && value.trim().is_empty() {
            return Err(CustomError::ConfigError(ConfigErrorKind::EmptyEnvVar(key.to_string())));
        }
        
        Ok(value.trim().to_string())
    }

    fn validate_url(url: &str, _field_name: &str) -> Result<String> {
        // Ensure URL starts with https:// and is valid
        let url_actual = Url::parse(&url).map_err(|_| 
            CustomError::ConfigError(ConfigErrorKind::InvalidUrlFormat(url.to_string()))
        )?;
        if url_actual.scheme() != "https" {
            return Err(CustomError::ConfigError(ConfigErrorKind::InvalidUrlFormat(url.to_string())));
        }

        // Remove trailing slash if present
        Ok(url.trim_end_matches('/').to_string())
    }

    fn build_token_url(base_url: &str, tenant: &str) -> String {
        if tenant.trim().is_empty() {
            format!("{}/auth/token", base_url)
        } else {
            format!("{}/auth/token?tenant={}", base_url, tenant)
        }
    }

    pub fn new() -> Result<Self> {
        // Load .env file
        dotenv().map_err(|_| 
            CustomError::ConfigError(ConfigErrorKind::MissingEnvFile)
        )?;

        // Get and validate required variables
        let raw_base_url = Self::get_env_var("BASE_URL", false)?;
        let base_url = Self::validate_url(&raw_base_url, "BASE_URL")?;
        
        // Tenant can be empty
        let tenant = Self::get_env_var("TENANT", true)?;
        
        // Get and validate other required variables
        let client_id = Self::get_env_var("CLIENT_ID", false)?;
        let client_secret = Self::get_env_var("CLIENT_SECRET", false)?;
        let source_file_name = Self::get_env_var("SOURCE_FILE_NAME", false)?;

        // Build and validate derived URLs
        let api_url = format!("{}/api", &base_url);
        let token_url = Self::build_token_url(&base_url, &tenant);

        // Return the completed struct
        Ok(Config {
            base_url,
            tenant,
            api_url,
            token_url,
            client_id,
            client_secret,
            source_file_name,
        })
    }
}