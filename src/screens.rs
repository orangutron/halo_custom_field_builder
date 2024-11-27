use std::io::{self, Write};
use crate::models::field::Field;
use crate::error::Result;
use log::{info, error};
use colored::*;
use crate::api::field_client::FieldClient;
use crate::models::import_result::ImportResults;

pub enum RunMode {
    Import,
    Debug,
    Quit,
}

pub enum DebugAction {
    Process,
    Skip,
    Quit,
}

pub struct ScreenManager {
    fields: Vec<Field>,
}

impl ScreenManager {
    pub fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }

    pub fn show_initial_stats(&self, token_type: &str) -> Result<()> {
        println!("\n{}", "Initial Status:".bright_blue().bold());
        println!("{}",   "=".repeat(80).bright_blue());
        
        println!("• Authentication: {} (Token Type: {})", 
            "Success".bright_green().bold(), 
            token_type.bright_yellow()
        );
        
        println!("• Fields loaded: {}", 
            self.fields.len().to_string().bright_yellow()
        );
        
        println!("• Status: {}", 
            "Ready to process".bright_green().bold()
        );
        
        println!("{}\n", "=".repeat(80).bright_blue());
        Ok(())
    }

    pub fn get_run_mode(&self) -> Result<RunMode> {
        println!("{}", "\nAvailable Operations:".bright_blue().bold());
        println!("{}", "=".repeat(80).bright_blue());
        
        println!("{}. {}", 
            "1".bright_yellow().bold(), 
            "Import all fields".bright_green()
        );
        
        println!("{}. {}", 
            "2".bright_yellow().bold(), 
            "Debug mode (field by field)".bright_cyan()
        );
        
        println!("{}. {}", 
            "3".bright_yellow().bold(), 
            "Quit program".bright_red()
        );
        
        print!("\n{}", "Enter your choice (1-3): ".bright_white().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => {
                println!("\n{}", "Selected: Import all fields".bright_green());
                Ok(RunMode::Import)
            },
            "2" => {
                println!("\n{}", "Selected: Debug mode".bright_cyan());
                Ok(RunMode::Debug)
            },
            "3" => {
                println!("\n{}", "Selected: Quit program".bright_red());
                Ok(RunMode::Quit)
            },
            _ => {
                error!("{}", "Invalid selection. Please try again.".bright_red());
                self.get_run_mode()
            }
        }
    }

    pub async fn process_all_fields(
        &self, 
        field_client: &FieldClient, 
        auth_token: &str
    ) -> Result<ImportResults> {
        let mut results = ImportResults::new();
        
        for field in &self.fields {
            print!("Processing field: {}", field.label.bright_yellow());
            io::stdout().flush()?;
            
            match field_client.create_field(field, auth_token).await {
                Ok(_) => {
                    results.add_success(field.label.clone());
                    println!(" {}", "✓".bright_green());
                },
                Err(e) => {
                    results.add_failure(field.label.clone(), e.to_string());
                    println!(" {}", "✗".bright_red());
                    println!("  {}: {}", "Error".bright_red(), e);
                }
            }
        }

        Ok(results)
    }

    pub async fn debug_mode(
        &self, 
        field_client: &FieldClient, 
        auth_token: &str
    ) -> Result<ImportResults> {
        info!("\nEntering Debug Mode");
        info!("This mode will process fields one at a time\n");

        let mut results = ImportResults::new();

        for (index, field) in self.fields.iter().enumerate() {
            match self.show_field_debug_prompt(index, field)? {
                DebugAction::Process => {
                    info!("Processing field: {}", field.label);
                    
                    match field_client.create_field(field, auth_token).await {
                        Ok(_) => {
                            results.add_success(field.label.clone());
                            info!("✓ Field processed successfully\n");
                        },
                        Err(e) => {
                            results.add_failure(field.label.clone(), e.to_string());
                            error!("✗ Field processing failed: {}\n", e);
                        }
                    }
                },
                DebugAction::Skip => {
                    info!("Skipping field: {}\n", field.label);
                    continue;
                },
                DebugAction::Quit => {
                    info!("Debug mode terminated by user");
                    break;
                }
            }
        }

        Ok(results)
    }

    fn show_field_debug_prompt(&self, index: usize, field: &Field) -> Result<DebugAction> {
        println!("\n{}", "=".repeat(80).bright_blue());
        println!("{}", format!("Field {} of {}", 
            (index + 1).to_string().bright_yellow(),
            self.fields.len().to_string().bright_yellow()
        ).bright_blue().bold());
        println!("{}", "=".repeat(80).bright_blue());
        
        println!("\n{}", "Field Details:".bright_blue().bold());
        println!("• Label: {}", field.label.bright_yellow());
        println!("• Name: {}", field.name.bright_yellow());
        println!("• Type: {}", field.type_id.to_string().bright_yellow());
        println!("• Input Type: {}", field.input_type_id.to_string().bright_yellow());
        
        if !field.options.is_empty() {
            println!("• Options: {}", field.options.bright_yellow());
        }

        println!("\n{}", "Available actions:".bright_blue().bold());
        println!("{}. {} field", 
            "1".bright_yellow().bold(), 
            "Process".bright_green()
        );
        println!("{}. {} field", 
            "2".bright_yellow().bold(), 
            "Skip".bright_cyan()
        );
        println!("{}. {} debug mode", 
            "3".bright_yellow().bold(), 
            "Quit".bright_red()
        );
        
        print!("\n{}", "Enter your choice (1-3): ".bright_white().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => Ok(DebugAction::Process),
            "2" => Ok(DebugAction::Skip),
            "3" => Ok(DebugAction::Quit),
            _ => {
                error!("{}", "Invalid selection. Please try again.".bright_red());
                self.show_field_debug_prompt(index, field)
            }
        }
    }
}