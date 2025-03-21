#!/bin/bash

# ===== AI-Friendly Tauri Runner =====
# 
# This script is designed specifically for AI assistants to safely
# run Tauri commands without getting stuck or encountering common issues.
#
# Usage: 
#   ./tools/ai-run-tauri.sh [dev|build] [--no-frontend]

set -e

COMMAND="dev"
FRONTEND=true

# Parse arguments
if [ "$1" == "build" ]; then
  COMMAND="build"
elif [ "$1" == "dev" ]; then
  COMMAND="dev"
fi

if [ "$2" == "--no-frontend" ]; then
  FRONTEND=false
fi

echo "====== JanusLens AI-Friendly Tauri Runner ======"
echo "Mode: $COMMAND"
echo "Include frontend: $FRONTEND"

# Check for port usage and clean up
if lsof -i:1420 >/dev/null 2>&1; then
  echo "Port 1420 is in use. Attempting to kill processes..."
  lsof -ti:1420 | xargs -r kill -9
  sleep 1
fi

# Create log directory if it doesn't exist
mkdir -p logs

# Start the frontend if needed
if [ "$FRONTEND" = true ] && [ "$COMMAND" = "dev" ]; then
  echo "Starting frontend in background..."
  npm run dev > logs/frontend-$(date +%Y%m%d-%H%M%S).log 2>&1 &
  FRONTEND_PID=$!
  echo "Frontend started with PID: $FRONTEND_PID"
  echo "To view frontend logs: tail -f logs/frontend-*.log"
  
  # Give the frontend a moment to start
  echo "Waiting for frontend to initialize..."
  sleep 5
fi

# Add binary specification to avoid selection prompts
echo "Starting Tauri $COMMAND..."
if [ "$COMMAND" = "dev" ]; then
  # For dev mode
  LOG_FILE="logs/tauri-dev-$(date +%Y%m%d-%H%M%S).log"
  cd src-tauri && cargo run --bin januslens > "../$LOG_FILE" 2>&1 &
  TAURI_PID=$!
  echo "Tauri started with PID: $TAURI_PID"
  echo "To view logs: tail -f $LOG_FILE"
  
  # Set timeout to auto-terminate after 30 minutes
  (
    sleep 1800
    if ps -p $TAURI_PID > /dev/null; then
      echo "Tauri has been running for 30 minutes. Terminating..."
      kill -9 $TAURI_PID
      
      # Also kill frontend if it was started
      if [ "$FRONTEND" = true ] && ps -p $FRONTEND_PID > /dev/null; then
        kill -9 $FRONTEND_PID
      fi
    fi
  ) &
  
  echo "Tauri will auto-terminate after 30 minutes."
  echo "To terminate manually: kill -9 $TAURI_PID"
else
  # For build mode
  LOG_FILE="logs/tauri-build-$(date +%Y%m%d-%H%M%S).log"
  npm run tauri build > "$LOG_FILE" 2>&1 &
  BUILD_PID=$!
  echo "Build started with PID: $BUILD_PID"
  echo "To view logs: tail -f $LOG_FILE"
  
  # Set timeout to auto-terminate after 60 minutes
  (
    sleep 3600
    if ps -p $BUILD_PID > /dev/null; then
      echo "Build has been running for 60 minutes. Terminating..."
      kill -9 $BUILD_PID
    fi
  ) &
  
  echo "Build will auto-terminate after 60 minutes."
  echo "To terminate manually: kill -9 $BUILD_PID"
fi

echo "Command started. AI-friendly Tauri runner completed." 