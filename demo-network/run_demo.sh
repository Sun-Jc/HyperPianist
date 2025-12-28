#!/usr/bin/env bash

# Exit on error
set -e

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
WORKSPACE_ROOT="$(dirname "$SCRIPT_DIR")"
DATA_FILE="$SCRIPT_DIR/data/ip_list.txt"

# Default to demo-low (restored)
BIN_NAME=${1:-"demo-low"}

# Build quietly
echo "Compiling $BIN_NAME..."
RUSTFLAGS="-Awarnings" cargo build -p demo-network --bin "$BIN_NAME" --quiet 2>/dev/null

BIN_PATH="$WORKSPACE_ROOT/target/debug/$BIN_NAME"

PIDS=()

# Function to kill all child processes on exit
cleanup() {
    echo "Stopping all nodes..."
    for pid in "${PIDS[@]}"; do
        if kill -0 "$pid" 2>/dev/null; then
            kill "$pid" 2>/dev/null || true
        fi
    done
}
trap cleanup EXIT INT TERM

echo "Starting 4 nodes running $BIN_NAME..."

# Launch Workers (1, 2, 3) first
$BIN_PATH 1 "$DATA_FILE" &
PIDS+=($!)
$BIN_PATH 2 "$DATA_FILE" &
PIDS+=($!)
$BIN_PATH 3 "$DATA_FILE" &
PIDS+=($!)

# Give them a split second to spin up
sleep 1

# Launch Master (0)
$BIN_PATH 0 "$DATA_FILE" &
PIDS+=($!)

# Wait for all background processes to finish
wait

echo "Demo completed successfully."
