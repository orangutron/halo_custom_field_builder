mod config;
mod models;
mod readers;
mod error;
mod auth;
mod logging;
mod screens;
mod api;
mod transformers;

use config::Config;
use readers::csv_reader::CsvReader;
use error::{Result, CustomError, AuthErrorKind};
use auth::client::AuthClient;
use log::{info, error};
use screens::{ScreenManager, RunMode};
use api::field_client::FieldClient;

async fn run() -> Result<()> {
    logging::setup_logging()?;

    info!("Starting application...\n");
    
    info!("Loading configuration...");
    let config = Config::new()?;
    info!("✓ Configuration loaded successfully\n");
    
    info!("Authenticating with API...");
    let auth_client = AuthClient::new(config.clone());
    
    let token = match auth_client.get_valid_token().await {
        Ok(token) => {
            info!("✓ Authentication successful");
            info!("✓ Token acquired and valid\n");
            token
        },
        Err(CustomError::AuthError(AuthErrorKind::InvalidCredentials)) => {
            error!("✗ Authentication failed: Invalid credentials");
            error!("Please check your client credentials in the .env file");
            std::process::exit(1);
        },
        Err(e) => {
            error!("✗ Authentication failed");
            return Err(e);
        }
    };
    
    info!("Reading CSV file...");
    let reader = CsvReader::new();
    let fields = reader.read_fields(&config)?;
    info!("✓ Successfully validated {} fields\n", fields.len());
    
    let screen_manager = ScreenManager::new(fields);
    screen_manager.show_initial_stats(&token.split_whitespace().next().unwrap_or("Unknown"))?;

    let field_client = FieldClient::new(config.clone(), token);
    
    match screen_manager.get_run_mode()? {
        RunMode::Import => {
            info!("\n{}", "=".repeat(80));
            info!("Starting Full Import Mode");
            info!("{}\n", "=".repeat(80));
            
            let results = screen_manager.process_all_fields(&field_client).await?;
            results.log_summary();
        },
        RunMode::Debug => {
            info!("\n{}", "=".repeat(80));
            info!("Starting Debug Mode");
            info!("{}\n", "=".repeat(80));
            
            let results = screen_manager.debug_mode(&field_client).await?;
            results.log_summary();
        },
        RunMode::Quit => {
            info!("Program terminated by user");
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = run().await {
        error!("\n✗ Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}