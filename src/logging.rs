use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use log::{LevelFilter, info, error};
use simplelog::*;
use chrono::{Local, Duration, DateTime};
use crate::error::{Result, CustomError, IOErrorKind};
use crate::models::import_result::ImportResults;

const MAX_LOG_DAYS: i64 = 7;
const MAX_LOGS: usize = 100;

pub fn setup_logging() -> Result<()> {
    // Create logs directory if it doesn't exist
    let logs_dir = Path::new("logs");
    fs::create_dir_all(logs_dir).map_err(|e| 
        CustomError::IOError(IOErrorKind::CreateDir(e.to_string()))
    )?;

    // Cleanup old logs
    cleanup_old_logs(logs_dir)?;

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let log_file = logs_dir.join(format!("run_{}.log", timestamp));
    
    // Configure file logger with timestamps
    let file_config = ConfigBuilder::new()
        .set_target_level(LevelFilter::Off)
        .set_location_level(LevelFilter::Off)
        .set_thread_level(LevelFilter::Off)
        .set_time_offset_to_local()
        .unwrap_or_else(|builder| builder)
        .build();

    // Configure terminal logger without timestamps
    let term_config = ConfigBuilder::new()
        .set_target_level(LevelFilter::Off)
        .set_location_level(LevelFilter::Off)
        .set_thread_level(LevelFilter::Off)
        .set_time_level(LevelFilter::Off)
        .build();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            term_config,
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            file_config,
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&log_file)
                .map_err(|e| CustomError::IOError(IOErrorKind::WriteFile(e.to_string())))?,
        ),
    ]).map_err(|e| CustomError::IOError(IOErrorKind::WriteFile(e.to_string())))?;

    log::info!("{}", "=".repeat(80));
    log::info!("Log session started at {}", timestamp);
    log::info!("{}\n", "=".repeat(80));

    Ok(())
}

fn cleanup_old_logs(logs_dir: &Path) -> Result<()> {
    let mut log_files: Vec<_> = fs::read_dir(logs_dir)
        .map_err(|e| CustomError::IOError(IOErrorKind::ReadDir(e.to_string())))?
        .filter_map(|r| r.ok())
        .filter(|entry| {
            entry.path()
                .extension()
                .map_or(false, |ext| ext == "log")
        })
        .collect();

    // Sort by modified time, newest first
    log_files.sort_by(|a, b| {
        let a_time = a.metadata().and_then(|m| m.modified()).ok();
        let b_time = b.metadata().and_then(|m| m.modified()).ok();
        b_time.cmp(&a_time)
    });

    let cutoff_date = Local::now() - Duration::days(MAX_LOG_DAYS);

    // Remove old files
    for entry in log_files.iter().skip(MAX_LOGS) {
        if let Ok(metadata) = entry.metadata() {
            if let Ok(modified) = metadata.modified() {
                let modified: DateTime<Local> = modified.into();
                if modified < cutoff_date {
                    let _ = fs::remove_file(entry.path());
                }
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub fn get_log_stats() -> Result<String> {
    let logs_dir = Path::new("logs");
    let mut total_size = 0;
    let mut file_count = 0;
    let mut oldest_date: Option<DateTime<Local>> = None;
    let mut newest_date: Option<DateTime<Local>> = None;

    if logs_dir.exists() {
        for entry in fs::read_dir(logs_dir)
            .map_err(|e| CustomError::IOError(IOErrorKind::ReadDir(e.to_string())))? {
            let entry = entry.map_err(|e| CustomError::IOError(IOErrorKind::ReadFile(e.to_string())))?;
            
            if entry.path().extension().map_or(false, |ext| ext == "log") {
                file_count += 1;
                let metadata = entry.metadata()
                    .map_err(|e| CustomError::IOError(IOErrorKind::Metadata(e.to_string())))?;
                total_size += metadata.len();
                
                if let Ok(modified) = metadata.modified() {
                    let modified: DateTime<Local> = modified.into();
                    oldest_date = Some(oldest_date.map_or(modified, |d| d.min(modified)));
                    newest_date = Some(newest_date.map_or(modified, |d| d.max(modified)));
                }
            }
        }
    }

    Ok(format!(
        "\nLog Statistics:\n\
         - Total log files: {}\n\
         - Total size: {}\n\
         - Oldest log: {}\n\
         - Newest log: {}\n\
         - Retention policy: {} days",
        file_count,
        format_bytes(total_size),
        oldest_date.map_or("None".to_string(), |d| d.format("%Y-%m-%d %H:%M:%S").to_string()),
        newest_date.map_or("None".to_string(), |d| d.format("%Y-%m-%d %H:%M:%S").to_string()),
        MAX_LOG_DAYS
    ))
}

#[allow(dead_code)]
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

#[allow(dead_code)]
pub fn log_import_result(result: &ImportResults) -> Result<()> {
    info!("\n{}", "=".repeat(80));
    info!("Import Results Summary");
    info!("{}", "=".repeat(80));
    
    for field in &result.successful {
        info!("✓ Successfully imported: {}", field.label);
    }
    
    for field in &result.failed {
        error!("✗ Failed to import: {} ({})", 
            field.label, 
            field.error.as_ref().unwrap_or(&"Unknown error".to_string())
        );
    }
    
    info!("\n{}", "=".repeat(80));
    Ok(())
}