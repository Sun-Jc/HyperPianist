#!/usr/bin/env bash

# This script reuses a persistent container named 'hyperpianist-running-demo'
# Ensures permissions are correct before running.

NAME="hyperpianist-running-demo"

# Ensure entrypoint is executable on the host so it mounts with correct bits
chmod +x docker-entrypoint.sh demo-network/run_demo.sh demo-network/network_config.sh

# Check if the container already exists (even if stopped)
if [ "$(docker ps -aq -f name=^/${NAME}$)" ]; then
    echo "--- Reusing existing container: $NAME ---"
    docker start -ai "$NAME"
else
    echo "--- Creating new persistent container: $NAME ---"
    docker run -it \
      --name "$NAME" \
      --cap-add=NET_ADMIN \
      -e NET_LOSS=${NET_LOSS:-"0%"} \
      -v "$(pwd)":/usr/src/hyperpianist \
      -v hyperpianist_container_target:/usr/src/hyperpianist/target \
      hyperpianist-demo
fi
