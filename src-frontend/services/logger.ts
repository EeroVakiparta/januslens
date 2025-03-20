/**
 * JanusLens Logger Service
 * 
 * Implements the logging strategy defined in docs/LOGGING_STRATEGY.md
 * Used for comprehensive application logging and AI testing support
 */

import { invoke } from '@tauri-apps/api/core';

// Log level enum
export enum LogLevel {
  ERROR = 'ERROR',
  WARN = 'WARN',
  INFO = 'INFO',
  DEBUG = 'DEBUG',
  TRACE = 'TRACE',
  VERIFY = 'VERIFY'
}

// Log entry structure
export interface LogEntry {
  timestamp: string;
  level: LogLevel;
  component: string;
  message: string;
  details?: any;
  contextId?: string;
}

// In-memory log storage
const memoryLogs: LogEntry[] = [];
const MAX_MEMORY_LOGS = 1000; // Maximum number of logs to keep in memory

/**
 * Main Logger class for JanusLens
 * Provides logging functionality with consistent formatting and storage
 */
export class Logger {
  private static contextId: string | null = null;
  private static enabled = true;
  private static logLevel = LogLevel.INFO;
  
  /**
   * Initialize the logger
   * @param options Logger initialization options
   */
  static initialize(options?: {
    enabled?: boolean;
    logLevel?: LogLevel;
  }) {
    if (options?.enabled !== undefined) {
      this.enabled = options.enabled;
    }
    
    if (options?.logLevel) {
      this.logLevel = options.logLevel;
    }
    
    this.info('Logger', 'Logger initialized', { level: this.logLevel, enabled: this.enabled });
  }
  
  /**
   * Generate a unique context ID for tracking related operations
   */
  static startContext(): string {
    this.contextId = Math.random().toString(36).substring(2, 8);
    return this.contextId;
  }
  
  /**
   * Clear the current context ID
   */
  static endContext() {
    this.contextId = null;
  }
  
  /**
   * Create a timestamp in ISO format
   */
  private static getTimestamp(): string {
    return new Date().toISOString();
  }
  
  /**
   * Should this log level be recorded based on the current setting
   */
  private static shouldLog(level: LogLevel): boolean {
    if (!this.enabled) return false;
    
    const levels = [LogLevel.ERROR, LogLevel.WARN, LogLevel.INFO, LogLevel.DEBUG, LogLevel.TRACE];
    const currentIndex = levels.indexOf(this.logLevel);
    const messageIndex = levels.indexOf(level);
    
    // VERIFY level is always logged
    if (level === LogLevel.VERIFY) return true;
    
    return messageIndex <= currentIndex;
  }
  
  /**
   * Create and store a log entry
   */
  static log(level: LogLevel, component: string, message: string, details?: any): void {
    if (!this.shouldLog(level)) return;
    
    const entry: LogEntry = {
      timestamp: this.getTimestamp(),
      level,
      component,
      message,
      details: details || null,
      contextId: this.contextId || undefined
    };
    
    // Store in memory buffer (circular buffer)
    memoryLogs.push(entry);
    if (memoryLogs.length > MAX_MEMORY_LOGS) {
      memoryLogs.shift();
    }
    
    // Format for console output
    const detailsStr = details ? ` | ${typeof details === 'object' ? JSON.stringify(details) : details}` : '';
    const contextStr = this.contextId ? ` [Context:${this.contextId}]` : '';
    const formattedMessage = `[${entry.timestamp}] [${level}] [${component}]${contextStr} ${message}${detailsStr}`;
    
    // Output to console with appropriate styling
    switch (level) {
      case LogLevel.ERROR:
        console.error(formattedMessage);
        break;
      case LogLevel.WARN:
        console.warn(formattedMessage);
        break;
      case LogLevel.INFO:
        console.info(formattedMessage);
        break;
      case LogLevel.DEBUG:
      case LogLevel.TRACE:
        console.debug(formattedMessage);
        break;
      case LogLevel.VERIFY:
        console.log(`%c${formattedMessage}`, 'color: purple; font-weight: bold');
        break;
    }
    
    // Send to backend for potential storage
    this.sendToBackend(entry).catch(e => {
      console.error('Failed to send log to backend:', e);
    });
  }
  
  /**
   * Send log entry to backend
   */
  private static async sendToBackend(entry: LogEntry): Promise<void> {
    try {
      await invoke('log_event', { entry });
    } catch (error) {
      // Silently fail - we don't want logging failures to cause application issues
    }
  }
  
  /**
   * Log methods for different levels
   */
  static error(component: string, message: string, details?: any): void {
    this.log(LogLevel.ERROR, component, message, details);
  }
  
  static warn(component: string, message: string, details?: any): void {
    this.log(LogLevel.WARN, component, message, details);
  }
  
  static info(component: string, message: string, details?: any): void {
    this.log(LogLevel.INFO, component, message, details);
  }
  
  static debug(component: string, message: string, details?: any): void {
    this.log(LogLevel.DEBUG, component, message, details);
  }
  
  static trace(component: string, message: string, details?: any): void {
    this.log(LogLevel.TRACE, component, message, details);
  }
  
  /**
   * Special verification log for AI testing
   */
  static verify(component: string, operation: string, expected: any, actual: any): void {
    const success = expected === actual || 
      (typeof expected === 'object' && typeof actual === 'object' && 
       JSON.stringify(expected) === JSON.stringify(actual));
    
    this.log(
      LogLevel.VERIFY,
      component,
      `${operation} | Expected: ${JSON.stringify(expected)} | Actual: ${JSON.stringify(actual)} | Success: ${success}`,
      { expected, actual, success }
    );
  }
  
  /**
   * Get recent logs from memory
   */
  static getLogs(filter?: {
    level?: LogLevel;
    component?: string;
    contextId?: string;
    limit?: number;
  }): LogEntry[] {
    let filtered = [...memoryLogs];
    
    if (filter?.level) {
      filtered = filtered.filter(log => log.level === filter.level);
    }
    
    if (filter?.component) {
      filtered = filtered.filter(log => log.component === filter.component);
    }
    
    if (filter?.contextId) {
      filtered = filtered.filter(log => log.contextId === filter.contextId);
    }
    
    if (filter?.limit && filter.limit > 0) {
      filtered = filtered.slice(-filter.limit);
    }
    
    return filtered;
  }
  
  /**
   * Capture a state snapshot for before/after comparisons
   */
  static captureState(phase: 'Before' | 'After', operation: string, state: any): void {
    this.debug(`StateSnapshot:${phase}`, operation, state);
  }
  
  /**
   * Export logs to file (to be implemented with Tauri API)
   */
  static async exportLogs(): Promise<string> {
    try {
      const filename = `januslens_logs_${new Date().toISOString().replace(/:/g, '-')}.json`;
      const result = await invoke('export_logs', { filename, logs: memoryLogs });
      return result as string;
    } catch (error) {
      this.error('Logger', 'Failed to export logs', error);
      throw error;
    }
  }
  
  /**
   * Clear all logs from memory
   */
  static clearLogs(): void {
    memoryLogs.length = 0;
    this.info('Logger', 'Logs cleared from memory');
  }
  
  /**
   * Set the current log level
   */
  static setLogLevel(level: LogLevel): void {
    this.logLevel = level;
    this.info('Logger', 'Log level changed', { level });
  }
  
  /**
   * Enable or disable logging
   */
  static setEnabled(enabled: boolean): void {
    this.enabled = enabled;
    this.info('Logger', 'Logger enabled state changed', { enabled });
  }
} 