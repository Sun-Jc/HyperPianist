FROM rustlang/rust:nightly-bullseye

# Install iproute2 for 'tc' and other network utilities
RUN apt-get update && apt-get install -y iproute2 bash procps && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/hyperpianist

# Copy the entire workspace
COPY . .

# Pre-build the demo-network package
RUN cargo build -p demo-network

# Ensure scripts are executable
RUN chmod +x demo-network/run_demo.sh docker-entrypoint.sh

# The container needs NET_ADMIN capability to run 'tc'
ENTRYPOINT ["./docker-entrypoint.sh"]
