#!/bin/bash
set -e

echo "--- Environment Setup ---"

# Load and apply network configuration from the script file
CONFIG_SCRIPT="./demo-network/network_config.sh"

if [ -f "$CONFIG_SCRIPT" ]; then
    chmod +x "$CONFIG_SCRIPT"
    echo "Loading network configuration from $CONFIG_SCRIPT..."
    $CONFIG_SCRIPT
else
    echo "Warning: $CONFIG_SCRIPT not found, skipping network setup."
fi

echo "--- Starting Demo ---"
# Execute the existing run script
exec ./demo-network/run_demo.sh