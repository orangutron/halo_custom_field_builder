mod config;
mod models;
mod readers;
mod error;
mod transformers;

use config::Config;
use readers::csv_reader::CsvReader;
use transformers::json_transformer::JsonTransformer;
use error::{Result, CustomError};

#[cfg(windows)]
fn clear_console() {
    std::process::Command::new("cmd")
        .args(["/c", "cls"])
        .status()
        .unwrap();
}

#[cfg(not(windows))]
fn clear_console() {
    std::process::Command::new("clear")
        .status()
        .unwrap();
}

fn run() -> Result<()> {
    clear_console();
    
    let config = Config::new()?;
    let reader = CsvReader::new();
    
    let fields = reader.read_fields(&config)?;
    println!("Successfully validated {} fields", fields.len());
    
    // Transform to JSON silently
    JsonTransformer::to_json(&fields)?;
    println!("JSON transformation complete");
    
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        match &e {
            CustomError::CsvError(csv_err) => eprintln!("CSV error details: {}", csv_err),
            CustomError::MissingColumn(col) => eprintln!(
                "Required column '{}' is missing from the CSV file. Please check your column headers", 
                col
            ),
            CustomError::FieldError(_) => (),
            CustomError::ConfigError(_) => eprintln!("Please check your configuration settings in .env file"),
        }
        std::process::exit(1);
    }
}