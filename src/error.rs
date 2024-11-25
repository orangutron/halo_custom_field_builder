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
    InvalidUrl(String),
    EmptyEnvVar(String),
    InvalidUrlFormat(String),
    JsonError(String),
}

#[derive(Debug)]
pub enum CustomError {
    CsvError(csv::Error),
    MissingColumn(String),
    FieldError(FieldError),
    ConfigError(ConfigErrorKind),
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
                        "Invalid type_id: {}. Valid values are: 0 (Text), 1 (Memo), 2 (Single Selection), \
                        3 (Multiple Selection), 4 (Date), 5 (Time), 6 (Checkbox), 10 (Rich)", 
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