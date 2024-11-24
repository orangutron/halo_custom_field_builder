mod config;
mod models;
mod readers;
mod error;

use config::Config;
use readers::csv_reader::CsvReader;
use error::{Result, CustomError};

fn run() -> Result<()> {
    let config = Config::new()?;
    let reader = CsvReader::new();
    
    let fields = reader.read_fields(&config)?;
    
    println!("Successfully loaded {} fields", fields.len());
    println!("Fields: {:#?}", fields);
    
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