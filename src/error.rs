use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct FieldError {
    pub row: usize,
    pub error: FieldErrorKind,
}

#[derive(Debug)]
pub enum FieldErrorKind {
    ParseError(String),
    RequiredFieldEmpty(String),
    InvalidFieldName(String),
    InvalidLabel(String),
    InvalidTypeId(String),
    InvalidInputType(String),
    MissingOptions(String),
}

#[derive(Debug)]
pub enum ConfigErrorKind {
    MissingEnvFile,
    MissingEnvVar(String),
    #[allow(dead_code)]
    InvalidUrl(String),
    EmptyEnvVar(String),
    InvalidUrlFormat(String),
    JsonError(String),
}

#[derive(Debug)]
pub enum AuthErrorKind {
    TokenFetchFailed(String),
    InvalidTokenResponse(String),
    #[allow(dead_code)]
    TokenExpired,
    #[allow(dead_code)]
    Unauthorized(String),
    InvalidCredentials,
    NetworkError(String),
}

#[derive(Debug)]
pub enum IOErrorKind {
    CreateDir(String),
    ReadDir(String),
    ReadFile(String),
    WriteFile(String),
    Metadata(String),
}

#[derive(Debug)]
pub enum ApiErrorKind {
    FieldCreationFailed(String, String),  // (field_label, error_message)
    #[allow(dead_code)]
    InvalidResponse(String),
    #[allow(dead_code)]
    NetworkError(String),
}

#[derive(Debug)]
pub enum CustomError {
    CsvError(csv::Error),
    MissingColumn(String),
    FieldError(FieldError),
    ConfigError(ConfigErrorKind),
    AuthError(AuthErrorKind),
    IOError(IOErrorKind),
    ApiError(ApiErrorKind),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::CsvError(e) => write!(f, "CSV error: {}", e),
            CustomError::MissingColumn(col) => write!(
                f, 
                "Required column '{}' is missing from the CSV file. Please check your column headers", 
                col
            ),
            CustomError::FieldError(error) => write!(
                f, 
                "Error in row {}: {}", 
                error.row + 2,  // +2 because: +1 for header row, +1 because rows start at 0
                match &error.error {
                    FieldErrorKind::ParseError(field) => format!(
                        "Failed to parse '{}'. Please ensure the value is in the correct format", 
                        field
                    ),
                    FieldErrorKind::RequiredFieldEmpty(field) => format!(
                        "The '{}' field cannot be empty. Please provide a value", 
                        field
                    ),
                    FieldErrorKind::InvalidFieldName(name) => format!(
                        "Field name '{}' is invalid. Names must contain only letters and numbers", 
                        name
                    ),
                    FieldErrorKind::InvalidLabel(label) => format!(
                        "Invalid label: {}. Labels must contain visible characters", 
                        label
                    ),
                    FieldErrorKind::InvalidTypeId(type_id) => format!(
                        "Invalid type_id: {}.\n\nValid values are:\n0 (Text)\n1 (Memo)\n2 (Single Selection\n3 (Multiple Selection)\n4 (Date)\n5 (Time)\n6 (Checkbox)\n10 (Rich)", 
                        type_id
                    ),
                    FieldErrorKind::InvalidInputType(msg) => msg.clone(),
                    FieldErrorKind::MissingOptions(msg) => format!(
                        "{}. Please provide a comma-separated list of options", 
                        msg
                    ),
                }
            ),
            CustomError::ConfigError(kind) => match kind {
                ConfigErrorKind::MissingEnvFile => 
                    write!(f, "Failed to load .env file. Please ensure it exists in the project root"),
                ConfigErrorKind::MissingEnvVar(var) => 
                    write!(f, "Required environment variable '{}' is must present", var),
                ConfigErrorKind::InvalidUrl(url) => 
                    write!(f, "Invalid URL format: {}", url),
                ConfigErrorKind::EmptyEnvVar(field) =>
                    write!(f, "Configuration value for '{}' must have value", field),
                ConfigErrorKind::InvalidUrlFormat(url) =>
                    write!(f, "Invalid URL format for '{}'. URL must be a valid HTTPS URL", url),
                ConfigErrorKind::JsonError(msg) => 
                    write!(f, "JSON serialization error: {}", msg),
            },
            CustomError::AuthError(kind) => match kind {
                AuthErrorKind::TokenFetchFailed(msg) => 
                    write!(f, "Failed to fetch authentication token: {}", msg),
                AuthErrorKind::InvalidTokenResponse(msg) => 
                    write!(f, "Invalid token response from server: {}", msg),
                AuthErrorKind::TokenExpired => 
                    write!(f, "Authentication token has expired"),
                AuthErrorKind::Unauthorized(msg) => 
                    write!(f, "Unauthorized: {}", msg),
                AuthErrorKind::InvalidCredentials => 
                    write!(f, "Invalid client credentials"),
                AuthErrorKind::NetworkError(msg) => 
                    write!(f, "Network error during authentication: {}", msg),
            },
            CustomError::IOError(kind) => match kind {
                IOErrorKind::CreateDir(msg) => write!(f, "Failed to create directory: {}", msg),
                IOErrorKind::ReadDir(msg) => write!(f, "Failed to read directory: {}", msg),
                IOErrorKind::ReadFile(msg) => write!(f, "Failed to read file: {}", msg),
                IOErrorKind::WriteFile(msg) => write!(f, "Failed to write file: {}", msg),
                IOErrorKind::Metadata(msg) => write!(f, "Failed to get metadata: {}", msg),
            },
            CustomError::ApiError(kind) => match kind {
                ApiErrorKind::FieldCreationFailed(label, error) => 
                    write!(f, "Failed to create field '{}': {}", label, error),
                ApiErrorKind::InvalidResponse(msg) => 
                    write!(f, "Invalid API response: {}", msg),
                ApiErrorKind::NetworkError(msg) => 
                    write!(f, "Network error: {}", msg),
            },
        }
    }
}

impl Error for CustomError {}

// This allows automatic conversion from csv::Error to our CustomError
impl From<csv::Error> for CustomError {
    fn from(error: csv::Error) -> Self {
        CustomError::CsvError(error)
    }
}

// Create a type alias for Result
pub type Result<T> = std::result::Result<T, CustomError>;

// Add From implementation for serde_json::Error
impl From<serde_json::Error> for CustomError {
    fn from(error: serde_json::Error) -> Self {
        CustomError::ConfigError(ConfigErrorKind::JsonError(error.to_string()))
    }
}

// Add conversion from reqwest::Error to CustomError
impl From<reqwest::Error> for CustomError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_status() {
            if let Some(status) = error.status() {
                if status == reqwest::StatusCode::UNAUTHORIZED {
                    return CustomError::AuthError(AuthErrorKind::InvalidCredentials);
                }
            }
        }
        
        if error.is_timeout() {
            return CustomError::AuthError(
                AuthErrorKind::NetworkError("Request timed out".to_string())
            );
        }
        
        CustomError::AuthError(AuthErrorKind::NetworkError(error.to_string()))
    }
}

// Add implementation for converting std::io::Error to CustomError
impl From<std::io::Error> for CustomError {
    fn from(error: std::io::Error) -> Self {
        CustomError::IOError(IOErrorKind::ReadFile(error.to_string()))
    }
}

impl fmt::Display for ApiErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiErrorKind::FieldCreationFailed(label, error) => 
                write!(f, "Failed to create field '{}': {}", label, error),
            ApiErrorKind::InvalidResponse(msg) => 
                write!(f, "Invalid API response: {}", msg),
            ApiErrorKind::NetworkError(msg) => 
                write!(f, "Network error: {}", msg),
        }
    }
}