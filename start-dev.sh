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
  pid=$(lsof -ti:"$1")
  if [ -n "$pid" ]; then
    echo "Killing process $pid on port $1"
    kill -9 "$pid"
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
  echo "Server started in background with PID: $!"
  echo "To view logs: tail -f dev-server.log"
  echo "To stop the server later: lsof -ti:$PORT | xargs kill -9"
else
  npm run dev
fi 