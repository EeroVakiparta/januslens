#!/bin/bash

# ===== AI-Friendly Git Operations =====
# 
# This script provides a safe way for AI assistants to perform Git operations
# without getting stuck on interactive prompts or pagers.
#
# Usage: 
#   ./tools/ai-git.sh COMMAND [args]
#
# Examples:
#   ./tools/ai-git.sh status
#   ./tools/ai-git.sh log
#   ./tools/ai-git.sh diff file.txt
#   ./tools/ai-git.sh commit -m "Message"

set -e

# Check if command is provided
if [ -z "$1" ]; then
  echo "Error: No command specified"
  echo "Usage: $0 COMMAND [args]"
  exit 1
fi

# Extract git command and arguments
GIT_CMD="$1"
shift
GIT_ARGS="$@"

# Create log directory if it doesn't exist
mkdir -p logs

# Log file for this operation
LOG_FILE="logs/git-$(date +%Y%m%d-%H%M%S).log"

echo "====== JanusLens AI-Friendly Git Runner ======"
echo "Command: git $GIT_CMD $GIT_ARGS"
echo "Logging to: $LOG_FILE"

# Define safer versions of common git commands
case "$GIT_CMD" in
  "log")
    # Always pipe log to cat to prevent pager
    GIT_TERMINAL_PROMPT=0 git log $GIT_ARGS | cat | tee $LOG_FILE
    ;;
    
  "diff")
    # Always pipe diff to cat to prevent pager
    GIT_TERMINAL_PROMPT=0 git diff $GIT_ARGS | cat | tee $LOG_FILE
    ;;
    
  "show")
    # Always pipe show to cat to prevent pager
    GIT_TERMINAL_PROMPT=0 git show $GIT_ARGS | cat | tee $LOG_FILE
    ;;
    
  "blame")
    # Always pipe blame to cat to prevent pager
    GIT_TERMINAL_PROMPT=0 git blame $GIT_ARGS | cat | tee $LOG_FILE
    ;;
    
  "pull" | "fetch" | "push")
    # Add timeout for network operations
    echo "Running with 30-second timeout..."
    (
      GIT_TERMINAL_PROMPT=0 git $GIT_CMD $GIT_ARGS
    ) &
    PID=$!
    
    # Set a timeout
    (
      sleep 30
      if ps -p $PID > /dev/null; then
        echo "Git operation timed out after 30 seconds. Terminating..."
        kill -9 $PID
      fi
    ) &
    
    # Wait for completion
    wait $PID || echo "Git operation failed or was terminated"
    ;;
    
  *)
    # For all other commands, just disable terminal prompts
    GIT_TERMINAL_PROMPT=0 git $GIT_CMD $GIT_ARGS | tee $LOG_FILE
    ;;
esac

echo "Git operation completed." 