#!/bin/bash
set -e

# Navigate to deNetwork directory if script is run from repo root
if [ -d "deNetwork" ]; then
    cd deNetwork
fi

echo "Building example..."
cargo build --example demo_net

# Create hosts file
echo "127.0.0.1:9000" > hosts.txt
echo "127.0.0.1:9001" >> hosts.txt
echo "127.0.0.1:9002" >> hosts.txt

echo "Running demo with 3 parties..."
export RUST_LOG=info

# Run parties in background
# We need to ensure they start roughly at the same time
# Note: cargo build --example puts binary in target/debug/examples/
../target/debug/examples/demo_net --id 0 --n 3 &
PID0=$!
../target/debug/examples/demo_net --id 1 --n 3 &
PID1=$!
../target/debug/examples/demo_net --id 2 --n 3 &
PID2=$!

wait $PID0 $PID1 $PID2
echo "Demo completed."

rm hosts.txt
