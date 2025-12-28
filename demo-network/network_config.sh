#!/bin/bash

# Configuration file for network simulation inside the container.
# This script is executed by docker-entrypoint.sh before starting the demo.

INTERFACE="lo"

# 1. Clear existing rules
tc qdisc del dev $INTERFACE root 2>/dev/null || true

# 2. Apply new rules
# You can modify these lines to simulate different environments:
# Example Loss: tc qdisc add dev $INTERFACE root netem loss 5%
# Example Latency: tc qdisc add dev $INTERFACE root netem delay 100ms
# Example Both: tc qdisc add dev $INTERFACE root netem delay 50ms loss 1%

# LOSS_PERCENT=${NET_LOSS:-"0%"}
LOSS_PERCENT="25%"

if [ "$LOSS_PERCENT" == "0%" ]; then
    echo "[Network Config] Clearing all rules from $INTERFACE..."
    tc qdisc del dev $INTERFACE root 2>/dev/null || true
else
    echo "[Network Config] Running: tc qdisc replace dev $INTERFACE root netem loss $LOSS_PERCENT"
    tc qdisc replace dev $INTERFACE root netem loss $LOSS_PERCENT
fi
