#!/bin/bash

# ===== Safe Command Runner for AI Agents =====
# 
# This script provides a safe way to run commands that might get stuck,
# especially when executed by AI agents like Claude in Cursor.
#
# Usage: 
#   ./tools/safe-run.sh "command to run" [timeout in seconds] [log file]
#
# Examples:
#   ./tools/safe-run.sh "npm run dev" 60
#   ./tools/safe-run.sh "git pull" 30 git-pull.log
#   ./tools/safe-run.sh "npm run tauri dev" 120 tauri-dev.log

set -e

# Default values
TIMEOUT=60
LOG_FILE="/dev/null"
COMMAND=$1

# Parse arguments
if [ -z "$COMMAND" ]; then
  echo "Error: No command specified"
  echo "Usage: $0 \"command to run\" [timeout in seconds] [log file]"
  exit 1
fi

if [ ! -z "$2" ]; then
  TIMEOUT=$2
fi

if [ ! -z "$3" ]; then
  LOG_FILE=$3
fi

echo "====== JanusLens Safe Command Runner ======"
echo "Running command: $COMMAND"
echo "Timeout: ${TIMEOUT}s"
if [ "$LOG_FILE" != "/dev/null" ]; then
  echo "Logging to: $LOG_FILE"
fi

# Check if the command contains npm run dev or similar
if [[ "$COMMAND" == *"npm run dev"* || "$COMMAND" == *"vite"* ]]; then
  # Check if port 1420 is in use and kill it
  if lsof -i:1420 >/dev/null 2>&1; then
    echo "Port 1420 is in use. Attempting to kill processes..."
    lsof -ti:1420 | xargs -r kill -9
    sleep 1
  fi
fi

# Run the command with a macOS/Linux compatible timeout approach
echo "Starting command with ${TIMEOUT}s timeout at $(date)"

# Create a function to kill the process after the timeout
function kill_after_timeout() {
  local pid=$1
  local timeout=$2
  (
    sleep $timeout
    if ps -p $pid > /dev/null; then
      echo "Timeout reached after ${timeout}s. Killing process..."
      kill -9 $pid
    fi
  ) &
}

if [ "$LOG_FILE" != "/dev/null" ]; then
  # With logging
  bash -c "$COMMAND" > $LOG_FILE 2>&1 &
  PID=$!
  echo "Process started with PID: $PID, output logging to $LOG_FILE"
  echo "To view logs: tail -f $LOG_FILE"
else
  # Without logging (direct output)
  bash -c "$COMMAND" &
  PID=$!
  echo "Process started with PID: $PID"
fi

# Set up the timeout
kill_after_timeout $PID $TIMEOUT

echo "To terminate manually: kill -9 $PID"
echo "Process will automatically terminate after ${TIMEOUT}s if not completed"
echo "Command started. Safe-run script completed." 