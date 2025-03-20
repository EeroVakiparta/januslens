use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::error::JanusError;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "WARN")]
    Warn,
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "DEBUG")]
    Debug,
    #[serde(rename = "TRACE")]
    Trace,
    #[serde(rename = "VERIFY")]
    Verify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub component: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

lazy_static! {
    static ref LOGS: Mutex<Vec<LogEntry>> = Mutex::new(Vec::new());
}

// Maximum number of logs to keep in memory
const MAX_MEMORY_LOGS: usize = 1000;

/// Add a log entry to both memory and file storage
pub fn log_event(entry: LogEntry) -> Result<(), JanusError> {
    // Add to in-memory logs with limit
    {
        let mut logs = LOGS.lock().map_err(|e| JanusError::UnknownError(format!("Failed to lock logs: {}", e)))?;
        logs.push(entry.clone());
        
        // Keep memory logs under limit
        if logs.len() > MAX_MEMORY_LOGS {
            *logs = logs.iter().skip(logs.len() - MAX_MEMORY_LOGS).cloned().collect();
        }
    }
    
    // Log to file
    append_to_log_file(&entry)?;
    
    // Also log to standard output
    let level_str = format!("{:?}", entry.level).to_uppercase();
    let details_str = match entry.details {
        Some(ref details) => format!(" | {}", details),
        None => String::new(),
    };
    let context_str = match entry.context_id {
        Some(ref ctx) => format!(" [Context:{}]", ctx),
        None => String::new(),
    };
    
    println!(
        "[{}] [{}] [{}]{} {}{}",
        entry.timestamp, level_str, entry.component, context_str, entry.message, details_str
    );
    
    Ok(())
}

/// Append a log entry to the log file
fn append_to_log_file(entry: &LogEntry) -> Result<(), JanusError> {
    let log_dir = get_log_dir()?;
    fs::create_dir_all(&log_dir).map_err(|e| JanusError::IoError(format!("Failed to create log directory: {}", e)))?;
    
    // Create a new log file for each day
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let log_file_path = log_dir.join(format!("januslens_{}.log", today));
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .map_err(|e| JanusError::IoError(format!("Failed to open log file: {}", e)))?;
    
    // Format the entry as a JSON line
    let entry_json = serde_json::to_string(entry)
        .map_err(|e| JanusError::ParseError(format!("Failed to serialize log entry: {}", e)))?;
    
    writeln!(file, "{}", entry_json)
        .map_err(|e| JanusError::IoError(format!("Failed to write to log file: {}", e)))?;
    
    Ok(())
}

/// Get the logs directory
fn get_log_dir() -> Result<PathBuf, JanusError> {
    let app_data_dir = dirs::data_dir()
        .ok_or_else(|| JanusError::ConfigError("Could not determine data directory".to_string()))?;
    
    Ok(app_data_dir.join("januslens").join("logs"))
}

/// Export logs to a file
#[tauri::command]
pub fn export_logs(filename: String, logs: Vec<LogEntry>) -> Result<String, JanusError> {
    let export_dir = dirs::document_dir()
        .ok_or_else(|| JanusError::ConfigError("Could not determine documents directory".to_string()))?
        .join("JanusLens");
    
    fs::create_dir_all(&export_dir)
        .map_err(|e| JanusError::IoError(format!("Failed to create export directory: {}", e)))?;
    
    let file_path = export_dir.join(filename);
    let mut file = File::create(&file_path)
        .map_err(|e| JanusError::IoError(format!("Failed to create export file: {}", e)))?;
    
    let logs_json = serde_json::to_string_pretty(&logs)
        .map_err(|e| JanusError::ParseError(format!("Failed to serialize logs: {}", e)))?;
    
    file.write_all(logs_json.as_bytes())
        .map_err(|e| JanusError::IoError(format!("Failed to write to export file: {}", e)))?;
    
    Ok(file_path.to_string_lossy().to_string())
}

/// Log an event from the frontend
#[tauri::command]
pub fn log_event_from_frontend(entry: LogEntry) -> Result<(), JanusError> {
    log_event(entry)
}

/// Get recent logs from memory
#[tauri::command]
pub fn get_recent_logs(
    level: Option<String>,
    component: Option<String>,
    context_id: Option<String>,
    limit: Option<usize>
) -> Result<Vec<LogEntry>, JanusError> {
    let logs = LOGS.lock()
        .map_err(|e| JanusError::UnknownError(format!("Failed to lock logs: {}", e)))?;
    
    let mut filtered_logs: Vec<LogEntry> = logs.clone();
    
    // Apply filters
    if let Some(level_str) = level {
        filtered_logs.retain(|log| {
            let log_level = format!("{:?}", log.level).to_uppercase();
            log_level == level_str.to_uppercase()
        });
    }
    
    if let Some(comp) = component {
        filtered_logs.retain(|log| log.component == comp);
    }
    
    if let Some(ctx) = context_id {
        filtered_logs.retain(|log| log.context_id.as_ref().map_or(false, |id| id == &ctx));
    }
    
    // Apply limit
    if let Some(lim) = limit {
        if filtered_logs.len() > lim {
            filtered_logs = filtered_logs.iter().skip(filtered_logs.len() - lim).cloned().collect();
        }
    }
    
    Ok(filtered_logs)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_entry_serialization() {
        let entry = LogEntry {
            timestamp: "2023-01-01T12:00:00Z".to_string(),
            level: LogLevel::Info,
            component: "TestComponent".to_string(),
            message: "Test message".to_string(),
            details: Some(serde_json::json!({ "key": "value" })),
            context_id: Some("test-context".to_string()),
        };
        
        let serialized = serde_json::to_string(&entry).unwrap();
        let deserialized: LogEntry = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(entry.timestamp, deserialized.timestamp);
        assert_eq!(format!("{:?}", entry.level), format!("{:?}", deserialized.level));
        assert_eq!(entry.component, deserialized.component);
        assert_eq!(entry.message, deserialized.message);
    }
} 