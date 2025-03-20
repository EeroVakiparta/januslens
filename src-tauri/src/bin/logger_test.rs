use januslens::logging::{LogLevel, LogEntry, log_event, get_recent_logs};
use januslens::error::JanusError;
use std::time::Duration;
use std::thread;
use chrono::Utc;
use colored::Colorize;

fn main() -> Result<(), JanusError> {
    println!("{}", "JanusLens Logger Test".bold().green());
    println!("{}", "===================".green());
    
    // Test basic logging
    println!("\n{}", "Testing basic logging...".cyan());
    
    let test_entry = LogEntry {
        timestamp: Utc::now().to_rfc3339(),
        level: LogLevel::Info,
        component: "LoggerTest".to_string(),
        message: "Test log message".to_string(),
        details: Some(serde_json::json!({
            "test_key": "test_value",
            "number": 42
        })),
        context_id: None,
    };
    
    log_event(test_entry)?;
    println!("{}", "✅ Basic logging successful".green());
    
    // Test different log levels
    println!("\n{}", "Testing different log levels...".cyan());
    
    for level in &[LogLevel::Error, LogLevel::Warn, LogLevel::Info, LogLevel::Debug, LogLevel::Trace] {
        let entry = LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            level: level.clone(),
            component: "LogLevelTest".to_string(),
            message: format!("Test {} level", format!("{:?}", level).to_uppercase()),
            details: None,
            context_id: None,
        };
        
        log_event(entry)?;
        thread::sleep(Duration::from_millis(100)); // Small delay for readability
    }
    
    println!("{}", "✅ Log level testing successful".green());
    
    // Test context tracking
    println!("\n{}", "Testing context tracking...".cyan());
    
    let context_id = "test-context-123";
    
    for i in 1..=3 {
        let entry = LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            level: LogLevel::Info,
            component: "ContextTest".to_string(),
            message: format!("Context test message {}", i),
            details: None,
            context_id: Some(context_id.to_string()),
        };
        
        log_event(entry)?;
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("{}", "✅ Context tracking successful".green());
    
    // Test retrieving logs
    println!("\n{}", "Testing log retrieval...".cyan());
    
    let all_logs = get_recent_logs(None, None, None, None)?;
    println!("Total logs in memory: {}", all_logs.len());
    
    let context_logs = get_recent_logs(None, None, Some(context_id.to_string()), None)?;
    println!("Logs with test context: {}", context_logs.len());
    assert_eq!(context_logs.len(), 3, "Should have 3 logs with the test context");
    
    let component_logs = get_recent_logs(None, Some("LogLevelTest".to_string()), None, None)?;
    println!("Logs from LogLevelTest component: {}", component_logs.len());
    assert_eq!(component_logs.len(), 5, "Should have 5 logs from the LogLevelTest component");
    
    println!("{}", "✅ Log retrieval successful".green());
    
    // Test verification logging
    println!("\n{}", "Testing verification logging...".cyan());
    
    let verify_entry = LogEntry {
        timestamp: Utc::now().to_rfc3339(),
        level: LogLevel::Verify,
        component: "VerifyTest".to_string(),
        message: format!("Operation | Expected: {} | Actual: {} | Success: {}", 
                         true, true, true),
        details: Some(serde_json::json!({
            "expected": true,
            "actual": true,
            "success": true
        })),
        context_id: None,
    };
    
    log_event(verify_entry)?;
    println!("{}", "✅ Verification logging successful".green());
    
    println!("\n{}", "All logger tests completed successfully!".bold().green());
    Ok(())
} 