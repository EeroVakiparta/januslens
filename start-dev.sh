#!/bin/bash

# Log header
echo "====== JanusLens Development Server Starter ======"
echo "Starting script at $(date)"

# Function to check if a port is in use
check_port() {
  lsof -i:"$1" >/dev/null 2>&1
  return $?
}

# Function to kill process on a specific port
kill_process_on_port() {
  echo "Attempting to kill process on port $1..."
  pids=$(lsof -ti:"$1")
  if [ -n "$pids" ]; then
    # Safely handle multiple PIDs by using xargs
    echo "Killing processes on port $1: $pids"
    lsof -ti:"$1" | xargs -r kill -9
    sleep 1
    if check_port "$1"; then
      echo "Failed to kill process on port $1"
      return 1
    else
      echo "Successfully killed process on port $1"
      return 0
    fi
  else
    echo "No process found on port $1"
    return 0
  fi
}

# Set port
PORT=1420

# Parse command line options
BACKGROUND=false
while getopts "bp:" opt; do
  case $opt in
    b) BACKGROUND=true ;;
    p) PORT=$OPTARG ;;
    *) echo "Unknown option: -$OPTARG"; exit 1 ;;
  esac
done

# Check if port is in use and kill if necessary
if check_port "$PORT"; then
  echo "Port $PORT is in use."
  kill_process_on_port "$PORT"
else
  echo "Port $PORT is available."
fi

# Start the development server
echo "Starting development server..."
if [ "$BACKGROUND" = true ]; then
  echo "Running in background mode. Output will be redirected to dev-server.log"
  npm run dev > dev-server.log 2>&1 &
  PID=$!
  echo "Server started in background with PID: $PID"
  echo "To view logs: tail -f dev-server.log"
  echo "To stop the server later: kill -9 $PID or lsof -ti:$PORT | xargs kill -9"
else
  # Add a timeout to prevent getting stuck indefinitely (macOS compatible)
  npm run dev &
  DEV_PID=$!
  
  # Kill after 5 minutes if still running
  (
    sleep 300
    if ps -p $DEV_PID > /dev/null; then
      echo "Development server has been running for 5 minutes. Terminating..."
      kill -9 $DEV_PID
    fi
  ) &
  
  # Wait for the dev process to complete
  wait $DEV_PID
fi 